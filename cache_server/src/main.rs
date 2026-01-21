use cache_protocol::SharedCache;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Duration, Instant};

use crate::commands::{handle_get, handle_has, handle_set, handle_stats};

mod commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache = Arc::new(SharedCache::new());

    // spawn background task
    spawn_janitor(Arc::clone(&cache));

    // start tcp server
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Cache Server running on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        let cache_ref = Arc::clone(&cache);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, cache_ref).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}

/// Background task to clean expired cache entries
fn spawn_janitor(cache: Arc<SharedCache>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let now = Instant::now();
            cache
                .store
                .retain(|_, entry| entry.expires_at.map_or(true, |expiry| now < expiry));
        }
    });
}

/// tcp connection handler
async fn handle_connection(
    mut socket: TcpStream,
    cache: Arc<SharedCache>,
) -> Result<(), std::io::Error> {
    let (reader, mut writer) = socket.split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        let response = match parts.as_slice() {
            ["GET", key] => handle_get(key, &cache),
            ["SET", key, val, ttl] => handle_set(key, val, Some(ttl), &cache),
            ["HAS", key] => handle_has(key, &cache),
            ["STATS"] => handle_stats(&cache),
            ["QUIT"] => break,
            _ => "ERROR: Unknown Command\n".to_string(),
        };

        writer.write_all(response.as_bytes()).await?;
    }
    Ok(())
}
