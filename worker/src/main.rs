#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use redis::AsyncCommands;

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://redis/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();

    loop {
        let job: String = con.lpop("scan_queue", None).await.unwrap_or_default();

        if !job.is_empty() {
            println!("🔍 Scanning: {}", job);

            // aqui entra o scanner real
        }
    }
}
