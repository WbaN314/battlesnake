#[macro_use]
extern crate rocket;

use battlesnake_game_of_chicken::{logic, OriginalGameState};
use log::{info, warn};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde_json::{json, Value};
use std::env;

#[get("/")]
fn handle_index() -> Json<Value> {
    Json(logic::info())
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<OriginalGameState>) -> Status {
    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(mut move_req: Json<OriginalGameState>) -> Json<Value> {
    // Log request
    let r = move_req.into_inner();
    warn!(
        "ID {} Turn {} -> {}",
        r.game.id,
        r.turn,
        serde_json::to_string(&r).unwrap()
    );

    let variant = env::var("VARIANT").unwrap_or(String::from("breadth_first"));

    move_req = Json(r);
    let response = logic::get_move(&move_req, variant);

    Json(json!({ "move": response }))
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<OriginalGameState>) -> Status {
    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    Status::Ok
}

#[launch]
fn rocket() -> _ {
    // Lots of web hosting services expect you to bind to the port specified by the `PORT`
    // environment variable. However, Rocket looks at the `ROCKET_PORT` environment variable.
    // If we find a value for `PORT`, we set `ROCKET_PORT` to that value.
    if let Ok(port) = env::var("PORT") {
        unsafe {
            env::set_var("ROCKET_PORT", &port);
        }
    }

    // We default to 'info' level logging. But if the `RUST_LOG` environment variable is set,
    // we keep that value instead.
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "warn");
        }
    }

    env_logger::init();

    info!("Starting Battlesnake Server...");

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "battlesnake/github/starter-snake-rust");
            })
        }))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
}
