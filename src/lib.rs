use jni::JNIEnv;
use jni::sys::jlong;
use jni::objects::JClass;

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
#[ohos_node_bindgen]
pub extern "C" fn add(left: i64, right: i64) -> i64 {
    add(left, right)
}
