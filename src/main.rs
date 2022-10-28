use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;
use rascam::{info, SimpleCamera};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::from_str;

use tinygui::view::{View, ViewSpawner};
use tinygui::views::{BoidsViewBuilder, Menu, MenuItem, TextInput};
use tinygui::GUI;

const URL: &str = "https://slack.com/api/users.profile.set";

#[derive(Deserialize, Clone)]
struct JSONEntry {
    text: String,
    emoji: String,
}

struct SlackMenuSpawner {
    entries: Vec<JSONEntry>,
}

impl ViewSpawner for SlackMenuSpawner {
    fn spawn(&self) -> Box<dyn View> {
        let mut menu = Box::new(Menu::new());
        for entry in self.entries.clone() {
            menu.add_entry(MenuItem::TextToFunc(
                entry.text.clone(),
                Box::new(move || send_status(entry.emoji.clone(), entry.text.clone())),
            ));
        }
        menu
    }
}

fn take_photo() {
    let info = info().unwrap();
    if info.cameras.len() < 1 {
        return;
    }
    let mut camera = SimpleCamera::new(info.cameras[0].clone()).unwrap();
    camera.activate().unwrap();

    thread::sleep(Duration::from_millis(2000));

    let photo = camera.take_one().unwrap();
    let time: DateTime<Local> = SystemTime::now().into();
    std::fs::create_dir_all("pics");
    File::create(Path::new(
        format!("pics/{}.jpg", time.format("%d-%m-%Y_%T")).as_str(),
    ))
    .unwrap()
    .write_all(&photo)
    .unwrap();
}

fn main() {
    // New GUI
    let mut gui = GUI::new();

    // Slack statuses list
    let mut file = File::open("statuses.json").expect("Could not find statuses.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read statuses.json");
    let slack_status_menu = SlackMenuSpawner {
        entries: from_str(contents.as_str()).expect("Could not parse statuses.json"),
    };

    // Main menu
    let mut main_menu = Menu::new();

    // Boids!
    let boids_builder = BoidsViewBuilder {};

    let text_spawner = TextInput::spawner();

    main_menu.add_entry(MenuItem::TextToView(
        "Set Slack Status".to_owned(),
        Box::new(slack_status_menu),
    ));
    main_menu.add_entry(MenuItem::TextToFunc(
        "Take Photo".to_owned(),
        Box::new(take_photo),
    ));
    main_menu.add_entry(MenuItem::TextToView(
        "Boids".to_owned(),
        Box::new(boids_builder),
    ));
    main_menu.add_entry(MenuItem::TextToView(
        "Test Keyboard".to_owned(),
        Box::new(text_spawner)));

    gui.renderer.push_view(Box::new(main_menu));
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
        let res = match client
            .post(URL)
            .json(&payload)
            .header("Authorization", format!("Bearer {}", token))
            .send()
        {
            Ok(res) => Some(res),
            _ => None,
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