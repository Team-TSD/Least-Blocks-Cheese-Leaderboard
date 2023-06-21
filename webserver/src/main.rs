use std::collections::{HashMap, HashSet};

use actix_files::Files;
use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder, HttpRequest};
use cheese_scraper::{update_player, Player};
#[get("/updatePlayer/{name}")]
async fn check_player(name: web::Path<String>) -> impl Responder {
    let val = update_player(&name).await;
    update_lb();
    match val {
        Ok(v) => format!("{}", v),
        Err(e) => format!("{}", e),
    }
}

#[get("/truePlayers")]
async fn get_true_players(req: HttpRequest) -> impl Responder {
    if let Some(val) = req.peer_addr() {
        let data = std::fs::read_to_string("whitelist.json").unwrap();
        let names : Vec<String>= serde_json::from_str(&data).unwrap();
        if !names.contains(&val.ip().to_string()){
            return "restricted".to_string();
        }
        return match std::fs::read_to_string("../players.json") {
            Err(_) => "error".to_string(),
            Ok(out) => out,
        }
    }
    "no ip".to_string()

}

#[get("/update")]
async fn update() -> impl Responder {
    update_lb();
    "ok".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    update_lb();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            // Enable the logger.
            .wrap(Logger::default())
            .service(check_player)
            .service(get_true_players)
            .service(update)
            .service(Files::new("/flags", "./static/flags"))
            .service(Files::new("/players", "./static/").index_file("players.json"))
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

type AltDict = HashMap<String, Vec<String>>;

fn update_lb(){
    let data = std::fs::read_to_string("../players.json").unwrap();
    let mut players : Vec<Player> = serde_json::from_str(&data).unwrap();

    let data = std::fs::read_to_string("alts.json").unwrap();
    let alt_dict : AltDict = serde_json::from_str(&data).unwrap();

    let data = std::fs::read_to_string("banned.json").unwrap();
    let banned : HashSet<String> = serde_json::from_str(&data).unwrap();

    let mut new_players : Vec<Player> = Vec::new();
    
    for (name, alts) in alt_dict{
        let (collected, leftover) : (Vec<Player>,Vec<Player>)= players.into_iter().partition(|p| alts.contains(&p.name) || p.name == name);
        if let Some(mut p) = collected.into_iter().max(){
            p.name = name;
            new_players.push(p);
        }
        players = leftover;
    }
    for player in players{
        if !banned.contains(&player.name) {
            new_players.push(player);
        }
    }
    new_players.sort();
    new_players.truncate(500);
    std::fs::write(
        "./static/players.json",
        serde_json::to_string_pretty(&new_players).unwrap()
    ).unwrap();
}