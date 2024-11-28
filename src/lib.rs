// The shared function
// pub fn add(left: i64, right: i64) -> i64 {
//     left + right
// }

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

// #[cfg(feature = "ios")]
use std::{
    os::raw::c_char,
    ffi::{CString, CStr},
};

// #[cfg(feature = "ios")]
#[no_mangle]
pub extern fn rust_greeting(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };

    CString::new("Hello ".to_owned() + recipient + "!").unwrap().into_raw()
}

// #[cfg(feature = "ios")]
#[no_mangle]
pub extern fn rust_greeting_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

// use napi_derive_ohos::napi;

// #[napi]
pub fn add(left: i64, right: i64) -> i64 {
    left + right
}

// #[link(name = "ace_napi.z")]
// #[link(name = "ace_ndk.z")]
// extern "C" {

// }
