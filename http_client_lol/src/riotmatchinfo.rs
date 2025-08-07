use tokio::*;
use serde::*;
use crate::riotclient;
use crate::RiotAccount;
use serde_json::Value;
use serde_json::to_string_pretty;
use std::fs;
use chrono::{Utc, TimeZone, Duration as ChronoDuration};
use humantime::format_duration;
use std::time::Duration;



pub struct MatchCounter { 
    pub totalgamestoday: u8,
}

#[derive(Debug, Deserialize)]
pub struct GameInsights {
    pub gameDuration: u32,
    pub gameType: String,

}
impl GameInsights {
    pub fn is_ranked(&self) -> bool {
        self.gameType == "MATCHED_GAME"
    }
}



const API_TOKEN: &str = "api token";



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
        gamestoday.totalgamestoday += 1; 
    };

    return gamestoday.totalgamestoday;

}

pub async fn getMatchInsightsWeekly(player_matches: &Value) -> (u8, u32, u8, u32) {
    let client = reqwest::Client::new();

    let mut gamestoday = MatchCounter {
        totalgamestoday: 0,
    };

    let mut total_duration = 0;
    let mut total_ranked_matches = 0;
    let mut longest_game = 0;
    let mut iterator = 0;
    let mut average_pings = 0;

    for i in player_matches.as_array().unwrap() {
        let MATCHINFO_API_ENDPOINT: &str = &(("https://americas.api.riotgames.com/lol/match/v5/matches/").to_owned() + i.as_str().unwrap());

        let match_details = client
            .get(MATCHINFO_API_ENDPOINT)
            .header("X-Riot-Token", API_TOKEN)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        let json = to_string_pretty(&match_details).unwrap();

        let info = &match_details["info"];
        if let Ok(insights) = serde_json::from_value::<GameInsights>(info.clone()) {
            total_duration += insights.gameDuration;

            if insights.gameDuration > longest_game {
                longest_game = insights.gameDuration;
                iterator +=1; 
            }

            if insights.is_ranked() {
                total_ranked_matches +=1;
            }
        }
        gamestoday.totalgamestoday += 1;

       /* println!("{}", json);
        break;*/

    }

    println!("{},{}, {}, {}", gamestoday.totalgamestoday, total_duration, total_ranked_matches, longest_game);

    return (gamestoday.totalgamestoday, total_duration, total_ranked_matches, longest_game);
}

