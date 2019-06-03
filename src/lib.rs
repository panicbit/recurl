#![feature(c_variadic)]

#[macro_use] extern crate c_str_macro;

use libc::*;
use crate::raw::CURLcode::{self, *};

mod curl;
use crate::curl::CURL;

#[allow(warnings)]
mod raw;
mod util;
mod options;
mod slist;
mod mime;
mod info;
mod error;
mod handle;
mod borrow_raw;

mod rawx {
    use libc::*;
    pub const CURL_ZERO_TERMINATED: size_t = size_t::max_value() - 1;
}

#[no_mangle]
pub extern fn curl_global_init(flags: c_long) -> CURLcode::Type {
    CURLE_OK
}

#[no_mangle]
pub unsafe extern fn curl_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return
    }

    // TODO
}

#[no_mangle]
pub unsafe extern fn curl_global_cleanup() {
}
