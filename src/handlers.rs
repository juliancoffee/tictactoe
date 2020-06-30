use actix_web::{get, ws};

use crate::domain::Game;

#[get("/api/new_game/{id}/{wish}")]
pub async new_game<G: Game>() {
    //TODO: implement new_game handler
}

#[get("/api/play_game/{game_id}/{user_id}")]
pub async play_game<G: Game>() {
    //TODO: implement playing game
}
