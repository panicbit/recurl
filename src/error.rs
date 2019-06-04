use std::ptr::null_mut;
use std::cmp::min;
use std::slice;
use std::cell::RefCell;
use libc::*;
use crate::util::root_rc::{RootRc, Weak};
use crate::raw::{CURLcode, CURL_ERROR_SIZE};

pub type RootRcErrorBuffer = RootRc<RefCell<ErrorBuffer>>;
pub type WeakErrorBuffer = Weak<RefCell<ErrorBuffer>>;

pub trait ErrorSink {
    fn with_error_buffer<F>(&self, f: F) where F: FnOnce(&mut ErrorBuffer);

    fn error(&mut self, code: CURLcode::Type, message: impl Into<String>) -> CURLcode::Type {
        self.with_error_buffer(|error_buffer| {
            error_buffer.set_error(code, message);
        });

        code
    }
}

pub struct ErrorBuffer {
    buffer: *mut u8,
}

impl ErrorBuffer {
    pub fn new() -> Self {
        Self {
            buffer: null_mut(),
        }
    }

    pub unsafe fn set_buffer(&mut self, buffer: *mut c_char) {
        self.buffer = buffer as *mut u8;
    }

    pub fn set_error(&mut self, code: CURLcode::Type, message: impl Into<String>) -> CURLcode::Type {
        if self.buffer.is_null() {
            return code;
        }

        let message = message.into();
        let buffer = unsafe { slice::from_raw_parts_mut(self.buffer, CURL_ERROR_SIZE as usize) };
        // TODO: Regard char boundaries when calculating len
        let len = min(message.len(), buffer.len() - 1);
        
        buffer[..len].copy_from_slice(&message.as_bytes()[..len]);
        buffer[len] = 0;

        code
    }
}

#[no_mangle]
pub extern fn curl_easy_strerror(code: CURLcode::Type) -> *const c_char {
    use crate::CURLcode::*;
    match code {
        CURLE_BAD_FUNCTION_ARGUMENT => c_str!("Bad function argument"),
        CURLE_UNKNOWN_OPTION => c_str!("Unknown option"),
        CURLE_NOT_BUILT_IN => c_str!("Not built-in"),
        _ => c_str!("Unknown error code"),
    }
    .as_ptr()
}

impl Default for ErrorBuffer {
    fn default() -> Self {
        Self::new()
    }
}
