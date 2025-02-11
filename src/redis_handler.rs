use redis::AsyncCommands;
const  REDIS_URL: &str = "redis://127.0.0.1/";

pub async fn add_to_queue(player_id: String, elo:u32){
    let client = redis::Client::open(REDIS_URL).unwrap();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    let _: () = con.zadd("matchmaking_queue", player_id, elo).await.unwrap();
}

pub async fn find_opponent(player_id: String, elo:u32) -> Option<String> {
    let client = redis::Client::open(REDIS_URL).unwrap();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();

    let players: Vec<(String, u32)> = con.zrangebyscore_limit("matchmaking_queue", elo - 100, elo + 100, 1, 1).await.unwrap();

    if let Some((opponent, _)) = players.into_iter().find(|(id, _)|id != &player_id){
        return Some(opponent);
    }
    None
}