use crate::error::HyperlightError::NoHypervisorFound;
use crate::{
    hypervisor::handlers::{MemAccessHandlerWrapper, OutBHandlerWrapper},
    hypervisor::Hypervisor,
    mem::{
        layout::SandboxMemoryLayout,
        mgr::SandboxMemoryManager,
        ptr::{GuestPtr, RawPtr},
        ptr_offset::Offset,
    },
    UninitializedSandbox,
};
use crate::{log_then_return, Result};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::{sync::MutexGuard, time::Duration};
use tracing::{instrument, Span};

/// A container with convenience methods attached for an
/// `Option<Box<dyn Hypervisor>>`
#[derive(Clone)]
pub(crate) struct HypervisorWrapper<'a> {
    hv_opt: Option<Arc<Mutex<Box<dyn Hypervisor>>>>,
    pub(crate) outb_hdl: OutBHandlerWrapper<'a>,
    pub(crate) mem_access_hdl: MemAccessHandlerWrapper<'a>,
    pub(crate) max_execution_time: Duration,
    pub(crate) max_wait_for_cancellation: Duration,
}

impl<'a> HypervisorWrapper<'a> {
    #[instrument(skip_all, parent = Span::current(), level= "Trace")]
    pub(super) fn new(
        hv_opt_box: Option<Box<dyn Hypervisor>>,
        outb_hdl: OutBHandlerWrapper<'a>,
        mem_access_hdl: MemAccessHandlerWrapper<'a>,
        max_execution_time: Duration,
        max_wait_for_cancellation: Duration,
    ) -> Self {
        Self {
            hv_opt: hv_opt_box.map(|hv| {
                let mutx = Mutex::from(hv);
                Arc::from(mutx)
            }),
            outb_hdl,
            mem_access_hdl,
            max_execution_time,
            max_wait_for_cancellation,
        }
    }

    /// if an internal `Hypervisor` exists, lock it and return a `MutexGuard`
    /// containing it.
    ///
    /// This `MutexGuard` represents exclusive read/write ownership of
    /// the underlying `Hypervisor`, so if this method returns an `Ok`,
    /// the value inside that `Ok` can be written or read.
    ///
    /// When the returned `MutexGuard` goes out of scope, the underlying lock
    /// will be released and the read/write guarantees will no longer be
    /// valid (the compiler won't let you do any operations on it, though,
    /// so you don't have to worry much about this consequence).
    #[instrument(err(Debug), skip_all, parent = Span::current(), level= "Trace")]
    pub(crate) fn get_hypervisor(&self) -> Result<MutexGuard<Box<dyn Hypervisor>>> {
        match self.hv_opt.as_ref() {
            None => {
                log_then_return!(NoHypervisorFound());
            }
            Some(h_arc_mut) => {
                let h_ref_mutex = Arc::as_ref(h_arc_mut);
                Ok(h_ref_mutex.lock()?)
            }
        }
    }
}

impl<'a> UninitializedSandbox<'a> {
    /// Set up the appropriate hypervisor for the platform
    ///
    #[instrument(err(Debug), skip_all, parent = Span::current(), level= "Trace")]
    pub(super) fn set_up_hypervisor_partition(
        mgr: &mut SandboxMemoryManager,
    ) -> Result<Box<dyn Hypervisor>> {
        let mem_size = u64::try_from(mgr.shared_mem.mem_size())?;
        let rsp_ptr = {
            let rsp_u64 = mgr.set_up_hypervisor_partition(mem_size)?;
            let rsp_raw = RawPtr::from(rsp_u64);
            GuestPtr::try_from(rsp_raw)
        }?;
        let base_ptr = GuestPtr::try_from(Offset::from(0))?;
        let pml4_ptr = {
            let pml4_offset_u64 = u64::try_from(SandboxMemoryLayout::PML4_OFFSET)?;
            base_ptr + Offset::from(pml4_offset_u64)
        };
        let entrypoint_ptr = {
            let entrypoint_total_offset = mgr.load_addr.clone() + mgr.entrypoint_offset;
            GuestPtr::try_from(entrypoint_total_offset)
        }?;
        let guard_page_offset = u64::from(mgr.layout.get_guard_page_offset());
        assert!(base_ptr == pml4_ptr);
        assert!(entrypoint_ptr > pml4_ptr);
        assert!(rsp_ptr > entrypoint_ptr);

        #[cfg(target_os = "linux")]
        {
            use crate::hypervisor::hypervisor_mem::HypervisorAddrs;
            use crate::hypervisor::{hyperv_linux, hyperv_linux::HypervLinuxDriver};
            use crate::hypervisor::{kvm, kvm::KVMDriver};
            use hyperlight_flatbuffers::mem::PAGE_SHIFT;

            if hyperv_linux::is_hypervisor_present().unwrap_or(false) {
                // the following line resolves to page frame number 512, because it's BASE_ADDRESS / 4096.
                // Each page is 4096 bytes, so this is the number of pages to the base address,
                // which will exactly result in the memory starting at the base address.
                let guest_pfn = u64::try_from(SandboxMemoryLayout::BASE_ADDRESS >> PAGE_SHIFT)?;
                let host_addr = u64::try_from(mgr.shared_mem.base_addr())?;
                let addrs = HypervisorAddrs {
                    entrypoint: entrypoint_ptr.absolute()?,
                    guest_pfn,
                    host_addr,
                    guard_page_offset,
                    mem_size,
                };
                let hv = HypervLinuxDriver::new(&addrs, rsp_ptr, pml4_ptr)?;
                Ok(Box::new(hv))
            } else if kvm::is_hypervisor_present().is_ok() {
                let host_addr = u64::try_from(mgr.shared_mem.base_addr())?;
                let hv = KVMDriver::new(
                    host_addr,
                    pml4_ptr.absolute()?,
                    guard_page_offset,
                    mem_size,
                    entrypoint_ptr.absolute()?,
                    rsp_ptr.absolute()?,
                )?;
                Ok(Box::new(hv))
            } else {
                log_then_return!(
                    "Linux platform detected, but neither KVM nor Linux HyperV detected"
                );
            }
        }
        #[cfg(target_os = "windows")]
        {
            use crate::hypervisor::hyperv_windows::HypervWindowsDriver;
            use crate::hypervisor::windows_hypervisor_platform;
            if windows_hypervisor_platform::is_hypervisor_present().unwrap_or(false) {
                let source_addr = mgr.shared_mem.raw_ptr();
                let guest_base_addr = u64::try_from(SandboxMemoryLayout::BASE_ADDRESS)?;
                let hv = HypervWindowsDriver::new(
                    mgr.shared_mem.mem_size(),
                    source_addr,
                    guest_base_addr,
                    guard_page_offset,
                    pml4_ptr.absolute()?,
                    entrypoint_ptr.absolute()?,
                    rsp_ptr.absolute()?,
                )?;
                Ok(Box::new(hv))
            } else {
                log_then_return!(NoHypervisorFound());
            }
        }
    }
}

impl<'a> Debug for HypervisorWrapper<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HypervisorWrapper")
            .field("has_hypervisor", &self.hv_opt.is_some())
            .finish()
    }
}
