use std::collections::HashMap;
use std::error::Error;
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    println!("hello from async world");
    greet().await;

    let handle1 = tokio::spawn(task_one());
    let handle2 = tokio::spawn(task_two());
    let handle3 = tokio::spawn(task_three());
    handle1.await;
    handle2.await;
    handle3.await;

    getAPIResponse().await;
}

async fn getAPIResponse() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}

async fn greet() {
    println!("Hello, how are you doin?");
    sleep(Duration::from_secs(1)).await;
    println!("Goodbye, hope you have a great day");
}

async fn task_one() {
    println!("task one has started");
    sleep(Duration::from_secs(1)).await;
    println!("task one has been executed after 1s");
}
async fn task_two() {
    println!("task two has started");
    sleep(Duration::from_secs(2)).await;
    println!("task two has been executed after 2s");
}
async fn task_three() {
    println!("task three has started");
    sleep(Duration::from_secs(3)).await;
    println!("task three has been executed after 3s");
}
/*
fn main() {
  tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      println!("hello from async world!");
    })
}
*/
