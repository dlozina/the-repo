use once_cell::sync::Lazy;
use reqwest::Client;
use std::sync::Arc;
use std::thread::sleep;
use std::time;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::Semaphore;

// Lazy initialization of the Reqwest client
static CLIENT: Lazy<Arc<Client>> = Lazy::new(|| {
    Arc::new(Client::new())
});

fn main() {
    //Create a multi-threaded runtime
    let runtime = Builder::new_multi_thread()
        .worker_threads(4) // Specify the number of threads
        .enable_all()      // Enable all runtime features
        .build()
        .unwrap();

    // Create a semaphore with 4 permits
    let semaphore = Arc::new(Semaphore::new(4));

    // Collect all the handles from spawned tasks
    let mut handles = Vec::new();

    for i in 0..10 {
        let client_clone = CLIENT.clone();
        let semaphore_clone = semaphore.clone();

        // Spawn tasks into the runtime
        let handle = runtime.spawn(async move {
            // Acquire a permit
            let permit = semaphore_clone.acquire().await.expect("Failed to acquire semaphore permit");

            // Perform the HTTP request
            let url = format!("https://hssdev-ap-west.eng.macrometa.io/docs/hss");
            let resp = client_clone.get(&url).send().await;

            match resp {
                Ok(response) => println!("Response {}: {:?}", i, response.status()),
                Err(e) => println!("Request {} failed: {}", i, e),
            }

            // When the permit goes out of scope, it will be automatically released
            drop(permit);
            let dur = time::Duration::from_millis(150);
            sleep(dur)
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    // Because we're now waiting on futures from the runtime, we don't need to explicitly join handles
    // We should instead ensure all tasks complete before exiting the main function
    for handle in handles {
        let _ = runtime.block_on(handle);
    }
}