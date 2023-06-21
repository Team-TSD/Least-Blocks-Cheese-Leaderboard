use core::fmt;
use std::fs::read_to_string;
use cheese_scraper::{CheeseRun, Player};
use webhook::client::WebhookClient;

#[derive(Debug)]
pub struct Change {
    old: Option<CheeseRun>,
    new: CheeseRun,
    name: String,
    old_place: Option<u32>,
    new_place: u32,
}
#[derive(Debug)]
pub struct Drop {
    old: CheeseRun,
    old_place: u32,
    name: String,
}

impl fmt::Display for Drop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "-{0: >3} ▼     [{2: <3}]       {1: <20}",
            self.old_place, self.name, self.old.blocks
        )
    }
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut old_blocks = "".to_string();
        let mut old_place = "".to_string();
        if let Some(b) = &self.old_place {
            old_place = (b+1).to_string();
        }
        if let Some(run) = &self.old {
            old_blocks = run.blocks.to_string();
        }
        write!(
            f,
            " {0: >3} ▲ {1: <3} [{3: >3} ▲ {4: <3}] {2}",
            old_place, self.new_place+1, self.name, old_blocks, self.new.blocks
        )
    }
}

pub fn format_changes(v: &Vec<Change>) -> String {
    let mut out = "```css\nImprovements\n".to_owned();
    for slice in v.iter() {
        if let Some(old) = &slice.old{
            if old.blocks == slice.new.blocks{
                continue;
            }
        }
        out.push_str(&(slice.to_string() + "\n"));
    }
    out.push_str("```");
    out.to_string()
}

pub fn format_drops(v: &Vec<Drop>) -> String {
    let mut out = "```diff\nDrops\n".to_owned();
    for slice in v.iter() {
        out.push_str(&format!("{}\n", slice.to_string()));
    }
    out.push_str("```");
    out.to_string()
}

pub fn compare() -> (Vec<Change>, Vec<Drop>) {
    let old_players: Vec<Player> =
        serde_json::from_str(&read_to_string("../old_players.json").unwrap()).unwrap();
    let new_players: Vec<Player> =
        serde_json::from_str(&read_to_string("../players.json").unwrap()).unwrap();

    let mut changes: Vec<Change> = Vec::new();
    let mut pbs: Vec<Change> = Vec::new();
    let mut drops: Vec<Drop> = Vec::new();

    for (i, player) in new_players.iter().enumerate() {
        let mut found: Option<(&Player, u32)> = None;
        for (i, p) in old_players.iter().enumerate() {
            if player.name == p.name {
                found = Some((p, i as u32));
                break;
            }
        }
        let change = match found {
            Some(p) => {
                if p.1 <= i as u32 {
                    continue;
                }
                Change {
                    old: Some(p.0.pb.clone()),
                    new: player.pb.clone(),
                    name: player.name.clone(),
                    old_place: Some(p.1),
                    new_place: i as u32,
                }
            }
            None => Change {
                old: None,
                new: player.pb.clone(),
                name: player.name.clone(),
                old_place: None,
                new_place: i as u32,
            },
        };
        if let Some(p) = found {
            pbs.push(Change {
                old: Some(p.0.pb.clone()),
                new: player.pb.clone(),
                name: player.name.clone(),
                old_place: Some(p.0.pb.blocks),
                new_place: player.pb.blocks,
            })
        }
        if let Some(old_place) = &change.old_place {
            if *old_place <= i as u32 {
                continue;
            }
        }
        changes.push(change);
    }

    for (i, player) in old_players.iter().enumerate() {
        let mut found: bool = false;
        for p in new_players.iter() {
            if player.name == p.name {
                found = true;
                break;
            }
        }
        if !found {
            drops.push(Drop {
                old: player.pb.clone(),
                old_place: i as u32,
                name: player.name.clone(),
            })
        }
    }
    (changes, drops)
}
#[tokio::main]
async fn main() {
    let (changes, drops) = compare();
    let hooks: Vec<String> = serde_json::from_str(&read_to_string("webhooks.json").unwrap()).unwrap();
    let out = format!("{}\n{}", format_changes(&changes), format_drops(&drops));
    for hook in hooks {
        let client: WebhookClient = WebhookClient::new(&hook);
        client
            .send(|message| {
                message
                    .username("cheese changes")
                    .avatar_url("https://avatars.githubusercontent.com/u/79282422?s=200&v=4")
                    .content(&out)
            })
            .await
            .unwrap();
    }
}

#[test]
fn try_comp() {
    let (changes, drops) = compare();
    std::fs::write(
        "output.txt",
        format!("{}\n{}", format_changes(&changes), format_drops(&drops)),
    )
    .unwrap();
}

#[tokio::test]
async fn send_init() {
    let hooks: Vec<String> = serde_json::from_str(&read_to_string("webhooks.json").unwrap()).unwrap();
    let out = format!("starting pull, expected to finish at <t:1676878384>");
    println!("{}", out);
    for hook in hooks {
        let client: WebhookClient = WebhookClient::new(&hook);
        client
            .send(|message| {
                message
                    .username("cheese changes")
                    .avatar_url("https://i.imgur.com/4fQ99rT.png")
                    .content(&out)
            })
            .await
            .unwrap();
    }
}


#[tokio::test]
async fn send_output() {
    let hooks: Vec<String> = serde_json::from_str(&read_to_string("webhooks.json").unwrap()).unwrap();
    let out = include_str!("../output.txt");
    for hook in hooks {
        let client: WebhookClient = WebhookClient::new(&hook);
        client
            .send(|message| {
                message
                    .username("cheese changes")
                    .avatar_url("https://avatars.githubusercontent.com/u/79282422?s=200&v=4")
                    .content(&out)
            })
            .await
            .unwrap();
    }
}
