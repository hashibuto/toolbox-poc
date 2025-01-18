use std::{
    ffi::{c_char, CStr}, thread::sleep, time::Duration
};

#[no_mangle]
pub unsafe extern "C" fn handle_remote_http_session_rs(endpoint: *const c_char) -> i64 {
    // this is the unsafe seam, whereby all intermediary processing occurs.
    let c_str = CStr::from_ptr(endpoint);
    let result = c_str.to_str();
    if result.is_err() {
        return -1;
    }

    let r_endpoint = result.unwrap();
    return handle_remote_http_session(r_endpoint);
}

// this code is all safe now
fn handle_remote_http_session(endpoint: &str) -> i64 {
    for i in 1..5 {
        println!("calling endpoint ({}): {}", i, endpoint);
        sleep(Duration::from_secs(1));
    }

    return 0;
}
