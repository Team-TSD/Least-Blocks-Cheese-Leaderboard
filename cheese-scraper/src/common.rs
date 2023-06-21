use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LeaderboardUser {
    pos: u32,
    id: u32,
    #[serde(deserialize_with="parse_name")]
    pub name: String,
    #[serde(rename = "game")]
    score: f32,
    ts: String,
}

fn parse_name<'de, D>(d: D) -> Result<String, D::Error> where D: serde::Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or("UNKNOWN".to_string())
        })
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CheeseRun {
    pub blocks: u32,
    pub date: String,
    pub replay: Option<String>, //replay link is sometimes purged or missing, its fine
    pub pps: f64,               //only used to sort during a block tie
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Country {
    pub name: String,
    pub region: String,
    #[serde(rename = "sub-region")]
    pub sub_region: String,
    #[serde(rename = "intermediate-region")]
    pub intermediate_region: String,
    #[serde(rename = "country-code")]
    pub country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryCode {
    #[serde(rename = "alpha-2")]
    pub alpha_2: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub pb: CheeseRun,
    pub country: Option<Country>,
}

impl PartialOrd for Player{
    fn partial_cmp(&self, other: &Self)->Option<core::cmp::Ordering>{
        Some(other.cmp(self))
    }
}
impl PartialEq for Player{
    fn eq(&self, other: &Self) -> bool{
        other.pb.pps == self.pb.pps && self.pb.blocks == other.pb.blocks
    }
}
impl Ord for Player{
    fn cmp(&self, other: &Self)->std::cmp::Ordering{
        if self.pb.blocks == other.pb.blocks {
            self.pb.pps.partial_cmp(&other.pb.pps).unwrap()
        } else {
            other.pb.blocks.cmp(&self.pb.blocks)
        }
    }
}
impl Eq for Player{}