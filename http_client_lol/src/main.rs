use tokio::*;
use serde::*;
use reqwest::*;

mod riotmatchinfo;
mod riotclient;

#[derive(Debug, Deserialize)]
struct RiotAccount {
    pub puuid: String, 
    pub gameName: String, 
    pub tagLine: String
}

#[tokio::main]
async fn main() {
    
    riotmatchinfo::getRiotAccountMatches();

}



