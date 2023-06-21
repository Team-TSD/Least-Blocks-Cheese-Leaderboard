use cheese_scraper::{get_names, pull_players};
#[tokio::main]
async fn main() {
    let names = get_names().await;
    let players = pull_players(names).await;

    std::fs::write(
        "../players.json",
        serde_json::to_string_pretty(&players).unwrap(),
    )
    .unwrap();
}
