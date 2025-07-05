use tokio::time::{sleep, Duration};

pub async fn log_event(status: &str) -> &str {

    let mut sleep_timer = 2;

    if status == "startup" {
        sleep_timer = 5;
    }

    println!("{sleep_timer}");
    sleep(Duration::from_secs(sleep_timer)).await;

    println!("Event completed: <{status}>");

    return status;

}