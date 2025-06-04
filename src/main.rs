// we have to download 100k urls
use reqwest::Client;
use std::thread::available_parallelism;
use tokio::runtime::Builder;
use tokio::task;
// use reqwest::

mod send_sync;

use send_sync::run_send_sync;

#[tokio::main]
async fn main() {
    run_send_sync();

    // // let value = "https://google.com";
    // // let count = 100;
    // // let urls = std::iter::repeat(value).take(count).collect();
    // let urls = vec!["https://google.com"; 100];
    // // println!("urls.length {:?}", urls.len());
    // // println!("urls {}", urls.get(1).unwrap());
    //
    // // let urls = vec!["https://google.com";100];
    // let mut join_handlers = Vec::new();
    //
    // let client = reqwest::Client::new();
    //
    // for url in urls {
    //     let client = client.clone();
    //     let join = task::spawn(async move { process_url(url, client.clone()).await });
    //
    //     join_handlers.push(join);
    // }
    //
    // for join_handler in join_handlers {
    //     join_handler.await.unwrap();
    // }
}

async fn process_url(url: &str, client: Client) {
    println!("process url {:?}", url);
    let resp = client.get(url).send().await.unwrap();

    println!("resp status {:?}", resp);
}
