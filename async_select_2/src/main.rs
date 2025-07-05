use tokio::time::{sleep, Duration};
use tokio::select;
use tokio::spawn;

#[tokio::main]
async fn main() {

    let timeout = spawn(sleep(Duration::from_secs(3)));
    let cooking_timer = spawn(countdown("Cooking Timer", 5));

    tokio::select!(
        result = timeout => {println!("Timeout Exceeded")} ,
        result = cooking_timer => {println!("{}", result.unwrap())},
    )
}

async fn countdown(label: &str, seconds: u64) -> String {

    sleep(Duration::from_secs(seconds)).await;

    return format!("{label} countdown finished");

}
