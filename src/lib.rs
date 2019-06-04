#![feature(c_variadic)]

#[macro_use] extern crate c_str_macro;

use libc::*;
use crate::raw::CURLcode::{self, *};

mod curl;
use crate::curl::CURL;

mod options;
use crate::options::Options;

mod info;
use crate::info::Infos;

#[allow(warnings)]
mod raw;
mod util;
mod slist;
mod mime;
mod error;

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
