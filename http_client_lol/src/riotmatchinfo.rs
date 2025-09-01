use serde::*;
use crate::riotclient;
use serde_json::Value;
use serde_json::to_string_pretty;


pub struct MatchCounter { 
    pub totalgamestoday: u8,
}

#[derive(Debug, Deserialize)]
pub struct GameInsights {
    pub gameduration: u32,
    pub gametype: String,

}
impl GameInsights {
    pub fn is_ranked(&self) -> bool {
        self.gametype == "MATCHED_GAME"
    }
}



const API_TOKEN: &str = "api token";



pub async fn getriotaccountmatches(matches_api_endpoint: String) -> Value {

    let _player_uuid = riotclient::getriotaccount_puuid(API_TOKEN).await;

    let client = reqwest::Client::new();

    let player_matches: Value = client.get(matches_api_endpoint).header("X-Riot-Token", API_TOKEN).send().await.unwrap().json::<Value>().await.unwrap();

    return player_matches;

}

pub async fn _getmatchestoday(player_matches: Value) -> u8 {

    let _client = reqwest::Client::new();

    let mut gamestoday = MatchCounter {
        totalgamestoday: 0
    };

    for i in player_matches.as_array().unwrap() {
        gamestoday.totalgamestoday += 1; 
    };

    return gamestoday.totalgamestoday;

}

pub async fn getmatchinsightsweekly(player_matches: &Value) -> (u8, u32, u8, u32) {
    let client = reqwest::Client::new();

    let mut gamestoday = MatchCounter {
        totalgamestoday: 0,
    };

    let mut total_duration = 0;
    let mut total_ranked_matches = 0;
    let mut longest_game = 0;

    for i in player_matches.as_array().unwrap() {
        let matchinfo_api_endpoint: &str = &(("https://americas.api.riotgames.com/lol/match/v5/matches/").to_owned() + i.as_str().unwrap());

        let match_details = client
            .get(matchinfo_api_endpoint)
            .header("X-Riot-Token", API_TOKEN)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        let _json = to_string_pretty(&match_details).unwrap();

        let info = &match_details["info"];
        if let Ok(insights) = serde_json::from_value::<GameInsights>(info.clone()) {
            total_duration += insights.gameduration;

            if insights.gameduration > longest_game {
                longest_game = insights.gameduration;
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

