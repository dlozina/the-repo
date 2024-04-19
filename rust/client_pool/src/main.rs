use reqwest::blocking::Client;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let pool_size = 5;
    let client_pool = Arc::new(Mutex::new(VecDeque::with_capacity(pool_size)));

    // Populate the pool with client instances
    for _ in 0..pool_size {
        client_pool.lock().unwrap().push_back(Client::new());
    }

    let handles = (0..10)
        .map(|i| {
            if i > 5 {
                thread::sleep(std::time::Duration::from_secs(1));
            }
            let client_pool = client_pool.clone();

            thread::spawn(move || {
                // Acquire a client from the pool
                let client = {
                    let mut pool = client_pool.lock().unwrap();
                    pool.pop_back().unwrap_or_else(Client::new)
                };

                // Perform the request using the acquired client
                let url = format!("https://hssdev-ap-west.eng.macrometa.io/docs/hss");
                let resp = client.get(url).send();

                match resp {
                    Ok(response) => println!("Response {}: {:?}", i, response.status()),
                    Err(e) => println!("Request {} failed: {}", i, e),
                }

                // Return the client to the pool after the request is done
                {
                    let mut pool = client_pool.lock().unwrap();
                    pool.push_back(client);
                }
            })
        })
        .collect::<Vec<_>>();

    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }
}