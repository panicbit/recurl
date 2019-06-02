use std::io::stdout;
use std::cell::RefCell;
use reqwest::RedirectPolicy;
use reqwest::Method;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use crate::mime;
use crate::raw::CURLcode::{self, *};
use crate::borrow_raw::*;
use crate::util::root_rc::RootRc;
use crate::error::{ErrorBuffer, ErrorSink};

pub struct CURL {
    pub(crate) url: Option<String>,
    last_effective_url: Option<String>,
    pub(crate) follow_location: bool,
    pub(crate) method: Method,
    pub(crate) post_fields: Option<Vec<u8>>,
    mime: Option<mime::curl_mime>,
    error_buffer: RootRc<RefCell<ErrorBuffer>>,
}

impl CURL {
    pub fn init() -> Box<CURL> {
        Box::new(Self {
            url: None,
            follow_location: false,
            method: Method::GET,
            mime: None,
            post_fields: None,
            last_effective_url: None,
            error_buffer: <_>::default(),
        })
    }

    pub fn into_raw(self: Box<Self>) -> *mut Self {
        Box::into_raw(self)
    }

    pub unsafe fn from_raw(this: *mut Self) -> Box<Self> {
        Box::from_raw(this)
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
pub extern fn curl_easy_init() -> *mut CURL {
    CURL::init().into_raw()
}

#[no_mangle]
pub unsafe extern fn curl_easy_cleanup(curl: *mut CURL) {
    if !curl.is_null() {
        CURL::from_raw(curl);
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
            Err(e) => return this.error(CURLE_HTTP_RETURNED_ERROR, e.to_string()),
        };

        if let Err(_) = response.copy_to(&mut stdout()) {
            return CURLE_HTTP_RETURNED_ERROR;
        }

        CURLE_OK
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}
