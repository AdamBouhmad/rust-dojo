use tokio::*;
use serde::*;
use crate::riotclient;
use crate::RiotAccount;
use serde_json::Value;
use serde_json::to_string_pretty;
use std::fs;
use chrono::{Utc, TimeZone, Duration};


pub struct MatchCounter { 
    pub totalgamestoday: u8,
}


const API_TOKEN: &str = "";



pub async fn getRiotAccountMatches(MATCHES_API_ENDPOINT: String) -> Value {

    let player_uuid = riotclient::getRiotAccountPUUID(API_TOKEN).await;

    let client = reqwest::Client::new();

    let player_matches: Value = client.get(MATCHES_API_ENDPOINT).header("X-Riot-Token", API_TOKEN).send().await.unwrap().json::<Value>().await.unwrap();

    return player_matches;

}

pub async fn getMatchesToday(player_matches: Value) -> u8 {

    let client = reqwest::Client::new();

    let mut gamestoday = MatchCounter {
        totalgamestoday: 0
    };

    for i in player_matches.as_array().unwrap() {
        let mut MATCHINFO_API_ENDPOINT: &str = &(("https://americas.api.riotgames.com/lol/match/v5/matches/").to_owned() + i.as_str().unwrap());

        let mut match_details = client.get(MATCHINFO_API_ENDPOINT).header("X-Riot-Token", API_TOKEN).send().await.unwrap().json::<Value>().await.unwrap();
        let mut json = to_string_pretty(&match_details).unwrap();
        //println!("{:?}", json);

        gamestoday.totalgamestoday += 1; 
    };

    return gamestoday.totalgamestoday;

}