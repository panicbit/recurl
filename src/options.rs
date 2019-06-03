use std::ffi::CStr;
use libc::*;
use reqwest::Method;
use crate::CURL;
use crate::borrow_raw::*;
use crate::raw::{
    CURLoption,
    CURLcode::{self, CURLE_OK, CURLE_BAD_FUNCTION_ARGUMENT},
};

pub struct Options {
    pub url: Option<String>,
    pub follow_location: bool,
    pub post_fields: Option<Vec<u8>>,
    pub method: Method,
}

impl Options {
    fn new() -> Self {
        Self {
            url: None,
            follow_location: false,
            post_fields: None,
            method: Method::GET,
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
    this.borrow_raw_mut(|this| {
        match option {
            CURLoption::CURLOPT_URL => set_option_url(this, args.arg()),
            CURLoption::CURLOPT_FOLLOWLOCATION => set_option_follow_location(this, args.arg()),
            CURLoption::CURLOPT_POSTFIELDS => set_option_post_fields(this, args.arg()),
            _ => CURLcode::CURLE_UNKNOWN_OPTION
        }
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}

unsafe fn set_option_url(curl: &mut CURL, url: *const c_char) -> CURLcode::Type {
    let url = match CStr::from_ptr(url).to_str() {
        Ok(url) => url.to_owned(),
        Err(_) => return CURLcode::CURLE_URL_MALFORMAT,
    };

    curl.options.url = Some(url);
    
    CURLE_OK
}

unsafe fn set_option_follow_location(curl: &mut CURL, state: c_long) -> CURLcode::Type {
    curl.options.follow_location = state == 1;
    CURLE_OK
}

unsafe fn set_option_post_fields(curl: &mut CURL, fields: *const c_char) -> CURLcode::Type {
    if fields.is_null() {
        curl.options.post_fields = None;
        return CURLE_OK;
    }

    let fields = CStr::from_ptr(fields).to_bytes().to_owned();
    curl.options.post_fields = Some(fields);
    curl.options.method = Method::POST;

    CURLE_OK
}
