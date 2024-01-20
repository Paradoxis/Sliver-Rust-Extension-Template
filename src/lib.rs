use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::str;

type GoCallback = extern "C" fn(*const c_char, c_int) -> c_int;

#[no_mangle]
pub extern "C" fn entrypoint(args_buffer: *mut c_char, buffer_size: u32, cb: GoCallback) -> c_int {
    // Parse the arguments
    let args: &str = match std::str::from_utf8(unsafe {
        std::slice::from_raw_parts(args_buffer as *const u8, size as usize)
    }) {
        Ok(v) => v,
        Err(_) => return 1,
    };

    // Return a greeting
    let greeting = format!("Hello, {}!", args);
    return callback(&greeting, cb);
}

fn callback(msg: &str, cb: GoCallback) -> c_int {
    let msg = CString::new(msg.to_string()).unwrap();
    cb(msg.as_ptr(), msg.to_bytes().len() as c_int)
}

#[no_mangle]
pub extern "C" fn DllMain(
    h_module: *mut c_void,
    ul_reason_for_call: u32,
    lp_reserved: *mut c_void,
) -> bool {
    true
}
