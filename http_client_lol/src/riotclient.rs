//use tokio::*;
use reqwest::*;
use serde::*;

#[derive(Debug, Deserialize)]
pub struct RiotAccount {
    pub puuid: String, 
    pub gameName: String, 
    pub tagLine: String
}

pub async fn getRiotAccountPUUID(API_TOKEN: &str) -> RiotAccount {
    let client = reqwest::Client::new();
    let response: RiotAccount = client
        .get("https://americas.api.riotgames.com/riot/account/v1/accounts/by-riot-id/I%20Am%20Camping%20You/NA1")
        .header("X-Riot-Token", API_TOKEN)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    response
}