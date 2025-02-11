use actix_web::{post, web, HttpResponse, Responder};
use actix_web::body::MessageBody;
use serde::{Deserialize, };

use crate::matchmaking::find_match;

#[derive(Deserialize)]
struct MatchRequest {
    player_id: String,
    elo: u32,
}

#[post("/queue")]
async fn queue_player(req: web::Json<MatchRequest>) -> impl Responder {
    let result = find_match(req.player_id.clone(), req.elo).await;
    match result {
        Some(opponent) => HttpResponse::Ok().json(opponent),
        None => HttpResponse::Accepted().body("En attente d'un match..."),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(queue_player);
}