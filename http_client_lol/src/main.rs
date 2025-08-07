use tokio::*;
use serde::*;
use reqwest::*;
use chrono::{Utc, TimeZone, Duration as ChronoDuration};
use humantime::format_duration;
use std::time::Duration;

mod riotmatchinfo;
mod riotclient;
mod datetime;
mod emailingservice;


#[derive(Debug, Deserialize)]
pub struct RiotAccount {
    pub puuid: String, 
    pub gameName: String, 
    pub tagLine: String
}

#[tokio::main]
async fn main() {

    let startTime = datetime::local_day_bounds(2025, 7, 01).0;
    let endTime = datetime::local_day_bounds(2025, 8, 05).1;

    let mut MATCHES_API_ENDPOINT: String = format!("https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/tW6MoDoh-vuQd-GXOaJjc8tDXjVxgDaqENSQbDH87YHqpISCrlgYvhfVTHJqWcwxG6fqwt9AZX9c7A/ids?startTime={}&endTime={}&count=100", startTime, endTime);

    let get_account_matches = riotmatchinfo::getRiotAccountMatches(MATCHES_API_ENDPOINT).await;
    
    let (matches_thisweek, timespent_thisweek, total_rankedmatches_thisweek, longest_game) = riotmatchinfo::getMatchInsightsWeekly(&get_account_matches).await;

    emailingservice::emailservice(matches_thisweek, timespent_thisweek, total_rankedmatches_thisweek, longest_game).await;
}



