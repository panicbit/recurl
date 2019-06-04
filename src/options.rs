use std::ffi::{CStr, VaList};
use std::str::Utf8Error;
use libc::*;
use reqwest::Method;
use crate::CURL;
use crate::borrow_raw::*;
use crate::raw::{
    CURLoption::{self, *},
    CURLcode::{self, *},
};
use crate::error::RootRcErrorBuffer;

pub struct Options {
    pub url: Option<String>,
    pub follow_location: bool,
    pub post_fields: Option<Vec<u8>>,
    pub method: Method,
    pub error_buffer: RootRcErrorBuffer,
}

impl Options {
    pub fn new() -> Self {
        Self {
            url: None,
            follow_location: false,
            post_fields: None,
            method: Method::GET,
            error_buffer: <_>::default(),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

#[no_mangle]
pub unsafe extern fn curl_easy_setopt(
    this: *mut CURL,
    option: CURLoption::Type,
    mut args: ...
) -> CURLcode::Type {
    this.borrow_raw_mut(|curl| {
        match option {
            CURLOPT_URL => owned_str_opt(args, |url| match url {
                Ok(url) => { curl.options.url = url; CURLE_OK },
                Err(e) => curl.error(CURLE_URL_MALFORMAT, e.to_string()),
            }),

            CURLOPT_FOLLOWLOCATION => long_opt(args, |state| {
                curl.options.follow_location = state == 1;
                CURLE_OK
            }),

            CURLOPT_POSTFIELDS => bytes_opt(args, |fields| {
                curl.options.method = Method::POST;
                curl.options.post_fields = fields.map(<_>::to_owned);
                CURLE_OK
            }),

            CURLOPT_ERRORBUFFER => {
                let buffer = args.arg::<*mut c_char>();
                curl.options.error_buffer
                    .borrow_mut()
                    .set_buffer(buffer);
                CURLE_OK
            },

            _ => CURLcode::CURLE_UNKNOWN_OPTION
        }
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}

unsafe fn str_opt<F, R>(mut args: VaList, f: F) -> R
where
    F: FnOnce(Result<Option<&str>, Utf8Error>) -> R
{
    let str = args.arg::<*const c_char>();

    if str.is_null() {
        return f(Ok(None));
    }

    let str = CStr::from_ptr(str).to_str().map(Some);
    f(str)
}

unsafe fn owned_str_opt<F, R>(args: VaList, f: F) -> R
where
    F: FnOnce(Result<Option<String>, Utf8Error>) -> R
{
    str_opt(args, |str| {
        let str = str.map(|str| str.map(<_>::to_owned));
        f(str)
    })
}

unsafe fn bytes_opt<F, R>(mut args: VaList, f: F) -> R
where
    F: FnOnce(Option<&[u8]>) -> R
{
    let bytes = args.arg::<*const c_char>();

    if bytes.is_null() {
        return f(None);
    }

    let bytes = CStr::from_ptr(bytes).to_bytes();
    f(Some(bytes))
}

unsafe fn long_opt<F, R>(mut args: VaList, f: F) -> R
where
    F: FnOnce(c_long) -> R
{
    let value = args.arg::<c_long>();
    f(value)
}
