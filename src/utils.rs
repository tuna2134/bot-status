use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Ratelimit {
    first_request_time: u64,
    request_count: u64,
}

pub async fn check_ratelimit(
    ratelimit: Arc<Mutex<HashMap<String, Ratelimit>>>,
    token: String,
    per_time: u64,
    per_count: u64,
) -> anyhow::Result<bool> {
    let mut ratelimit = ratelimit.lock().await;
    let now_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    match ratelimit.get_mut(&token) {
        Some(data) => {
            if data.request_count > per_count {
                if now_time - data.first_request_time < per_time {
                    Ok(true)
                } else {
                    data.request_count = 0;
                    Ok(false)
                }
            } else {
                data.request_count += 1;
                Ok(false)
            }
        }
        None => {
            ratelimit.insert(
                token,
                Ratelimit {
                    first_request_time: now_time,
                    request_count: 1,
                },
            );
            Ok(false)
        }
    }
}
