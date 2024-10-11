use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jdoubleArray, jstring};
use solana_sdk::signature::{read_keypair_file, Keypair};

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
    let keypair_path: String = env.get_string(&keypair_path).expect("Couldn't get Java string!").into();
    let pool_url: String = env.get_string(&pool_url).expect("Couldn't get Java string!").into();

    let keypair = match read_keypair_file(&keypair_path) {
        Ok(kp) => kp,
        Err(_) => return create_error_array(&mut env),
    };

    let rewards = get_balance_internal(&mut env, &keypair, &pool_url).unwrap_or(-1.0);
    let wallet_balance = get_balance_internal(&mut env, &keypair, &pool_url).unwrap_or(-1.0);
    let staked_balance = get_balance_internal(&mut env, &keypair, &pool_url).unwrap_or(-1.0);

    let result = env.new_double_array(3).expect("Couldn't create Java array");
    let values = [rewards, wallet_balance, staked_balance];
    env.set_double_array_region(&result, 0, &values).expect("Couldn't set array region");

    result.into_raw()
}

fn get_balance_internal(env: &mut JNIEnv, keypair: &Keypair, pool_url: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let balance = runtime.block_on(async {
        match pool_url.split('/').last() {
            Some("rewards") => balance::get_rewards(keypair, &pool_url.to_string()).await,
            Some("stake") => balance::get_stake(keypair, &pool_url.to_string()).await,
            _ => balance::get_balance(keypair, &pool_url.to_string()).await,
        }
    });
    
    Ok(balance)
}