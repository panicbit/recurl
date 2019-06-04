use std::ffi::{VaList, CStr};
use std::convert::TryFrom;
use libc::*;
use crate::borrow_raw::*;
use crate::CURL;
use crate::raw::CURLINFO::{self, *};
use crate::raw::CURLcode::{self, *};

unsafe fn str_info(mut args: VaList, str: &CStr) {
    let ret = args.arg::<*mut *const c_char>();
    *ret = str.as_ptr();
}

unsafe fn long_info(mut args: VaList, value: c_long) {
    let ret = args.arg::<*mut c_long>();
    *ret = value;
}

#[no_mangle]
pub unsafe extern fn curl_easy_getinfo(
    curl: *mut CURL,
    info: CURLINFO::Type,
    args:...
) -> CURLcode::Type 
{
    curl.borrow_raw_mut(|curl| {
        match info {
            // CURLINFO_NONE => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_EFFECTIVE_URL => str_info(args, curl.last_effective_url()),
            CURLINFO_FILETIME => long_info(args, curl.file_time.map(|t| t.timestamp()).unwrap_or(-1)),
            CURLINFO_CONTENT_LENGTH_DOWNLOAD => long_info(args, curl.content_length_download.and_then(|l| i64::try_from(l).ok()).unwrap_or(-1)),
            CURLINFO_SIZE_DOWNLOAD => long_info(args, curl.size_download as c_long),
            CURLINFO_CONDITION_UNMET => long_info(args, 1), // TODO: implement conditions
            CURLINFO_RESPONSE_CODE => long_info(args, curl.response_code as c_long),
            CURLINFO_TOTAL_TIME => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_TOTAL_TIME)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_NAMELOOKUP_TIME => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_NAMELOOKUP_TIME)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_CONNECT_TIME => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_CONNECT_TIME)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_PRETRANSFER_TIME => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_PRETRANSFER_TIME)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SIZE_UPLOAD => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SIZE_UPLOAD)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SIZE_UPLOAD_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SIZE_UPLOAD_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SIZE_DOWNLOAD_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SIZE_DOWNLOAD_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SPEED_DOWNLOAD => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SPEED_DOWNLOAD)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SPEED_DOWNLOAD_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SPEED_DOWNLOAD_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SPEED_UPLOAD => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SPEED_UPLOAD)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SPEED_UPLOAD_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SPEED_UPLOAD_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_HEADER_SIZE => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_HEADER_SIZE)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_REQUEST_SIZE => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_REQUEST_SIZE)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SSL_VERIFYRESULT => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SSL_VERIFYRESULT)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_FILETIME_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_FILETIME_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_CONTENT_LENGTH_DOWNLOAD_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_CONTENT_LENGTH_DOWNLOAD_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_CONTENT_LENGTH_UPLOAD => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_CONTENT_LENGTH_UPLOAD)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_CONTENT_LENGTH_UPLOAD_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_CONTENT_LENGTH_UPLOAD_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_STARTTRANSFER_TIME => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_STARTTRANSFER_TIME)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_CONTENT_TYPE => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_CONTENT_TYPE)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_REDIRECT_TIME => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_REDIRECT_TIME)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_REDIRECT_COUNT => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_REDIRECT_COUNT)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_PRIVATE => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_PRIVATE)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_HTTP_CONNECTCODE => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_HTTP_CONNECTCODE)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_HTTPAUTH_AVAIL => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_HTTPAUTH_AVAIL)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_PROXYAUTH_AVAIL => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_PROXYAUTH_AVAIL)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_OS_ERRNO => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_OS_ERRNO)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_NUM_CONNECTS => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_NUM_CONNECTS)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SSL_ENGINES => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SSL_ENGINES)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_COOKIELIST => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_COOKIELIST)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_LASTSOCKET => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_LASTSOCKET)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_FTP_ENTRY_PATH => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_FTP_ENTRY_PATH)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_REDIRECT_URL => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_REDIRECT_URL)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_PRIMARY_IP => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_PRIMARY_IP)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_APPCONNECT_TIME => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_APPCONNECT_TIME)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_CERTINFO => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_CERTINFO)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_CONDITION_UNMET => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_CONDITION_UNMET)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_RTSP_SESSION_ID => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_RTSP_SESSION_ID)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_RTSP_CLIENT_CSEQ => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_RTSP_CLIENT_CSEQ)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_RTSP_SERVER_CSEQ => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_RTSP_SERVER_CSEQ)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_RTSP_CSEQ_RECV => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_RTSP_CSEQ_RECV)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_PRIMARY_PORT => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_PRIMARY_PORT)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_LOCAL_IP => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_LOCAL_IP)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_LOCAL_PORT => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_LOCAL_PORT)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_TLS_SESSION => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_TLS_SESSION)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_ACTIVESOCKET => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_ACTIVESOCKET)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_TLS_SSL_PTR => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_TLS_SSL_PTR)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_HTTP_VERSION => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_HTTP_VERSION)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_PROXY_SSL_VERIFYRESULT => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_PROXY_SSL_VERIFYRESULT)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_PROTOCOL => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_PROTOCOL)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_SCHEME => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_SCHEME)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_TOTAL_TIME_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_TOTAL_TIME_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_NAMELOOKUP_TIME_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_NAMELOOKUP_TIME_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_CONNECT_TIME_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_CONNECT_TIME_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_PRETRANSFER_TIME_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_PRETRANSFER_TIME_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_STARTTRANSFER_TIME_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_STARTTRANSFER_TIME_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_REDIRECT_TIME_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_REDIRECT_TIME_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_APPCONNECT_TIME_T => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_APPCONNECT_TIME_T)); return CURLE_BAD_FUNCTION_ARGUMENT},
            CURLINFO_LASTONE => {eprintln!("recurl: unimplemented '{}'", stringify!(CURLINFO_LASTONE)); return CURLE_BAD_FUNCTION_ARGUMENT},
            _ => return CURLE_BAD_FUNCTION_ARGUMENT,
        };

        CURLE_OK
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}
