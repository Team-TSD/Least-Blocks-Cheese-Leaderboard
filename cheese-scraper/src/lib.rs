mod common;
mod config;
mod crawl;
mod html_parser;
mod query;

use std::collections::HashSet;

use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use tokio::time::{sleep, Duration};

pub use common::{CheeseRun, Player};

pub async fn update_player(name: &String) -> Result<String, anyhow::Error> {
    let runs = crawl::get_player_runs(name).await?;
    let (country, name) = crawl::get_player_info(&name).await?;
    let name = name.unwrap();
    let pb = runs.into_iter().min_by(|x, y| {
        if x.blocks == y.blocks {
            y.pps.partial_cmp(&x.pps).unwrap()
        } else {
            x.blocks.cmp(&y.blocks)
        }
    });
    let pb = match pb {
        Some(v) => v,
        None => return Ok("fail".to_string()),
    };
    let mut players: Vec<Player> =
        serde_json::from_str(&std::fs::read_to_string("../players.json").unwrap()).unwrap();
    if let Some(index) = players.iter().position(|value| value.name == name) {
        players.swap_remove(index);
    }
    players.push(Player {
        name,
        pb,
        country,
    });
    players.sort();
    std::fs::write(
        "../players.json",
        serde_json::to_string_pretty(&players).unwrap(),
    )
    .unwrap();
    Ok("success".to_string())
}

#[tokio::test]
async fn update_test() {
    update_player(&"FrEyHOE".to_string()).await.unwrap();
}


pub async fn get_names() -> HashSet<String> {
    println!("pulling player names from leaderboard");

    let leaderboard_bar =
        ProgressBar::new(config::CONFIG.max_leaderboard_players.unwrap_or(15000) as u64);
    leaderboard_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed}] [{bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .with_key(
            "eta",
            |state: &ProgressState, w: &mut dyn std::fmt::Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            },
        )
        .progress_chars("#>-"),
    );

    let names = crawl::get_player_names(Some(&leaderboard_bar))
        .await
        .unwrap();

    leaderboard_bar.finish();
    names
}

pub async fn pull_players(names: HashSet<String>) -> Vec<common::Player> {
    println!("Fetching runs for each player");

    let m = MultiProgress::new();
    let total_bar = m.add(ProgressBar::new(names.len() as u64));
    total_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed}] [{bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .with_key(
            "eta",
            |state: &ProgressState, w: &mut dyn std::fmt::Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            },
        )
        .progress_chars("#>-"),
    );
    let player_stats = m.add(ProgressBar::new(3));
    player_stats.set_style(
        ProgressStyle::with_template("{spinner} [{pos}/{len}] {wide_msg}")
            .unwrap()
            .tick_chars("←↖↑↗→↘↓↙"),
    );
    total_bar.inc(0);

    let mut players: Vec<common::Player> = Vec::new();
    for name in names {
        player_stats.set_message(format!("[{}]", name));
        player_stats.set_position(1);

        let runs = crawl::get_player_runs(&name).await.unwrap();
        let runs_len = runs.len();
        let pb = runs.into_iter().min_by(|x, y| {
            if x.blocks == y.blocks {
                y.pps.partial_cmp(&x.pps).unwrap()
            } else {
                x.blocks.cmp(&y.blocks)
            }
        });

        let pb: CheeseRun = match pb {
            Some(run) => run,
            None => continue,
        };

        player_stats.set_message(format!("[{}] [pb: {} runs: {}]", name, pb.blocks, runs_len));
        player_stats.inc(1);
        sleep(Duration::from_millis(config::CONFIG.between_query_delay)).await;

        let (country, _) = crawl::get_player_info(&name).await.unwrap();
        if let Some(country) = &country {
            player_stats.set_message(format!(
                "[{}] [pb: {} runs: {}] [country: {}]",
                name, pb.blocks, runs_len, country.name
            ));
        }
        player_stats.inc(1);
        sleep(Duration::from_millis(config::CONFIG.between_query_delay)).await;

        let player = common::Player { name, pb, country };
        players.push(player);

        total_bar.inc(1);
        std::fs::write(
            &config::CONFIG.output_path,
            serde_json::to_string_pretty(&players).unwrap(),
        )
        .unwrap();
        sleep(Duration::from_millis(config::CONFIG.between_query_delay)).await;
    }

    total_bar.finish();
    player_stats.finish();

    players.sort();
    players
}

#[tokio::test]
async fn sort() {
    let data = std::fs::read_to_string(&config::CONFIG.output_path).unwrap();
    let mut players: Vec<Player> = serde_json::from_str(&data).unwrap();
    players.sort();
    std::fs::write(
        "players.json",
        serde_json::to_string_pretty(&players).unwrap(),
    )
    .unwrap();
}
