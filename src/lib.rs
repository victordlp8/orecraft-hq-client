use crate::libutils::{is_secure, url_trimming};
use crate::mine::{mine, MineArgs};
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint, jlong, jstring};
use jni::JNIEnv;
use solana_sdk::signature::{read_keypair_file, Keypair};
use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc as tokio_mpsc;
use tokio::sync::Mutex;

mod balance;
mod database;
pub mod libutils;
mod mine;

#[no_mangle]
pub extern "system" fn Java_industries_dlp8_rust_RustBridge_helloRust<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    name: JString<'local>,
) -> jstring {
    let name: String = env.get_string(&name).expect("Couldn't get Java string!").into();
    let greeting = format!("Hello, I am {} the happy rustacean!", name);
    env.new_string(greeting).expect("Couldn't create Java string!").into_raw()
}

#[no_mangle]
pub extern "system" fn Java_industries_dlp8_rust_RustBridge_balancesOutput<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    keypair_path: JString<'local>,
    pool_url: JString<'local>,
) -> jstring {
    let keypair_path: String = env.get_string(&keypair_path).expect("Couldn't get Java string!").into();
    let pool_url: String = env.get_string(&pool_url).expect("Couldn't get Java string!").into();

    let keypair = read_keypair_file(&keypair_path).expect("Failed to read keypair file");
    let runtime = Runtime::new().expect("Failed to create runtime");

    let balances = runtime.block_on(async {
        let rewards = balance::get_rewards(&keypair, &pool_url).await;
        let wallet_balance = balance::get_balance(&keypair, &pool_url).await;
        let staked_balance = balance::get_stake(&keypair, &pool_url).await;
        [rewards, wallet_balance, staked_balance]
    });

    runtime.shutdown_timeout(std::time::Duration::from_secs(1));

    let result_string = format!("{},{},{}", balances[0], balances[1], balances[2]);
    env.new_string(result_string).unwrap_or_else(|_| {
        env.new_string("Error creating Java string").expect("Couldn't create error string")
    }).into_raw()
}


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
