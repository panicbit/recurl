use std::ffi::{CStr, VaList};
use std::str::Utf8Error;
use std::time::Duration;
use std::mem::transmute;
use std::ptr::null_mut;
use libc::*;
use reqwest::Method;
use crate::CURL;
use crate::util::borrow_raw::*;
use crate::raw::{
    stdout,
    CURLoption::{self, *},
    CURLcode::{self, *},
};
use crate::error::RootRcErrorBuffer;

type WriteFunction = unsafe extern fn(
    ptr: *const c_char,
    size: size_t,
    nitems: size_t,
    userdata: *mut c_void,
) -> size_t;

const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(300);

pub struct Options {
    pub url: Option<String>,
    pub follow_location: bool,
    pub post_fields: Option<Vec<u8>>,
    pub method: Method,
    pub error_buffer: RootRcErrorBuffer,
    pub connect_timeout: Option<Duration>,
    pub file_time: bool,
    pub no_progress: bool,
    pub write_function: WriteFunction,
    pub write_data: *mut c_void,
    pub header_function: Option<WriteFunction>,
    pub header_data: *mut c_void,
}

impl Options {
    pub fn new() -> Self {
        Self {
            url: None,
            follow_location: false,
            post_fields: None,
            method: Method::GET,
            error_buffer: <_>::default(),
            connect_timeout: Some(DEFAULT_CONNECT_TIMEOUT),
            file_time: false,
            no_progress: true,
            write_function: default_write_function,
            write_data: unsafe { stdout as *mut c_void },
            header_function: None,
            header_data: null_mut(),
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

            CURLOPT_FOLLOWLOCATION => bool_opt(args, |state| {
                curl.options.follow_location = state;
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

            CURLOPT_CONNECTTIMEOUT => long_opt(args, |timeout| {
                curl.options.connect_timeout = Some(match timeout {
                    0 => DEFAULT_CONNECT_TIMEOUT,
                    _ => Duration::from_secs(timeout as u64),
                });
                CURLE_OK
            }),

            CURLOPT_CONNECTTIMEOUT_MS => long_opt(args, |timeout| {
                curl.options.connect_timeout = match timeout {
                    0 => None,
                    _ => Some(Duration::from_millis(timeout as u64)),
                };
                CURLE_OK
            }),

            CURLOPT_FILETIME => bool_opt(args, |state| {
                curl.options.file_time = state;
                CURLE_OK
            }),

            CURLOPT_NOPROGRESS => bool_opt(args, |state| {
                curl.options.no_progress = state;
                CURLE_OK
            }),

            CURLOPT_WRITEDATA => {
                curl.options.write_data = args.arg::<*mut c_void>();
                CURLE_OK
            },

            CURLOPT_HEADERFUNCTION => {
                let ptr = args.arg::<*const c_void>();
                let header_function = transmute::<_, WriteFunction>(ptr);
                curl.options.header_function = Some(header_function).filter(|_| !ptr.is_null());
                CURLE_OK
            }

            CURLOPT_HEADERDATA => {
                curl.options.header_data = args.arg::<*mut c_void>();
                CURLE_OK
            }

            _ => {
                eprintln!("recurl: unknown option ({})", option);
                CURLcode::CURLE_UNKNOWN_OPTION
            }
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

unsafe fn bool_opt<F, R>(args: VaList, f: F) -> R
where
    F: FnOnce(bool) -> R
{
    long_opt(args, |value| f(value == 1))
}

unsafe extern fn default_write_function(ptr: *const c_char, size: size_t, nmemb: size_t, userdata: *mut c_void) -> size_t {
    fwrite(ptr as *const c_void, size, nmemb, userdata as *mut FILE)
}
