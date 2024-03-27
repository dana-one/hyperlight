use core::ffi::c_char;
use core::ffi::CStr;
use log::error;

use alloc::{string::ToString, vec::Vec};
use hyperlight_flatbuffers::flatbuffer_wrappers::guest_error::{ErrorCode, GuestError};

use crate::host_function_call::outb;
use crate::host_function_call::OutBAction;
use crate::{entrypoint::halt, P_PEB};
use alloc::string::String;

pub(crate) fn write_error(error_code: ErrorCode, message: Option<&str>) {
    let guest_error = GuestError::new(
        error_code.clone(),
        message.map_or("".to_string(), |m| m.to_string()),
    );
    let mut guest_error_buffer: Vec<u8> = (&guest_error)
        .try_into()
        .expect("Invalid guest_error_buffer, could not be converted to a Vec<u8>");

    unsafe {
        assert!(!(*P_PEB.unwrap()).guestErrorData.guestErrorBuffer.is_null());
        let len = guest_error_buffer.len();
        if guest_error_buffer.len() > (*P_PEB.unwrap()).guestErrorData.guestErrorSize as usize {
            error!(
                "Guest error buffer is too small to hold the error message: size {} buffer size {} message may be truncated",
                guest_error_buffer.len(),
                (*P_PEB.unwrap()).guestErrorData.guestErrorSize as usize
            );
            // get the length of the message
            let message_len = message.map_or("".to_string(), |m| m.to_string()).len();
            // message is too long, truncate it
            let truncate_len = message_len
                - (guest_error_buffer.len()
                    - (*P_PEB.unwrap()).guestErrorData.guestErrorSize as usize);
            let truncated_message = message
                .map_or("".to_string(), |m| m.to_string())
                .chars()
                .take(truncate_len)
                .collect::<String>();
            let guest_error = GuestError::new(error_code, truncated_message);
            guest_error_buffer = (&guest_error)
                .try_into()
                .expect("Invalid guest_error_buffer, could not be converted to a Vec<u8>");
        }

        // Optimally, we'd use copy_from_slice here, but, because
        // p_guest_error_buffer is a *mut c_void, we can't do that.
        // Instead, we do the copying manually using pointer arithmetic.
        // Plus; before, we'd do an assert w/ the result from copy_from_slice,
        // but, because copy_nonoverlapping doesn't return anything, we can't do that.
        // Instead, we do the prior asserts/checks to check the destination pointer isn't null
        // and that there is enough space in the destination buffer for the copy.
        let dest_ptr = (*P_PEB.unwrap()).guestErrorData.guestErrorBuffer as *mut u8;
        core::ptr::copy_nonoverlapping(guest_error_buffer.as_ptr(), dest_ptr, len);
    }
}

pub(crate) fn reset_error() {
    unsafe {
        let peb_ptr = P_PEB.unwrap();
        core::ptr::write_bytes(
            (*peb_ptr).guestErrorData.guestErrorBuffer,
            0,
            (*peb_ptr).guestErrorData.guestErrorSize as usize,
        );
    }
}

pub(crate) fn set_error(error_code: ErrorCode, message: &str) {
    write_error(error_code, Some(message));
}

pub(crate) fn set_error_and_halt(error_code: ErrorCode, message: &str) {
    set_error(error_code, message);
    halt();
}

#[no_mangle]
pub(crate) extern "win64" fn set_stack_allocate_error() {
    outb(OutBAction::Abort as u16, ErrorCode::StackOverflow as u8);
}

/// Exposes a C API to allow the guest to set an error
///
/// # Safety
/// TODO
#[no_mangle]
#[allow(non_camel_case_types)]
pub unsafe extern "C" fn setError(code: u64, message: *const c_char) {
    let error_code = ErrorCode::from(code);
    match message.is_null() {
        true => write_error(error_code, None),
        false => {
            let message = unsafe { CStr::from_ptr(message).to_str().ok() }
                .expect("Invalid error message, could not be converted to a string");
            write_error(error_code, Some(message));
        }
    }
    halt();
}
