use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jdoubleArray, jstring};

mod balance;
mod libutils;

use crate::libutils::{read_keypair_from_file, create_error_array};

#[no_mangle]
pub extern "system" fn Java_industries_dlp8_rust_RustBridge_helloRust<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, name: JString<'local>) -> jstring {
    // Convert the Java String to a Rust String
    let name: String = env.get_string(&name).expect("Couldn't get Java string!").into();
    
    // Create the greeting message
    let greeting = format!("Hello, I am {} the happy rustacean!", name);
    
    // Convert the Rust String back to a Java String
    let output = env.new_string(greeting).expect("Couldn't create Java string!");

    // Return the Java String
    return output.into_raw();
}

#[no_mangle]
pub extern "system" fn Java_industries_dlp8_rust_RustBridge_getBalances<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    keypair_path: JString<'local>,
    url: JString<'local>
) -> jdoubleArray {
    // Convert JNI parameters to Rust types
    let keypair_path: String = env.get_string(&keypair_path).expect("Couldn't get keypair path!").into();
    let url: String = env.get_string(&url).expect("Couldn't get URL!").into();

    // Read keypair from file
    let keypair = match read_keypair_from_file(&keypair_path) {
        Ok(kp) => kp,
        Err(e) => {
            eprintln!("Failed to read keypair: {}", e);
            return create_error_array(&mut env);
        }
    };

    // Call the balance functions asynchronously
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let (rewards, wallet_balance, staked_balance) = runtime.block_on(async {
        tokio::join!(
            balance::get_rewards(&keypair, url.clone(), false),
            balance::get_balance(&keypair, url.clone(), false),
            balance::get_stake(&keypair, url, false)
        )
    });

    // Create a Java double array to hold the results
    let result = env.new_double_array(3).expect("Couldn't create new double array");

    // Convert Rust f64 to Java double and set array elements
    let values = [rewards, wallet_balance, staked_balance];
    env.set_double_array_region(&result, 0, &values).expect("Couldn't set array elements");

    return result.into_raw()
}