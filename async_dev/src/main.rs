
mod log_events;

#[tokio::main]
async fn main(){

    log_events::log_event("shutdown").await;

}