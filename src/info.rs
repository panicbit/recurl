use libc::*;
use crate::borrow_raw::*;
use crate::CURL;
use crate::raw::CURLINFO::{self, *};
use crate::raw::CURLcode::{self, *};

#[no_mangle]
pub unsafe extern fn curl_easy_getinfo(
    curl: *mut CURL,
    info: CURLINFO::Type,
    mut args:...
) -> CURLcode::Type 
{
    curl.borrow_raw_mut(|curl| {
        match info {
            CURLINFO_NONE => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_EFFECTIVE_URL => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_RESPONSE_CODE => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_TOTAL_TIME => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_NAMELOOKUP_TIME => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_CONNECT_TIME => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_PRETRANSFER_TIME => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SIZE_UPLOAD => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SIZE_UPLOAD_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SIZE_DOWNLOAD => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SIZE_DOWNLOAD_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SPEED_DOWNLOAD => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SPEED_DOWNLOAD_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SPEED_UPLOAD => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SPEED_UPLOAD_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_HEADER_SIZE => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_REQUEST_SIZE => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SSL_VERIFYRESULT => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_FILETIME => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_FILETIME_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_CONTENT_LENGTH_DOWNLOAD => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_CONTENT_LENGTH_DOWNLOAD_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_CONTENT_LENGTH_UPLOAD => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_CONTENT_LENGTH_UPLOAD_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_STARTTRANSFER_TIME => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_CONTENT_TYPE => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_REDIRECT_TIME => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_REDIRECT_COUNT => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_PRIVATE => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_HTTP_CONNECTCODE => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_HTTPAUTH_AVAIL => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_PROXYAUTH_AVAIL => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_OS_ERRNO => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_NUM_CONNECTS => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SSL_ENGINES => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_COOKIELIST => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_LASTSOCKET => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_FTP_ENTRY_PATH => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_REDIRECT_URL => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_PRIMARY_IP => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_APPCONNECT_TIME => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_CERTINFO => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_CONDITION_UNMET => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_RTSP_SESSION_ID => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_RTSP_CLIENT_CSEQ => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_RTSP_SERVER_CSEQ => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_RTSP_CSEQ_RECV => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_PRIMARY_PORT => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_LOCAL_IP => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_LOCAL_PORT => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_TLS_SESSION => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_ACTIVESOCKET => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_TLS_SSL_PTR => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_HTTP_VERSION => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_PROXY_SSL_VERIFYRESULT => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_PROTOCOL => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_SCHEME => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_TOTAL_TIME_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_NAMELOOKUP_TIME_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_CONNECT_TIME_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_PRETRANSFER_TIME_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_STARTTRANSFER_TIME_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_REDIRECT_TIME_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_APPCONNECT_TIME_T => CURLE_BAD_FUNCTION_ARGUMENT,
            CURLINFO_LASTONE => CURLE_BAD_FUNCTION_ARGUMENT,
            _ => CURLE_BAD_FUNCTION_ARGUMENT,
        }
    })
    .unwrap_or(CURLE_BAD_FUNCTION_ARGUMENT)
}