use solana_sdk::{signature::Keypair, signer::Signer};

use crate::libutils::is_secure;

#[derive(Debug)]
pub enum BalanceType {
    Wallet,
    Rewards,
    Stake,
}

pub async fn get_rewards(key: &Keypair, url: &String) -> f64 {
    get_balance_by_type(key, url, BalanceType::Rewards).await
}

pub async fn get_balance(key: &Keypair, url: &String) -> f64 {
    get_balance_by_type(key, url, BalanceType::Wallet).await
}

pub async fn get_stake(key: &Keypair, url: &String) -> f64 {
    get_balance_by_type(key, url, BalanceType::Stake).await
}

pub async fn balance(key: &Keypair, url: &String) {
    let (rewards, balance, staked_balance) = tokio::join!(
        get_rewards(key, url),
        get_balance(key, url),
        get_stake(key, url)
    );

    println!("  Unclaimed Rewards: {:.11} ORE", rewards);
    println!("  Wallet (Stakable): {:.11} ORE", balance);
    println!("  Staked Balance:    {:.11} ORE", staked_balance);
}

pub async fn get_balance_by_type(key: &Keypair, url: &String, balance_type: BalanceType) -> f64 {
    let client = reqwest::Client::new();
    let url_prefix = if is_secure(&url) { "https" } else { "http" };

    let endpoint = match balance_type {
        BalanceType::Wallet => "balance",
        BalanceType::Rewards => "rewards",
        BalanceType::Stake => "stake",
    };

    let response = client
        .get(format!(
            "{}://{}/miner/{}?pubkey={}",
            url_prefix,
            url,
            endpoint,
            key.pubkey().to_string()
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    match balance_type {
        BalanceType::Stake if response.contains("Failed to g") => 0.0,
        _ => response.parse::<f64>().unwrap_or(0.0),
    }
}
