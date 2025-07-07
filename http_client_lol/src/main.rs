use tokio::*;
use serde::*;
use reqwest::*;

mod riotclient;

const API_TOKEN: &str = "";

#[tokio::main]
async fn main() {
    
    let account_puuid = riotclient::getRiotAccountPUUID(API_TOKEN).await;
    println!("{:?}", account_puuid.puuid);
}



