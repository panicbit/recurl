#![feature(c_variadic)]

#[macro_use] extern crate c_str_macro;

use std::io::stdout;
use std::cell::RefCell;
use reqwest::RedirectPolicy;
use reqwest::Method;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use libc::*;
use crate::borrow_raw::*;
use crate::raw::CURLcode::{self, *};
use util::root_rc::RootRc;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod raw;
pub mod util;
mod options;
mod slist;
mod mime;
mod info;

mod error;
use error::{ErrorBuffer, ErrorSink};

mod handle;
mod borrow_raw;

mod rawx {
    use libc::*;
    pub const CURL_ZERO_TERMINATED: size_t = size_t::max_value() - 1;
}

pub struct CURL {
    url: Option<String>,
    last_effective_url: Option<String>,
    follow_location: bool,
    method: Method,
    post_fields: Option<Vec<u8>>,
    mime: Option<mime::curl_mime>,
    error_buffer: RootRc<RefCell<ErrorBuffer>>,
}

impl CURL {
    pub fn init() -> CURL {
        Self {
            url: None,
            follow_location: false,
            method: Method::GET,
            mime: None,
            post_fields: None,
            last_effective_url: None,
            error_buffer: <_>::default(),
        }
    }

    pub fn error(&mut self, code: CURLcode::Type, message: impl Into<String>) -> CURLcode::Type {
        self.error_buffer.borrow_mut().set_error(code, message)
    }

    pub fn error_buffer(&self) -> &RootRc<RefCell<ErrorBuffer>> {
        &self.error_buffer
    }
}

impl ErrorSink for CURL {
    fn with_error_buffer<F>(&self, f: F) where F: FnOnce(&mut ErrorBuffer) {
        f(&mut self.error_buffer.borrow_mut())
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
        CURLE_BAD_FUNCTION_ARGUMENT => c_str!("Bad function argument"),
        CURLE_UNKNOWN_OPTION => c_str!("Unknown option"),
        CURLE_NOT_BUILT_IN => c_str!("Not built-in"),
        _ => c_str!("Unknown error code"),
    }
    .as_ptr()
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
            Err(e) => return this.error(CURLE_HTTP_RETURNED_ERROR, e.to_string()),
        };

        if let Err(_) = response.copy_to(&mut stdout()) {
            return CURLE_HTTP_RETURNED_ERROR;
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

    // TODO
}

#[no_mangle]
pub unsafe extern fn curl_global_cleanup() {
}
