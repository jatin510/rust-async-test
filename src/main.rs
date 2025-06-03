// we have to download 100k urls
use std::thread::available_parallelism;
use reqwest::Client;
use tokio::runtime::Builder;
use tokio::task;
// use reqwest::

#[tokio::main]
async fn main() {

    // let value = "https://google.com";
    // let count = 100;
    // let urls = std::iter::repeat(value).take(count).collect();
    let urls = vec!["https://google.com";10000];
    // println!("urls.length {:?}", urls.len());
    // println!("urls {}", urls.get(1).unwrap());

    // let urls = vec!["https://google.com";100];
    let mut join_handlers =Vec::new();

    let client = reqwest::Client::new();


    for url in urls {
        let client  =client.clone();
        let join = task::spawn(async  move{
            process_url(url, client.clone()).await
        });

      join_handlers.push(join);
    }

    for join_handler in join_handlers{
        join_handler.await.unwrap();
    }

}

async fn process_url(url:&str, client: Client) {
    println!("process url {:?}", url);
    let resp = client.get(url).send().await.unwrap();

    println!("resp status {:?}", resp);
}
