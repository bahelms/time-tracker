use clap::{Args, Parser};
use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::str;

/// Track the time you spend on tasks!
#[derive(Parser, Debug)]
#[command(name = "time_tracker")]
pub enum TimeTrackerCLI {
    /// Start the time tracking server in the foreground
    Server,
    Start(StartParams),
    Status(StatusParams),
    /// Stop the current session
    Stop,
}

/// Start a session
#[derive(Args, Debug)]
pub struct StartParams {
    /// The name of the target session
    pub session_name: String,
}

/// Display the statuses of sessions
#[derive(Args, Debug)]
// #[command(author, version)]
pub struct StatusParams {
    /// Display the status of all sessions
    #[arg(long)]
    all: bool,

    /// The name of the target session
    session_name: Option<String>,
}

pub fn start_session(server: String, session_name: String) {
    connect(server, |mut conn| {
        let command = format!("START:{}\n", session_name);
        send_command(&mut conn, command);
        read_response(conn);
    })
}

pub fn stop_session(server: String) {
    connect(server, |mut conn| {
        send_command(&mut conn, "STOP\n".to_string());
        read_response(conn);
    })
}

pub fn status(server: String, params: StatusParams) {
    let option = if params.all {
        "--all".to_string()
    } else if let Some(name) = params.session_name {
        name
    } else {
        String::new()
    };

    connect(server, |mut conn| {
        send_command(&mut conn, format!("STATUS:{}\n", option));
        read_response(conn);
    })
}

fn connect<F>(server: String, callback: F)
where
    F: FnOnce(TcpStream),
{
    match TcpStream::connect(server) {
        Ok(conn) => callback(conn),
        Err(err) => println!("Error: failed to connect to server - {}", err),
    }
}

fn send_command(stream: &mut TcpStream, command: String) {
    stream
        .write_all(command.as_bytes())
        .expect("Failed to write to server");
}

fn read_response(stream: TcpStream) {
    let mut reader = std::io::BufReader::new(stream);
    let mut response = Vec::new();
    reader
        .read_until(0, &mut response)
        .expect("Reading message failed");
    println!(
        "{}",
        str::from_utf8(&response).expect("Error: failed to parse response")
    );
}
