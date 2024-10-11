use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jdoubleArray, jstring};
use solana_sdk::signature::read_keypair_file;

mod libutils;
mod balance;

use libutils::create_error_array;

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
    pool_url: JString<'local>
) -> jdoubleArray {
    // Convert JNI parameters to Rust types
    let keypair_path: String = match env.get_string(&keypair_path) {
        Ok(s) => s.into(),
        Err(_) => return create_error_array(&mut env),
    };
    let pool_url: String = match env.get_string(&pool_url) {
        Ok(s) => s.into(),
        Err(_) => return create_error_array(&mut env),
    };

    // Read keypair from file
    let keypair = match read_keypair_file(&keypair_path) {
        Ok(kp) => kp,
        Err(_) => return create_error_array(&mut env),
    };

    // Call the balance functions asynchronously
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return create_error_array(&mut env),
    };
    
    let (rewards, wallet_balance, staked_balance) = runtime.block_on(async {
        tokio::join!(
            balance::get_rewards(&keypair, &pool_url),
            balance::get_balance(&keypair, &pool_url),
            balance::get_stake(&keypair, &pool_url)
        )
    });

    // Check if any of the balance functions returned an error
    if rewards < 0.0 || wallet_balance < 0.0 || staked_balance < 0.0 {
        return create_error_array(&mut env);
    }

    // Create a Java double array to hold the results
    let result = match env.new_double_array(3) {
        Ok(arr) => arr,
        Err(_) => return create_error_array(&mut env),
    };

    // Convert Rust f64 to Java double and set array elements
    let values = [rewards, wallet_balance, staked_balance];
    if let Err(_) = env.set_double_array_region(&result, 0, &values) {
        return create_error_array(&mut env);
    }

    result.into_raw()
}