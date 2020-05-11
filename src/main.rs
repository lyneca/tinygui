use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::env;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{Value, from_str};

use tinygui::GUI;
use tinygui::views::{Menu,MenuItem};

const URL: &str = "https://slack.com/api/users.profile.set";

#[derive(Deserialize)]
struct JSONEntry {
    text: String,
    emoji: String
}

fn main() {
    let mut gui = GUI::new();
    let mut slack_status_menu = Menu::new();
    
    let mut file = File::open("statuses.json").expect("Could not find statuses.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read statuses.json");
    let entries: Vec<JSONEntry> = from_str(contents.as_str()).expect("Could not parse statuses.json");
    for entry in entries {
        slack_status_menu.add_entry(MenuItem::TextToFunc(entry.text.clone(), Box::new(move || {
            send_status(entry.emoji.clone(), entry.text.clone())
        })));
    }
    gui.renderer.add_view(Box::new(slack_status_menu));
    gui.run();
}

fn send_status(icon: String, text: String) {
    let client = Client::new();
    let mut payload = HashMap::new();
    let mut profile = HashMap::new();
    profile.insert("status_text", text);
    profile.insert("status_emoji", icon);
    profile.insert("status_expiration", "0".to_owned());
    payload.insert("profile", profile);
    if let Ok(token) = env::var("SLACK_TOKEN") {
        let res = match client.post(URL)
            .json(&payload)
            .header("Authorization", format!("Bearer {}", token))
            .send() {
                Ok(res) => Some(res),
                _ => None
            };
        match res {
            Some(response) => match response.text() {
                Ok(t) => println!("{}", t),
                _ => {}
            },
            None => {}
        }
    } else {
        println!("Couldn't set status");
    }
}
