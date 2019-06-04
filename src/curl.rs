use std::cell::RefCell;
use std::ffi::{CString, CStr};
use reqwest::RedirectPolicy;
use reqwest::header::{HeaderValue, CONTENT_TYPE, LAST_MODIFIED};
use chrono::{DateTime, FixedOffset};
use libc::*;
use std::io::{self, Write};
use crate::{Options, mime};
use crate::raw::CURLcode::{self, *};
use crate::borrow_raw::*;
use crate::util::root_rc::RootRc;
use crate::error::{ErrorBuffer, ErrorSink};

pub struct CURL {
    pub(crate) options: Options,
    pub(crate) last_effective_url: Option<CString>,
    mime: Option<mime::curl_mime>,
    pub(crate) file_time: Option<DateTime<FixedOffset>>,
    pub(crate) content_length_download: Option<u64>,
}

impl CURL {
    pub fn init() -> Box<CURL> {
        Box::new(Self {
            options: <_>::default(),
            mime: None,
            last_effective_url: None,
            file_time: None,
            content_length_download: None,
        })
    }

    pub fn into_raw(self: Box<Self>) -> *mut Self {
        Box::into_raw(self)
    }

    pub unsafe fn from_raw(this: *mut Self) -> Box<Self> {
        Box::from_raw(this)
    }

    pub fn error(&mut self, code: CURLcode::Type, message: impl Into<String>) -> CURLcode::Type {
        self.options.error_buffer.borrow_mut().set_error(code, message)
    }

    pub fn error_buffer(&self) -> &RootRc<RefCell<ErrorBuffer>> {
        &self.options.error_buffer
    }

    pub fn last_effective_url(&self) -> &CStr {
        self.last_effective_url.as_ref()
            .map(CString::as_c_str)
            .unwrap_or_default()
    }

    pub fn perform(&mut self) -> CURLcode::Type {
        let options = &mut self.options;

        let url = match options.url.as_ref() {
            Some(url) => url,
            None => return CURLE_OK,
        };

        let redirect_policy = match options.follow_location {
            true => RedirectPolicy::limited(30),
            false => RedirectPolicy::none(),
        };

        let client = reqwest::Client::builder()
            .redirect(redirect_policy)
            .connect_timeout(options.connect_timeout)
            .build()
            .unwrap();

        let mut request = client.request(options.method.clone(), url);

        if let Some(post_fields) = &options.post_fields {
            request = request.body(post_fields.to_owned());
            request = request.header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }

        let mut response = match request.send() {
            Ok(response) => response,
            Err(e) => return self.error(CURLE_HTTP_RETURNED_ERROR, e.to_string()),
        };

        self.content_length_download = response.content_length();

        if self.options.file_time {
            self.file_time = response.headers()
                .get(LAST_MODIFIED)
                .and_then(parse_last_modified);
        }

        // TODO: Improve handling of null in URLs
        self.last_effective_url = CStr::from_bytes_with_nul(response.url().as_str().as_bytes()).ok().map(<_>::into);

        let mut writer = FFIWriter {
            write_function: self.options.write_function,
            write_data: self.options.write_data,
        };

        if let Err(_) = response.copy_to(&mut writer) {
            return CURLE_HTTP_RETURNED_ERROR;
        }

        if !self.options.no_progress {
            // TODO: Improve progress with progress_streams and indicatif
            println!("Progress: 100%");
        }

        CURLE_OK
    }
}

impl ErrorSink for CURL {
    fn with_error_buffer<F>(&self, f: F) where F: FnOnce(&mut ErrorBuffer) {
        f(&mut self.options.error_buffer.borrow_mut())
    }
}

#[no_mangle]
pub extern fn curl_easy_init() -> *mut CURL {
    CURL::init().into_raw()
}

#[no_mangle]
pub unsafe extern fn curl_easy_reset(this: *mut CURL) {
    this.borrow_raw_mut(|this| {
        this.options = <_>::default();
    });
}

#[no_mangle]
pub unsafe extern fn curl_easy_cleanup(curl: *mut CURL) {
    if !curl.is_null() {
        CURL::from_raw(curl);
    }
}

#[no_mangle]
pub unsafe extern fn curl_easy_perform(this: *mut CURL) -> CURLcode::Type {
    this.borrow_raw_mut(CURL::perform)
        .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}

fn parse_last_modified(last_modified: &HeaderValue) -> Option<DateTime<FixedOffset>> {
    last_modified.to_str().ok().and_then(|last_modified| {
        DateTime::parse_from_rfc2822(last_modified.trim()).ok()
    })
}

struct FFIWriter {
    write_function: unsafe extern fn(
        ptr: *const c_char,
        size: size_t,
        nmemb: size_t,
        userdata: *mut c_void,
    ) -> size_t,
    write_data: *mut c_void,
}

impl Write for FFIWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        unsafe {
            Ok((self.write_function)(
                bytes.as_ptr() as *mut c_char,
                1,
                bytes.len(),
                self.write_data,
            ))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
