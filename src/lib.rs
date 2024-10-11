use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jdoubleArray, jstring};
use solana_sdk::signature::read_keypair_file;
use tokio::runtime::Runtime;

mod balance;
mod libutils;

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
    let keypair_path: String = env.get_string(&keypair_path).expect("Couldn't get Java string!").into();
    let pool_url: String = env.get_string(&pool_url).expect("Couldn't get Java string!").into();

    let keypair = match read_keypair_file(&keypair_path) {
        Ok(kp) => kp,
        Err(_) => return create_error_array(&mut env),
    };

    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    let balances = runtime.block_on(async {
        let rewards = balance::get_rewards(&keypair, &pool_url).await;
        let wallet_balance = balance::get_balance(&keypair, &pool_url).await;
        let staked_balance = balance::get_stake(&keypair, &pool_url).await;
        [rewards, wallet_balance, staked_balance]
    });

    let result = env.new_double_array(3).expect("Couldn't create Java array");
    env.set_double_array_region(&result, 0, &balances).expect("Couldn't set array region");

    result.into_raw()
}