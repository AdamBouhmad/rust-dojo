//use tokio::*;
use reqwest::*;
use crate::RiotAccount;


pub async fn getriotaccount_puuid(api_token: &str) -> Result<RiotAccount> {
    let client = reqwest::Client::new();
    let response: RiotAccount = client
        .get("https://americas.api.riotgames.com/riot/account/v1/accounts/by-riot-id/I%20Am%20Camping%20You/NA1")
        .header("X-Riot-Token", api_token)
        .send()
        .await?
        .json()
        .await?;

    Ok(response) 
}