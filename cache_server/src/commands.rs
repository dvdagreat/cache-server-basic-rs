use cache_protocol::SharedCache;
use tokio::time::Duration;

pub fn handle_get(key: &str, cache: &SharedCache) -> String {
    match cache.get(key) {
        Some(val) => format!("VALUE: {}\n", String::from_utf8_lossy(&val)),
        None => "NIL\n".to_string(),
    }
}

pub fn handle_set(key: &str, val: &str, ttl: Option<&str>, cache: &SharedCache) -> String {
    // Setting a default 60s TTL if not provided in the command
    // println!("{:?}",);
    let mut final_ttl = match ttl {
        None => 60,
        Some(ttl) => ttl.parse::<u64>().unwrap_or(60),
    };

    if final_ttl > 100 {
        final_ttl = 60;
    }

    cache.set(
        key.to_string(),
        val.as_bytes().to_vec(),
        Some(Duration::from_secs(final_ttl)),
    );
    "OK\n".to_string()
}

pub fn handle_has(key: &str, cache: &SharedCache) -> String {
    if cache.has(key) {
        "TRUE\n".to_string()
    } else {
        "FALSE\n".to_string()
    }
}

pub fn handle_stats(cache: &SharedCache) -> String {
    let stats = cache.stats();
    format!(
        "ITEMS: {}\nBYTES: {}\n",
        stats.item_count, stats.total_bytes
    )
}
