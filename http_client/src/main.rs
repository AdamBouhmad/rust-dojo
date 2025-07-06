use tokio::*;
use reqwest::*;
use serde::*;

#[derive(Debug, Deserialize)]
struct HTTPPayload {
    userId: u32,
    id: u32,
    title: String,
    body: String
}


#[tokio::main]
async fn main() {
    
    let url: &str = "https://jsonplaceholder.typicode.com/posts/1";

    //println!("{}", http_request(url).await.unwrap());

    let my_response = http_request(url).await;

    println!("Title Field: {} \n Body Field: {}", my_response.title, my_response.body);

}


async fn http_request(url: &str) -> HTTPPayload {

    let response = reqwest::get(url).await.unwrap();

    let parsed: HTTPPayload = response.json().await.unwrap();

    parsed
}