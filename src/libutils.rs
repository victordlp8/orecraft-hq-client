use jni::sys::jstring;
use jni::JNIEnv;

pub fn create_error_string(env: &mut JNIEnv) -> jstring {
    let error_string = env
        .new_string("-1.0,-1.0,-1.0")
        .expect("Couldn't create error string");
    error_string.into_raw()
}

pub fn url_trimming(url: &String) -> String {
    let trimmed_url = url
        .trim_start_matches("http://")
        .trim_start_matches("https://");
    trimmed_url.trim_end_matches('/').to_string()
}

pub fn is_secure(url: &String) -> bool {
    url.starts_with("https://") || url.starts_with("wss://")
}