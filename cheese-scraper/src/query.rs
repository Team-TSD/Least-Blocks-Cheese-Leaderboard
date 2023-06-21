use crate::common::LeaderboardUser;

pub async fn query_run_page(name: &String, page: f32) -> Result<String, anyhow::Error> {
    let request_url =
        format!("https://jstris.jezevec10.com/cheese?display=5&user={name}&lines=100L&page={page}");
    let response = reqwest::get(request_url).await?;
    let text = response.text().await?;
    Ok(text)
}
pub async fn query_profile(name: &String) -> Result<String, anyhow::Error> {
    let request_url = format!("https://jstris.jezevec10.com/u/{name}");

    let response = reqwest::get(request_url).await?;
    let text = response.text().await?;
    Ok(text)
}

pub async fn query_leaderboard(t: usize) -> Result<Vec<LeaderboardUser>, anyhow::Error> {
    //api returns 500 players
    let request_url = format!(
        "https://jstris.jezevec10.com/api/leaderboard/3?mode=3&offset={offset}",
        offset = t
    );
    let response = reqwest::get(request_url).await?;

    let users: Vec<LeaderboardUser> = response.json().await?;
    Ok(users)
}
