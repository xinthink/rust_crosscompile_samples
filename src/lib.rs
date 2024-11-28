use std::os::raw::c_char;
use std::ffi::{CString, CStr};

// The shared function
pub fn add(left: i64, right: i64) -> i64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Interfaces for targeted OSs

#[cfg(feature = "android")]
use jni::{
    JNIEnv,
    objects::JClass,
    sys::jlong,
};

#[cfg(feature = "android")]
#[no_mangle]
pub extern "system" fn Java_com_example_rustsample_MathLib_add__JJ(
    _env: JNIEnv,
    _class: JClass,
    left: jlong,
    right: jlong,
) -> jlong {
    add(left, right)
}

#[cfg(feature = "ohos")]
use oh_napi_sys::{
    napi_env,
    napi_value,
    napi_callback_info,
};

#[cfg(feature = "ohos")]
pub extern "C" fn addd(env: napi_env, info: napi_callback_info) -> napi_value {
    unsafe {
        // let env = Env::from(env);

        let args = env.get_args(info);

        let a = env.get_int64_from(args[0]);
        let b = env.get_int64_from(args[1]);
        let c = add(a, b);

        env.create_int64(c)
    }
}

#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient + "!").unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
