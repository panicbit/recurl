use std::ffi::CStr;
use libc::*;
use reqwest::Method;
use crate::CURL;
use crate::borrow_raw::*;
use crate::raw::{
    CURLoption,
    CURLcode::{self, CURLE_OK, CURLE_BAD_FUNCTION_ARGUMENT},
};

impl CURL {
    pub fn set_option_url(&mut self, url: Option<String>) {
        self.url = url;
    }

    pub fn set_option_follow_location(&mut self, state: bool) {
        self.follow_location = state;
    }

    pub fn set_option_post_fields(&mut self, fields: Option<Vec<u8>>) {
        self.method = Method::POST;
        self.post_fields = fields;
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
    curl.set_option_url(Some(url));
    CURLE_OK
}

unsafe fn set_option_follow_location(curl: &mut CURL, state: c_long) -> CURLcode::Type {
    curl.set_option_follow_location(state == 1);
    CURLE_OK
}

unsafe fn set_option_post_fields(curl: &mut CURL, fields: *const c_char) -> CURLcode::Type {
    if fields.is_null() {
        curl.set_option_post_fields(None);
        return CURLE_OK;
    }

    let fields = CStr::from_ptr(fields).to_bytes().to_owned();
    curl.set_option_post_fields(Some(fields));

    CURLE_OK
}
