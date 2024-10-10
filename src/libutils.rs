use std::fs::File;
use std::io::Read;
use solana_sdk::signature::Keypair;
use jni::JNIEnv;
use jni::sys::jdoubleArray;

pub fn read_keypair_from_file(path: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut keypair_bytes = Vec::new();
    file.read_to_end(&mut keypair_bytes)?;
    Ok(Keypair::from_bytes(&keypair_bytes)?)
}

pub fn create_error_array(env: &mut JNIEnv) -> jdoubleArray {
    let error_array = env.new_double_array(3).expect("Couldn't create error array");
    let error_values = [-1.0, -1.0, -1.0];
    env.set_double_array_region(&error_array, 0, &error_values).expect("Couldn't set error array elements");
    return error_array.into_raw()
}

pub fn is_secure(url: &String) -> bool {
    url.starts_with("https://") || url.starts_with("wss://")
}