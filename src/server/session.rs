use chrono::{DateTime, Utc};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const CONFIG_DIRECTORY: &str = ".config/time_tracker";
const SESSION_HISTORY_FILENAME: &str = "history.txt";
const CURRENT_SESSION_FILENAME: &str = "current_session.txt";

pub struct Session;

impl Session {
    pub fn start(id: &str) -> Option<String> {
        let path = current_session_path();
        if Path::new(&path).exists() {
            return Some(
                "Can't start new session. A current session is still running.".to_string(),
            );
        }

        let session = format!("{}|{}", id, Utc::now().to_rfc3339());
        store_current_session(session, path);
        Some(format!("Starting session for {}", id))
    }

    pub fn stop() -> Option<String> {
        let path = current_session_path();
        if !Path::new(&path).exists() {
            return Some("There is no current session running.".to_string());
        }

        let contents = read_file_contents(&path);
        let closed_session = format!("{}|{}\n", contents, Utc::now().to_rfc3339());
        persist_session_to_history(closed_session, path);
        Some("Stopping current session".to_string())
    }

    pub fn status() -> Option<String> {
        let path = current_session_path();
        if !Path::new(&path).exists() {
            return Some("There is no current session running.".to_string());
        }

        let contents = read_file_contents(&path);
        let contents: Vec<&str> = contents.split('|').collect();
        let dt = DateTime::parse_from_rfc3339(contents[1])
            .expect("Parsing timestamp failed")
            .with_timezone(&chrono::Local);
        let formatted_dt = dt.format("%Y-%m-%d %k:%M:%S %p");
        let duration = chrono::Local::now() - dt;
        let formatted_duration = format!(
            "{} hours, {} minutes",
            duration.num_hours(),
            duration.num_minutes()
        );

        Some(format!(
            "Current session: {}\nStarted: {}\nDuration: {}",
            contents[0], formatted_dt, formatted_duration
        ))
    }
}

fn current_session_path() -> String {
    format!("{}/{}", init_config_dir(), CURRENT_SESSION_FILENAME)
}

fn store_current_session(session: String, path: String) {
    let mut file = File::create(path).expect("Error creating file");
    file.write_all(session.as_bytes())
        .expect("Error writing session to file");
}

fn read_file_contents(pathname: &str) -> String {
    let mut file = File::open(pathname).expect("Error opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading session file");
    contents
}

fn persist_session_to_history(session: String, path: String) {
    let mut file = open_history_file();
    file.write_all(session.as_bytes())
        .expect("Error saving closed session");
    fs::remove_file(path).expect("Error removing current session file");
}

fn open_history_file() -> File {
    let history_file = format!("{}/{}", init_config_dir(), SESSION_HISTORY_FILENAME);

    if Path::new(&history_file).exists() {
        OpenOptions::new()
            .append(true)
            .open(history_file)
            .expect("Error opening history file")
    } else {
        File::create(Path::new(&history_file)).expect("Error creating file")
    }
}

fn init_config_dir() -> String {
    let config_dir = format!(
        "{}/{}",
        std::env::var("HOME").expect("Error getting HOME env var"),
        CONFIG_DIRECTORY,
    );
    if !Path::new(&config_dir).exists() {
        fs::create_dir_all(&config_dir).expect("Error creating config directory");
    }
    config_dir
}
