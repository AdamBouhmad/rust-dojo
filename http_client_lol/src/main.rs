use tokio::*;
use serde::*;
use reqwest::*;
use chrono::{Local, TimeZone, Duration};



mod riotmatchinfo;
mod riotclient;

#[derive(Debug, Deserialize)]
pub struct RiotAccount {
    pub puuid: String, 
    pub gameName: String, 
    pub tagLine: String
}

#[tokio::main]
async fn main() {

    let startTime = local_day_bounds(2025, 7, 15).0;
    let endTime = local_day_bounds(2025, 7, 15).1;

    let mut MATCHES_API_ENDPOINT: String = format!("https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/tW6MoDoh-vuQd-GXOaJjc8tDXjVxgDaqENSQbDH87YHqpISCrlgYvhfVTHJqWcwxG6fqwt9AZX9c7A/ids?startTime={}&endTime={}&count=50", startTime, endTime);

    let get_account_matches = riotmatchinfo::getRiotAccountMatches(MATCHES_API_ENDPOINT).await;
    
    let matches_today = riotmatchinfo::getMatchesToday(get_account_matches).await;

    println!("{}", matches_today);
}


fn local_day_bounds(year: i32, month: u32, day: u32) -> (i64, i64) {
    // Start of the day at 00:00 local time
    let start = Local.ymd(year, month, day).and_hms(0, 0, 0);
    // End of the day at 23:59:59 local time
    let end = start + Duration::days(1) - Duration::seconds(1);
    (start.timestamp(), end.timestamp())
}



