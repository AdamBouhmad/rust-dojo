mod log_events;
use tokio::spawn;

#[tokio::main]
async fn main() {

    spawn(log_events::log_event("startup"));
    spawn(log_events::log_event("shutdown"));

    println!("Both tasks have been launched");
    
}
