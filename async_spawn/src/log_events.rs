use tokio::time::{sleep, Duration};

pub async fn log_event(status: &str) {

    let mut sleep_timer = 2;

    if status == "startup" {
        let sleep_timer = 5;
    }

    sleep(Duration::from_secs(sleep_timer)).await;

    println!("Event completed: <{status}>")

}