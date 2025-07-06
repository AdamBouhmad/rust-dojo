use tokio::*;
use serde::*;
use reqwest::*;

#[derive(Debug, Deserialize)]
struct RiotAccount {
    puuid: String, 
    gameName: String, 
    tagLine: String
}

const API_TOKEN: &str = "my_hardcoded_api_token :)";

#[tokio::main]
async fn main() {
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
    
    println!("{:?}", response);
}
