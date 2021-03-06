use std::cell::RefCell;
use std::ffi::{CString, CStr};
use std::io::{self, Write};
use reqwest::RedirectPolicy;
use reqwest::header::{HeaderValue, CONTENT_TYPE, LAST_MODIFIED};
use chrono::{DateTime, FixedOffset};
use progress_streams::ProgressReader;
use libc::*;
use crate::{Options, Infos, mime};
use crate::raw::{
    CURLcode::{self, *},
    curl_off_t,
};
use crate::util::{
    borrow_raw::*,
    root_rc::RootRc,
};
use crate::error::{ErrorBuffer, ErrorSink};
pub struct CURL {
    pub options: Options,
    pub infos: Infos,
    mime: Option<mime::curl_mime>,
}

impl CURL {
    pub fn init() -> Box<CURL> {
        Box::new(Self {
            options: Options::new(),
            infos: Infos::new(),
            mime: None,
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
        self.infos
            .last_effective_url.as_ref()
            .map(CString::as_c_str)
            .unwrap_or_default()
    }

    pub fn perform(&mut self) -> CURLcode::Type {
        let options = &mut self.options;
        let infos = &mut self.infos;

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

        if cfg!(debug_assertions) {
            eprintln!("recurl: Requesting {:?}", options.url);
        }

        let response = match request.send() {
            Ok(response) => response,
            Err(e) => return self.error(CURLE_HTTP_RETURNED_ERROR, e.to_string()),
        };

        infos.response_code = response.status().as_u16();
        infos.content_length_download = response.content_length();

        if options.file_time {
            infos.file_time = response.headers()
                .get(LAST_MODIFIED)
                .and_then(parse_last_modified);
        }

        // TODO: Improve handling of null in URLs
        infos.last_effective_url = CStr::from_bytes_with_nul(response.url().as_str().as_bytes()).ok().map(<_>::into);

        let mut header_function = options.header_function;
        if !options.header_data.is_null() {
            header_function.get_or_insert(options.write_function);
        }

        // TODO: Include ALL header data, not just fields
        if let Some(header_function) = header_function {
            for (header, value) in response.headers() {
                let mut field = Vec::with_capacity(header.as_str().len() + 2 + value.len() + 2);
                write!(&mut field, "{}: ", header).ok();
                field.extend_from_slice(value.as_bytes());
                write!(&mut field, "\r\n").ok();

                unsafe {
                    let res = header_function(field.as_ptr() as *const c_char, 1, field.len(), options.header_data);
                    if res != field.len() {
                        return self.error(CURLE_WRITE_ERROR, "Error while handling headers");
                    }
                }
            }
        }

        let mut writer = FFIWriter {
            write_function: options.write_function,
            write_data: options.write_data,
        };

        infos.size_download = 0;

        let mut reader = ProgressReader::new(response, |dl_progress| {
            infos.size_download += dl_progress as u64;

            if options.no_progress {
                return;
            }

            // TOOD: Allow xfer_info_function to abort dl
            unsafe {
                let dl_total = infos.content_length_download.unwrap_or(0) as curl_off_t;
                (options.xfer_info_function)(
                    options.xfer_info_data,
                    dl_total,
                    infos.size_download as curl_off_t,
                    0,
                    0,
                );
            }
        });

        // TODO: Handle CURL_WRITEFUNC_PAUSE
        infos.size_download = match io::copy(&mut reader, &mut writer) {
            Ok(size_download) => size_download,
            Err(e) => return self.error(CURLE_HTTP_RETURNED_ERROR, e.to_string()),
        };

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
