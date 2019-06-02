use std::ptr::null_mut;
use std::cmp::min;
use std::slice;
use libc::*;
use crate::raw::{CURLcode, CURL_ERROR_SIZE};

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

impl Default for ErrorBuffer {
    fn default() -> Self {
        Self::new()
    }
}
