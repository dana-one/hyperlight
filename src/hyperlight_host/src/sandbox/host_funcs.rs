use std::io::{stdout, Write};

use hyperlight_common::flatbuffer_wrappers::function_types::{ParameterValue, ReturnValue};
use hyperlight_common::flatbuffer_wrappers::host_function_definition::HostFunctionDefinition;
use hyperlight_common::flatbuffer_wrappers::host_function_details::HostFunctionDetails;
use is_terminal::IsTerminal;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use tracing::{instrument, Span};

use super::{ExtraAllowedSyscall, FunctionsMap};
use crate::func::HyperlightFunction;
use crate::mem::mgr::SandboxMemoryManager;
#[cfg(all(feature = "seccomp", target_os = "linux"))]
use crate::signal_handlers::mark_as_hyperlight_thread;
#[cfg(all(feature = "seccomp", target_os = "linux"))]
use crate::signal_handlers::sigsys_signal_handler::{self, IOCTL_PARAM, SYSCALL_NUMBER};
use crate::HyperlightError::HostFunctionNotFound;
use crate::{new_error, Result};

#[derive(Default, Clone)]
/// A Wrapper around details of functions exposed by the Host
pub struct HostFuncsWrapper {
    functions_map: FunctionsMap,
    function_details: HostFunctionDetails,
}

impl HostFuncsWrapper {
    #[instrument(skip_all, parent = Span::current(), level = "Trace")]
    fn get_host_funcs(&self) -> &FunctionsMap {
        &self.functions_map
    }
    #[instrument(skip_all, parent = Span::current(), level = "Trace")]
    fn get_host_funcs_mut(&mut self) -> &mut FunctionsMap {
        &mut self.functions_map
    }
    #[instrument(skip_all, parent = Span::current(), level = "Trace")]
    fn get_host_func_details(&self) -> &HostFunctionDetails {
        &self.function_details
    }
    #[instrument(skip_all, parent = Span::current(), level = "Trace")]
    fn get_host_func_details_mut(&mut self) -> &mut HostFunctionDetails {
        &mut self.function_details
    }

    /// Register a host function with the sandbox.
    #[instrument(err(Debug), skip_all, parent = Span::current(), level = "Trace")]
    pub(crate) fn register_host_function(
        &mut self,
        mgr: &mut SandboxMemoryManager,
        hfd: &HostFunctionDefinition,
        func: HyperlightFunction,
    ) -> Result<()> {
        register_host_function_helper(self, mgr, hfd, func, None)
    }

    /// Register a host function with the sandbox, with a list of extra syscalls
    /// that the function is allowed to make.
    #[instrument(err(Debug), skip_all, parent = Span::current(), level = "Trace")]
    #[cfg(all(feature = "seccomp", target_os = "linux"))]
    pub(crate) fn register_host_function_with_syscalls(
        &mut self,
        mgr: &mut SandboxMemoryManager,
        hfd: &HostFunctionDefinition,
        func: HyperlightFunction,
        extra_allowed_syscalls: Vec<ExtraAllowedSyscall>,
    ) -> Result<()> {
        register_host_function_helper(self, mgr, hfd, func, Some(extra_allowed_syscalls))
    }

    /// Assuming a host function called `"HostPrint"` exists, and takes a
    /// single string parameter, call it with the given `msg` parameter.
    ///
    /// Return `Ok` if the function was found and was of the right signature,
    /// and `Err` otherwise.
    #[instrument(err(Debug), skip_all, parent = Span::current(), level = "Trace")]
    pub(super) fn host_print(&mut self, msg: String) -> Result<i32> {
        let res = call_host_func_impl(
            self.get_host_funcs(),
            "HostPrint",
            vec![ParameterValue::String(msg)],
        )?;
        res.try_into()
            .map_err(|_| HostFunctionNotFound("HostPrint".to_string()))
    }
    /// From the set of registered host functions, attempt to get the one
    /// named `name`. If it exists, call it with the given arguments list
    /// `args` and return its result.
    ///
    /// Return `Err` if no such function exists,
    /// its parameter list doesn't match `args`, or there was another error
    /// getting, configuring or calling the function.
    #[instrument(err(Debug), skip_all, parent = Span::current(), level = "Trace")]
    pub(super) fn call_host_function(
        &self,
        name: &str,
        args: Vec<ParameterValue>,
    ) -> Result<ReturnValue> {
        call_host_func_impl(self.get_host_funcs(), name, args)
    }
}

fn register_host_function_helper(
    self_: &mut HostFuncsWrapper,
    mgr: &mut SandboxMemoryManager,
    hfd: &HostFunctionDefinition,
    func: HyperlightFunction,
    extra_allowed_syscalls: Option<Vec<ExtraAllowedSyscall>>,
) -> Result<()> {
    if let Some(_syscalls) = extra_allowed_syscalls {
        #[cfg(all(feature = "seccomp", target_os = "linux"))]
        self_
            .get_host_funcs_mut()
            .insert(hfd.function_name.to_string(), func, Some(_syscalls));

        #[cfg(not(all(feature = "seccomp", target_os = "linux")))]
        return Err(new_error!(
            "Extra syscalls are only supported on Linux with seccomp"
        ));
    } else {
        self_
            .get_host_funcs_mut()
            .insert(hfd.function_name.to_string(), func, None);
    }
    self_
        .get_host_func_details_mut()
        .insert_host_function(hfd.clone());
    // Functions need to be sorted so that they are serialised in sorted order
    // this is required in order for flatbuffers C implementation used in the Gues Library
    // to be able to search the functions by name.
    self_
        .get_host_func_details_mut()
        .sort_host_functions_by_name();
    let buffer: Vec<u8> = self_.get_host_func_details().try_into().map_err(|e| {
        new_error!(
            "Error serializing host function details to flatbuffer: {}",
            e
        )
    })?;
    mgr.write_buffer_host_function_details(&buffer)?;

    Ok(())
}

#[instrument(err(Debug), skip_all, parent = Span::current(), level = "Trace")]
fn call_host_func_impl(
    host_funcs: &FunctionsMap,
    name: &str,
    args: Vec<ParameterValue>,
) -> Result<ReturnValue> {
    // Inner function containing the common logic
    fn call_func(
        host_funcs: &FunctionsMap,
        name: &str,
        args: Vec<ParameterValue>,
    ) -> Result<ReturnValue> {
        let func_with_syscalls = host_funcs
            .get(name)
            .ok_or_else(|| HostFunctionNotFound(name.to_string()))?;

        let func = func_with_syscalls.0.clone();

        #[cfg(all(feature = "seccomp", target_os = "linux"))]
        {
            let syscalls = func_with_syscalls.1.clone();
            let seccomp_filter =
                crate::seccomp::guest::get_seccomp_filter_for_host_function_worker_thread(
                    syscalls,
                )?;
            seccompiler::apply_filter(&seccomp_filter)?;
        }

        #[cfg(feature = "function_call_metrics")]
        {
            let start = std::time::Instant::now();
            let result = func.call(args.clone());
            crate::histogram_vec_observe!(
                &crate::sandbox::metrics::SandboxMetric::HostFunctionCallsDurationMicroseconds,
                &[name],
                start.elapsed().as_micros() as f64
            );
            result
        }

        #[cfg(not(feature = "function_call_metrics"))]
        func.call(args)
    }

    cfg_if::cfg_if! {
        if #[cfg(all(feature = "seccomp", target_os = "linux"))] {
            // Register the signal handler once
            sigsys_signal_handler::register_signal_handler_once()?;

            // Clone variables for the thread
            let host_funcs_cloned = host_funcs.clone();
            let name_cloned = name.to_string();
            let args_cloned = args.clone();

            // Create a new thread when seccomp is enabled on Linux
            let join_handle = std::thread::Builder::new()
                .name(format!("Host Function Worker Thread for: {:?}", name_cloned))
                .spawn(move || {
                    // Mark this thread as a hyperlight thread
                    mark_as_hyperlight_thread();

                    // Clear thread-local storage at the start
                    SYSCALL_NUMBER.with(|syscall_num| {
                        *syscall_num.borrow_mut() = None;
                    });
                    IOCTL_PARAM.with(|param| {
                        *param.borrow_mut() = None;
                    });

                    let res = call_func(&host_funcs_cloned, &name_cloned, args_cloned);

                    SYSCALL_NUMBER.with(|syscall_num| {
                        if let Some(syscall) = *syscall_num.borrow() {
                            if syscall == libc::SYS_ioctl as usize {
                                IOCTL_PARAM.with(|param| {
                                    let ioctl_param = param.borrow().unwrap_or(0);
                                    Err(crate::HyperlightError::DisallowedSyscall(format!(
                                        "IOCTL({:x})",
                                        ioctl_param
                                    )))
                                })
                            } else {
                                Err(crate::HyperlightError::DisallowedSyscall(
                                    syscall.to_string(),
                                ))
                            }
                        } else {
                            res
                        }
                    })
                })?;

            join_handle.join().map_err(|_| new_error!("Error joining thread executing host function"))?
        } else {
            // Directly call the function without creating a new thread
            call_func(host_funcs, name, args)
        }
    }
}

/// The default writer function is to write to stdout with green text.
#[instrument(err(Debug), skip_all, parent = Span::current(), level = "Trace")]
pub(super) fn default_writer_func(s: String) -> Result<i32> {
    match stdout().is_terminal() {
        false => {
            print!("{}", s);
            Ok(s.len() as i32)
        }
        true => {
            let mut stdout = StandardStream::stdout(ColorChoice::Auto);
            let mut color_spec = ColorSpec::new();
            color_spec.set_fg(Some(Color::Green));
            stdout.set_color(&color_spec)?;
            stdout.write_all(s.as_bytes())?;
            stdout.reset()?;
            Ok(s.len() as i32)
        }
    }
}
