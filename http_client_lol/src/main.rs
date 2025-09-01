use serde::*;

mod riotmatchinfo;
mod riotclient;
mod datetime;
mod emailingservice;


#[derive(Debug, Deserialize)]
pub struct RiotAccount {
    pub puuid: String, 
    pub game_name: String, 
    pub tag_line: String
}

#[tokio::main]
async fn main() {

    let start_time = datetime::local_day_bounds(2025, 7, 01).0;
    let end_time = datetime::local_day_bounds(2025, 8, 05).1;

    let matches_api_endpoint: String = format!("https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/tW6MoDoh-vuQd-GXOaJjc8tDXjVxgDaqENSQbDH87YHqpISCrlgYvhfVTHJqWcwxG6fqwt9AZX9c7A/ids?startTime={}&endTime={}&count=100", start_time, end_time);

    let get_account_matches = riotmatchinfo::getriotaccountmatches(matches_api_endpoint).await;
    
    let (matches_thisweek, timespent_thisweek, total_rankedmatches_thisweek, longest_game) = riotmatchinfo::getmatchinsightsweekly(&get_account_matches).await;

    emailingservice::emailservice(matches_thisweek, timespent_thisweek, total_rankedmatches_thisweek, longest_game).await;
}



