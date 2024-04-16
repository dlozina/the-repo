use once_cell::sync::Lazy;
use reqwest::Client;
use std::sync::Arc;
use std::thread::sleep;
use std::time;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::Semaphore;

fn main() {
    let client = Arc::new(Client::new());
    let runtime = Arc::new(Runtime::new().unwrap());

    let handles = (0..10)
        .map(|i| {
            let client_clone = client.clone();
            let runtime_clone = runtime.clone();

            std::thread::spawn(move || {
                runtime_clone.block_on(async {
                    // Replace "http://example.com" with your desired URL
                    let url = format!("https://hssdev-ap-west.eng.macrometa.io/docs/hss");
                    let resp = client_clone.get(&url).send().await;

                    match resp {
                        Ok(response) => println!("Response {}: {:?}", i, response.status()),
                        Err(e) => println!("Request {} failed: {}", i, e),
                    }
                });
            })
        })
        .collect::<Vec<_>>();

// Wait for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }
}
