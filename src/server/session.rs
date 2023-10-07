use chrono::Utc;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const CURRENT_SESSION_FILE: &str = "./current_session.txt";

pub struct Session {
    // current_session_file: String,
}

impl Session {
    pub fn start(id: &str) {
        if Path::new(CURRENT_SESSION_FILE).exists() {
            return println!("Can't start new session. A current session is still running.");
        }

        println!("Starting session for {}", id);
        let session = format!("{}|{}", id, Utc::now().to_rfc3339());
        store_current_session(session);
    }

    pub fn stop() {
        if !Path::new(CURRENT_SESSION_FILE).exists() {
            return println!("There is no current session running.");
        }

        println!("Stopping current session");
        let contents = read_file_contents(CURRENT_SESSION_FILE);
        let closed_session = format!("{}|{}\n", contents, Utc::now().to_rfc3339());
        persist_session_to_history(closed_session);
    }
}

fn store_current_session(session: String) {
    let mut file = File::create(CURRENT_SESSION_FILE).expect("Error creating file");
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

fn persist_session_to_history(session: String) {
    let mut file = open_history_file();
    file.write_all(session.as_bytes())
        .expect("Error saving closed session");
    fs::remove_file(CURRENT_SESSION_FILE).expect("Error removing current session file");
}

fn open_history_file() -> File {
    let history_file = format!(
        "{}/.config/time_tracker_history.txt",
        std::env::var("HOME").expect("Error getting HOME env var")
    );

    if Path::new(&history_file).exists() {
        OpenOptions::new()
            .append(true)
            .open(history_file)
            .expect("Error opening history file")
    } else {
        File::create(Path::new(&history_file)).expect("Error creating file")
    }
}

// let current_session_id = contents.split('|').collect::<Vec<&str>>()[0];
//
// let parsed = chrono::DateTime::parse_from_rfc3339("2023-10-05T20:48:07.921875828+00:00");
// match parsed {
//     Ok(expr) => {
//         println!("Tracking {} - {} - parsed {}", id, time, expr);
//     }
//     Err(e) => println!("Error {}", e),
// }
