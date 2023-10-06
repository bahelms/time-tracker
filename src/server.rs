use chrono::Utc;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
pub async fn start(location: String) {
    let listener = TcpListener::bind(&location)
        .await
        .expect("Failed to bind to addr");

    println!("Time Tracker is now awaiting your command!");

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_stream(stream).await;
        });
    }
}

async fn handle_stream(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .await
        .expect("Reading message failed");
    execute_command(buffer);
}

fn execute_command(cmd: String) {
    match cmd.split(':').collect::<Vec<&str>>()[..] {
        ["START", session_id] => {
            start_session(session_id);
        }
        _ => {
            println!("Unknown command {}", cmd);
        }
    }
}

const SESSION_FILE: &str = "./current_session.txt";

fn start_session(id: &str) {
    if Path::new(SESSION_FILE).exists() {
        return println!("Can't start new session. A current session is still running.");
    }

    println!("Starting session for {}", id);
    let session = format!("{}|{}", id, Utc::now().to_rfc3339());
    store_session(session);
    // let parsed = chrono::DateTime::parse_from_rfc3339("2023-10-05T20:48:07.921875828+00:00");
    // match parsed {
    //     Ok(expr) => {
    //         println!("Tracking {} - {} - parsed {}", id, time, expr);
    //     }
    //     Err(e) => println!("Error {}", e),
    // }
}

fn store_session(session: String) {
    let mut file = File::create(SESSION_FILE).expect("Error creating file");
    // let current_session_id = contents.split('|').collect::<Vec<&str>>()[0];
    file.write_all(session.as_bytes())
        .expect("Error writing session to file");
}

fn stop_session() {
    let contents = read_file_contents(SESSION_FILE);
    let end_time = Utc::now().to_rfc3339();
}

fn read_file_contents(pathname: &str) -> String {
    let mut file = File::open(pathname).expect("Error opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading session file");
    contents
}
