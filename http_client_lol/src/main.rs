use tokio::*;
use serde::*;
use reqwest::*;

mod riotmatchinfo;
mod riotclient;
mod datetime;

#[derive(Debug, Deserialize)]
pub struct RiotAccount {
    pub puuid: String, 
    pub gameName: String, 
    pub tagLine: String
}

#[tokio::main]
async fn main() {

    let startTime = datetime::local_day_bounds(2025, 7, 20).0;
    let endTime = datetime::local_day_bounds(2025, 7, 20).1;

    let mut MATCHES_API_ENDPOINT: String = format!("https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/tW6MoDoh-vuQd-GXOaJjc8tDXjVxgDaqENSQbDH87YHqpISCrlgYvhfVTHJqWcwxG6fqwt9AZX9c7A/ids?startTime={}&endTime={}&count=50", startTime, endTime);

    let get_account_matches = riotmatchinfo::getRiotAccountMatches(MATCHES_API_ENDPOINT).await;
    
    let matches_today = riotmatchinfo::getMatchesToday(get_account_matches).await;

    println!("{}", matches_today);
}



