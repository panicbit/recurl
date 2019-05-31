use crate::CURL;
use crate::borrow_raw::*;
use crate::raw::CURLcode::{
    self,
    CURLE_OK,
    CURLE_BAD_FUNCTION_ARGUMENT,
};
use crate::rawx::CURL_ZERO_TERMINATED;
use libc::*;
use std::ptr::null_mut;
use std::ffi::CStr;
use std::slice;

#[allow(non_camel_case_types)]
pub struct curl_mime {
    // Invariant: curl_mimepart is pinned to the heap,
    // because pointers to it are handed out to C-land
    parts: Vec<Box<curl_mimepart>>,
}

impl curl_mime {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            parts: Vec::new(),
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

#[allow(non_camel_case_types)]
pub struct curl_mimepart {
    name: Option<String>,
    data: Option<Vec<u8>>,
    mime_type: Option<String>,
}

impl curl_mimepart {
    fn new() -> Box<Self> {
        Box::new(Self {
            data: None,
            name: None,
            mime_type: None,
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
}

#[no_mangle]
pub unsafe extern fn curl_init(_curl: *mut CURL) -> *mut curl_mime {
    curl_mime::new().into_raw()
}

#[no_mangle]
pub unsafe extern fn curl_mime_addpart(mime: *mut curl_mime) -> *mut curl_mimepart {
    mime.borrow_raw_mut(|mime| {
        let part_ptr = Box::into_raw(curl_mimepart::new());
        let part = Box::from_raw(part_ptr);

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
            Err(_) => return CURLE_BAD_FUNCTION_ARGUMENT,
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
            Err(_) => return CURLE_BAD_FUNCTION_ARGUMENT,
        };

        part.set_mime_type(Some(mime_type));

        CURLE_OK
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}
