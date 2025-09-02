use serde::*;

use eframe::egui; 

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

    #[derive(Default)]
    struct QueueCap {
        matches_thisweek: u8,
        get_total_matches_today: u8,
        timespent_thisweek: u32,
    }
    // We implement the `eframe::App` trait for our struct.
    impl eframe::App for QueueCap {
        // The `update` function is called repeatedly, once per frame.
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("QueueCap is a tool to help users stick to a set maximum number of games a day to improve LoL Performance");
                ui.horizontal(|ui| {
                    ui.label(format!("Total Games Played Today {}", self.get_total_matches_today));
                    ui.label(format!("Total Games Played This Week {}", self.matches_thisweek));
                    ui.label(format!("Total Time Played {}", self.timespent_thisweek));
                });
            });
        }
    }



#[tokio::main]
async fn main() -> Result<(), eframe::Error> {

    let start_time = datetime::local_day_bounds(2025, 7, 25).0;
    let end_time = datetime::local_day_bounds(2025, 7, 31).1;

    let matches_api_endpoint_weekly: String = format!("https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/tW6MoDoh-vuQd-GXOaJjc8tDXjVxgDaqENSQbDH87YHqpISCrlgYvhfVTHJqWcwxG6fqwt9AZX9c7A/ids?startTime={}&endTime={}&count=100", start_time, end_time);

    let matches_api_endpoint_daily: String = format!("https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/tW6MoDoh-vuQd-GXOaJjc8tDXjVxgDaqENSQbDH87YHqpISCrlgYvhfVTHJqWcwxG6fqwt9AZX9c7A/ids?startTime={}&endTime={}&count=100", datetime::local_day_bounds(2025, 8, 31).0, datetime::local_day_bounds(2025, 9, 01).1);

    let get_account_matches = riotmatchinfo::getriotaccountmatches(matches_api_endpoint_weekly).await;

    let matches_today_data = riotmatchinfo::getriotaccountmatches(matches_api_endpoint_daily).await;
    let get_total_matches_today = riotmatchinfo::getmatchestoday(&matches_today_data).await;
    
    let (matches_thisweek, timespent_thisweek, total_rankedmatches_thisweek, longest_game) = riotmatchinfo::getmatchinsightsweekly(&get_account_matches).await;

    emailingservice::emailservice(matches_thisweek, timespent_thisweek, total_rankedmatches_thisweek, longest_game).await;

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "QueueCap",
        options,
        Box::new(|_cc| Ok(Box::new(QueueCap{matches_thisweek, get_total_matches_today, timespent_thisweek, ..Default::default()}))),
    )
}


