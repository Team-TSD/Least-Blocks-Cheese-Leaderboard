use crate::common::{Country, CountryCode};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub leaderboard_query_delay: u64,
    pub run_query_delay: u64,
    pub retry_query_delay: u64,
    pub between_query_delay: u64,
    pub max_leaderboard_players: Option<usize>,
    pub output_path: String,
}

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| serde_json::from_str(include_str!("../config.json")).unwrap());

pub static COUNTRY_MAP: Lazy<HashMap<String, Country>> = Lazy::new(|| {
    let countries: Vec<Country> =
        serde_json::from_str(include_str!("../countryCodes.json")).unwrap();
    let country_codes: Vec<CountryCode> =
        serde_json::from_str(include_str!("../countryCodes.json")).unwrap();
    let mut country_map: HashMap<String, Country> = HashMap::new();
    for i in 0..countries.len() {
        country_map.insert(country_codes[i].alpha_2.clone(), countries[i].clone());
    }
    country_map
});
