// we have to download 100k urls
use reqwest::Client;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::task;
use tokio::task::spawn;
use tokio::time::sleep;
// use reqwest::

mod rust_pin;
mod send_sync;

use send_sync::run_send_sync;
// use rust_pin::run_pin;

// #[tokio::main]
// fn main() {
//     run_send_sync();
//
//
//     let tokio_runtime = Runtime::new();
//     // tokio_runtime.expect("hello").spawn_blocking(|_|{
//     //     get_website_info()
//     // });
//
//     // on way is using block_on
//     tokio_runtime.unwrap().block_on(async{
//         get_website_info().await
//     })
// }


#[tokio::main]
async fn main() {
    run_send_sync();


   let handler = task::spawn(async {get_website_info().await});

    handler.await.unwrap();
}

async fn get_website_info() {
    let urls = vec!["https://google.com"; 10000];

    let mut join_handlers = Vec::new();

    let client = reqwest::Client::new();

    let mut i = 0;
    for url in urls {
        let client = client.clone();
        i += 1;
        let join = task::spawn(async move { process_url(url, client, i).await });

        join_handlers.push(join);
    }

    for join_handler in join_handlers {
        join_handler.await.unwrap();
    }
}

async fn process_url(url: &str, client: Client, mut i: i32) {
    println!("{i} process url {:?}", url);
    sleep(Duration::from_millis(100));
    println!("{i} done");
    // let resp = client.get(url).send().await.unwrap();

    // println!("resp status {:?}", resp.status());
}
