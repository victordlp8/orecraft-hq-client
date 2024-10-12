use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use solana_sdk::signature::read_keypair_file;
use tokio::runtime::Runtime;

mod balance;
mod libutils;

use libutils::create_error_string;

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
pub extern "system" fn Java_industries_dlp8_rust_RustBridge_balancesOutput<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    keypair_path: JString<'local>,
    pool_url: JString<'local>,
) -> jstring {
    let keypair_path: String = match env.get_string(&keypair_path) {
        Ok(s) => s.into(),
        Err(_) => return create_error_string(&mut env),
    };
    let pool_url: String = match env.get_string(&pool_url) {
        Ok(s) => s.into(),
        Err(_) => return create_error_string(&mut env),
    };

    let keypair = match read_keypair_file(&keypair_path) {
        Ok(kp) => kp,
        Err(_) => return create_error_string(&mut env),
    };

    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return create_error_string(&mut env),
    };

    let balances = runtime.block_on(async {
        let rewards = balance::get_rewards(&keypair, &pool_url).await;
        let wallet_balance = balance::get_balance(&keypair, &pool_url).await;
        let staked_balance = balance::get_stake(&keypair, &pool_url).await;
        [rewards, wallet_balance, staked_balance]
    });

    // Explicitly shut down the runtime
    runtime.shutdown_timeout(std::time::Duration::from_secs(1));

    let result_string = format!("{},{},{}", balances[0], balances[1], balances[2]);

    // Convert the Rust String to a Java String
    let output = match env.new_string(result_string) {
        Ok(s) => s,
        Err(_) => {
            let error_message = "Error creating Java string";
            env.new_string(error_message).expect("Couldn't create error string")
        }
    };

    // Return the Java String
    output.into_raw()
}
