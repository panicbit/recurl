use std::os::raw::*;
use std::ffi::CStr;
use std::ptr::null_mut;

#[allow(non_camel_case_types)]
pub struct curl_slist {
    elements: Vec<String>,
}

impl curl_slist {
    fn new() -> Box<Self> {
        Box::new(Self {
            elements: Vec::new(),
        })
    }

    fn append(&mut self, value: String) {
        self.elements.push(value);
    }

    pub fn into_raw(self: Box<Self>) -> *mut Self {
        Box::into_raw(self)
    }

    pub unsafe fn from_raw(this: *mut Self) {
        if this.is_null() {
            return;
        }

        Box::from_raw(this);
    }
}

#[no_mangle]
pub unsafe extern fn curl_slist_append(
    this: *mut curl_slist,
    value: *const c_char
) -> *mut curl_slist
{
    if value.is_null() {
        return null_mut();
    }

    let value = match CStr::from_ptr(value).to_str() {
        Ok(value) => value.to_owned(),
        Err(_) => return null_mut(),
    };

    let this = match this.is_null() {
        true => &mut *curl_slist::new().into_raw(),
        false => &mut *this,
    };

    this.append(value);

    this
}

#[no_mangle]
pub unsafe extern fn curl_slist_free_all(this: *mut curl_slist) {
    curl_slist::from_raw(this);
}
