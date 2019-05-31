#![feature(c_variadic)]
extern crate reqwest;

use std::io::stdout;
use reqwest::RedirectPolicy;
use reqwest::Method;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use libc::*;
use crate::borrow_raw::*;
use crate::raw::CURLcode::{
    self,
    CURLE_OK,
    CURLE_BAD_FUNCTION_ARGUMENT,
};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod raw;
mod options;
mod slist;
mod mime;

mod handle;
mod borrow_raw;

mod rawx {
    pub const CURL_ZERO_TERMINATED: usize = usize::max_value() - 1;
}

pub struct CURL {
    url: Option<String>,
    follow_location: bool,
    method: Method,
    post_fields: Option<Vec<u8>>,
    mime: Option<mime::curl_mime>,
}

impl CURL {
    pub fn init() -> CURL {
        Self {
            url: None,
            follow_location: false,
            method: Method::GET,
            mime: None,
            post_fields: None,
        }
    }
}

#[no_mangle]
pub extern fn curl_global_init(flags: c_long) -> CURLcode::Type {
    CURLE_OK
}

#[no_mangle]
pub extern fn curl_easy_init() -> *mut CURL {
    let curl = CURL::init();
    let curl = Box::new(curl);
    let curl = Box::into_raw(curl);
    curl
}

#[no_mangle]
pub unsafe extern fn curl_easy_cleanup(curl: *mut CURL) {
    if !curl.is_null() {
        Box::from_raw(curl);
    }
}

#[no_mangle]
pub extern fn curl_easy_strerror(code: CURLcode::Type) -> *const c_char {
    match code {
        CURLcode::CURLE_BAD_FUNCTION_ARGUMENT => "Bad function argument\0".as_ptr() as _,
        CURLcode::CURLE_UNKNOWN_OPTION => "Unknown option\0".as_ptr() as _,
        CURLcode::CURLE_NOT_BUILT_IN => "Not built-in\0".as_ptr() as _,
        _ => "Unknown error code\0".as_ptr() as _,
    }
}

#[no_mangle]
pub unsafe extern fn curl_easy_perform(this: *mut CURL) -> CURLcode::Type {
    this.borrow_raw_mut(|this| {
        let url = match this.url.as_ref() {
            Some(url) => url,
            None => return CURLE_OK,
        };

        let redirect_policy = match this.follow_location {
            true => RedirectPolicy::limited(30),
            false => RedirectPolicy::none(),
        };
        let client = reqwest::Client::builder()
            .redirect(redirect_policy)
            .build()
            .unwrap();

        let mut request = client.request(this.method.clone(), url);

        if let Some(post_fields) = &this.post_fields {
            request = request.body(post_fields.to_owned());
            request = request.header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }

        let mut response = match request.send() {
            Ok(response) => response,
            Err(_) => return CURLcode::CURLE_HTTP_RETURNED_ERROR,
        };

        if let Err(_) = response.copy_to(&mut stdout()) {
            return CURLcode::CURLE_HTTP_RETURNED_ERROR;
        }

        CURLE_OK
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}

#[no_mangle]
pub unsafe extern fn curl_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return
    }

    unimplemented!("curl_free")
}

#[no_mangle]
pub unsafe extern fn curl_global_cleanup() {
}
