use crate::{
    common::{CheeseRun, Country},
    config::CONFIG,
    html_parser::{parse_profile, parse_run_page},
    query::{query_leaderboard, query_profile, query_run_page},
};
use indicatif::ProgressBar;
use std::collections::hash_set::HashSet;
use tokio::time::{sleep, Duration};
use tokio_retry::{strategy::FixedInterval, Retry};

pub async fn get_player_names(bar: Option<&ProgressBar>) -> Result<HashSet<String>, anyhow::Error> {
    let mut t: usize = 0;
    let mut player_names = HashSet::new();
    'outer: loop {
        let leaderboard_users = Retry::spawn(
            FixedInterval::from_millis(CONFIG.retry_query_delay).take(10),
            || query_leaderboard(t),
        )
        .await?;
        if leaderboard_users.len() == 0 {
            break;
        }
        t += leaderboard_users.len();
        if let Some(bar) = bar {
            bar.inc(leaderboard_users.len() as u64);
        }
        for user in leaderboard_users {
            player_names.insert(user.name);
            if let Some(max) = CONFIG.max_leaderboard_players {
                if player_names.len() >= max {
                    break 'outer;
                }
            }
        }
        sleep(Duration::from_millis(CONFIG.leaderboard_query_delay)).await;
    }
    Ok(player_names)
}

pub async fn get_player_runs(name: &String) -> Result<Vec<CheeseRun>, anyhow::Error> {
    let mut runs = Vec::new();
    let mut page = 0.0;
    loop {
        let document = Retry::spawn(
            FixedInterval::from_millis(CONFIG.retry_query_delay).take(10),
            || query_run_page(name, page),
        )
        .await?;
        let (new_runs, new_page) = parse_run_page(document)?;
        runs.extend(new_runs);
        match new_page {
            Some(num) => page = num,
            None => break,
        }
        sleep(Duration::from_millis(CONFIG.run_query_delay)).await;
    }
    Ok(runs)
}


#[tokio::test]
async fn info_test() {
    let (_, name) = get_player_info(&"EqualTurtle".to_string()).await.unwrap();
    println!("{:?}",name);
}

pub async fn get_player_info(name: &String) -> Result<(Option<Country>, Option<String>), anyhow::Error> {
    let document = Retry::spawn(
        FixedInterval::from_millis(CONFIG.retry_query_delay).take(10),
        || query_profile(name),
    )
    .await?;
    Ok(parse_profile(document)?)
}

#[tokio::test]
async fn runs() {
    let runs = get_player_runs(&"freyhoe".to_string()).await.unwrap();
    println!("{:#?}", runs);
}
