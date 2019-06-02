use crate::CURL;
use crate::borrow_raw::*;
use crate::raw::CURLcode::{self, *};
use crate::rawx::CURL_ZERO_TERMINATED;
use crate::error::*;
use libc::*;
use std::ptr::null_mut;
use std::ffi::CStr;
use std::slice;

#[allow(non_camel_case_types)]
pub struct curl_mime {
    // Invariant: curl_mimepart is pinned to the heap,
    // because pointers to it are handed out to C-land
    parts: Vec<Box<curl_mimepart>>,
    error_buffer: WeakErrorBuffer,
}

impl curl_mime {
    pub fn new(curl: Option<&CURL>) -> Box<Self> {
        Box::new(Self {
            parts: Vec::new(),
            error_buffer: curl.map(|curl| curl.error_buffer().weak()).unwrap_or_default(),
        })
    }

    pub fn add_part(&mut self, part: Box<curl_mimepart>) {
        self.parts.push(part);
    }

    pub fn into_raw(self: Box<Self>) -> *mut Self {
        Box::into_raw(self)
    }

    pub unsafe fn from_raw(this: *mut Self) {
        if !this.is_null() {
            Box::from_raw(this);
        }
    }
}

impl ErrorSink for curl_mime {
    fn with_error_buffer<F>(&self, f: F) where F: FnOnce(&mut ErrorBuffer) {
        self.error_buffer.with_ref(|buf| f(&mut buf.borrow_mut()));
    }
}

#[allow(non_camel_case_types)]
pub struct curl_mimepart {
    name: Option<String>,
    data: Option<Vec<u8>>,
    mime_type: Option<String>,
    error_buffer: WeakErrorBuffer,
}

impl curl_mimepart {
    fn new(error_buffer: WeakErrorBuffer) -> Box<Self> {
        Box::new(Self {
            data: None,
            name: None,
            mime_type: None,
            error_buffer,
        })
    }

    fn set_data(&mut self, data: Option<Vec<u8>>) {
        self.data = data;
    }

    fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    fn set_mime_type(&mut self, mime_type: Option<String>) {
        self.mime_type = mime_type;
    }

    fn into_raw(self: Box<Self>) -> *mut Self {
        Box::into_raw(self)
    }

    unsafe fn from_raw(this: *mut Self) -> Box<Self> {
        Box::from_raw(this)
    }
}

impl ErrorSink for curl_mimepart {
    fn with_error_buffer<F>(&self, f: F) where F: FnOnce(&mut ErrorBuffer) {
        self.error_buffer.with_ref(|buf| f(&mut buf.borrow_mut()));
    }
}

#[no_mangle]
pub unsafe extern fn curl_mime_init(curl: *mut CURL) -> *mut curl_mime {
    curl.borrow_raw_opt(curl_mime::new).into_raw()
}

#[no_mangle]
pub unsafe extern fn curl_mime_addpart(mime: *mut curl_mime) -> *mut curl_mimepart {
    mime.borrow_raw_mut(|mime| {
        let error_buffer = mime.error_buffer.clone();
        let part_ptr = curl_mimepart::new(error_buffer).into_raw();
        let part = curl_mimepart::from_raw(part_ptr);

        mime.add_part(part);

        part_ptr
    })
    .unwrap_or(null_mut())
}

#[no_mangle]
pub unsafe extern fn curl_mime_free(mime: *mut curl_mime) {
    curl_mime::from_raw(mime);
}

#[no_mangle]
pub unsafe extern fn curl_mime_data(
    part: *mut curl_mimepart,
    data: *const c_char,
    mut datasize: size_t,
) -> CURLcode::Type {
    part.borrow_raw_mut(|part| {
        if data.is_null() {
            part.set_data(None);
            return CURLE_OK;
        }
    
        if datasize == CURL_ZERO_TERMINATED {
            datasize = CStr::from_ptr(data).to_bytes().len();
        }

        let data = slice::from_raw_parts(data as *const u8, datasize);
        let data = data.to_owned();

        part.set_data(Some(data));

        CURLE_OK
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}

#[no_mangle]
pub unsafe extern fn curl_mime_name(
    part: *mut curl_mimepart,
    name: *const c_char,
) -> CURLcode::Type {
    part.borrow_raw_mut(|part| {
        if name.is_null() {
            part.set_name(None);
            return CURLE_OK;
        }

        let name = match CStr::from_ptr(name).to_str() {
            Ok(name) => name.to_owned(),
            Err(e) => return part.error(CURLE_BAD_FUNCTION_ARGUMENT, e.to_string()),
        };

        part.set_name(Some(name));

        CURLE_OK
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}

#[no_mangle]
pub unsafe extern fn curl_mime_type(
    part: *mut curl_mimepart,
    mime_type: *const c_char,
) -> CURLcode::Type {
    part.borrow_raw_mut(|part| {
        if mime_type.is_null() {
            part.set_mime_type(None);
            return CURLE_OK;
        }

        let mime_type = match CStr::from_ptr(mime_type).to_str() {
            Ok(mime_type) => mime_type.to_owned(),
            Err(e) => return part.error(CURLE_BAD_FUNCTION_ARGUMENT, e.to_string()),
        };

        part.set_mime_type(Some(mime_type));

        CURLE_OK
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}
