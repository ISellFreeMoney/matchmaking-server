use crate::redis_handler::{add_to_queue, find_opponent};
use tokio::time::{sleep, Duration};

pub async fn find_match(player_id:String, elo: u32) -> Option<String> {
    add_to_queue(player_id.clone(), elo).await;
    sleep(Duration::from_secs(3)).await;
    find_opponent(player_id, elo).await
}