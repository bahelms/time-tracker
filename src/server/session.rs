use chrono::{DateTime, Local, TimeZone, Utc};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

const CONFIG_DIRECTORY: &str = ".config/time_tracker";
const SESSION_HISTORY_FILENAME: &str = "history.txt";
const CURRENT_SESSION_FILENAME: &str = "current_session.txt";

#[derive(Debug)]
pub struct Session {
    name: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

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

    pub fn status(option: &str) -> Option<String> {
        match option {
            "--all" => history_status(),
            "" => current_session_status(),
            session_name => status_for_session(session_name),
        }
    }

    fn new(serialized_data: String) -> Self {
        let values: Vec<&str> = serialized_data.split('|').collect();
        Self {
            name: values[0].to_string(),
            start: parse_timestamp(values[1], Utc),
            end: parse_timestamp(values[2], Utc),
        }
    }
}

fn parse_timestamp<T: TimeZone>(timestamp: &str, tz: T) -> DateTime<T> {
    DateTime::parse_from_rfc3339(timestamp)
        .expect("Unable to parse datetime")
        .with_timezone(&tz)
}

fn history_status() -> Option<String> {
    Some("History status under construction".to_string())
}

fn status_for_session(session_name: &str) -> Option<String> {
    let mut sessions_count = 0;
    let mut total_minutes = 0;

    let file = File::open(history_file()).expect("Error opening history file");
    for line in BufReader::new(file).lines() {
        let session = Session::new(line.expect("Error reading session line"));
        if session.name == session_name {
            sessions_count += 1;
            let duration = session.end - session.start;
            total_minutes += duration.num_minutes();
        }
    }

    let hours = total_minutes / 60;
    let minutes = total_minutes.rem_euclid(60);
    Some(format!(
        "{} stats\n\t- Number of sessions: {}\n\t- Weekly average: {:.2} hours\n\t- Total Duration: {} hours, {} minutes",
        session_name, sessions_count, 0, hours, minutes
    ))
}

fn current_session_status() -> Option<String> {
    let path = current_session_path();
    if !Path::new(&path).exists() {
        return Some("There is no current session running.".to_string());
    }

    let contents = read_file_contents(&path);
    let contents: Vec<&str> = contents.split('|').collect();
    let dt = parse_timestamp(contents[1], Local);
    let formatted_dt = dt.format("%Y-%m-%d %k:%M:%S %p");
    let duration = Local::now() - dt;
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

fn history_file() -> String {
    format!("{}/{}", init_config_dir(), SESSION_HISTORY_FILENAME)
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
