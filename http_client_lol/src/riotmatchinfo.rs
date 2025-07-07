use tokio::*;
use serde::*;
use crate::riotclient;
use crate::riotclient::RiotAccount;


const MATCHES_API_ENDPOINT: &str = "https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{puuid}/ids";
const API_TOKEN: &str = "";


pub async fn getRiotAccountMatches() -> RiotAccount {

    let player_uuid = riotclient::getRiotAccountPUUID(API_TOKEN).await;

    println!("{:?}", player_uuid);

    return player_uuid

}