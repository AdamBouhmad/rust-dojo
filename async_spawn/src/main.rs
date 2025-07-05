mod log_events;
use tokio::spawn;
use tokio::join;

#[tokio::main]
async fn main() {

    let startup_event = spawn(log_events::log_event("startup"));
    let shutdown_event = spawn(log_events::log_event("shutdown"));

    println!("Both tasks have been launched");

    //awaiting one by one, no concurrency 
        //startup_event.await.unwrap();
        //shutdown_event.await.unwrap();

    //concurrent awaiting both handles
    let _ = join!(startup_event, shutdown_event);
    
}
