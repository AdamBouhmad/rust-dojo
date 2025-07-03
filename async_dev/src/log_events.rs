use tokio::time::{sleep, Duration};

const sleep_timer: u64 = 3;

pub async fn log_event(status: &str) {

    sleep(Duration::from_secs(sleep_timer)).await;

    println!("Event completed: <{status}>");



}