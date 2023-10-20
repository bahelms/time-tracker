use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod session;
use session::Session;

pub fn start(location: String) {
    ctrlc::set_handler(move || {
        println!();
        Session::stop();
        println!("Bye bye!");
        std::process::exit(0);
    })
    .expect("Error setting ctrlc handler");

    let listener = TcpListener::bind(&location).expect("Failed to bind to addr");

    println!("Time Tracker is now awaiting your command!");

    loop {
        let (stream, _addr) = listener.accept().unwrap();
        thread::spawn(move || {
            handle_stream(stream);
        });
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("Reading message failed");

    if let Some(result) = execute_command(buffer.trim()) {
        let _ = stream
            .write(result.as_bytes())
            .expect("Failed to respond to client");
    }
}

fn execute_command(cmd: &str) -> Option<String> {
    match cmd.split(':').collect::<Vec<&str>>()[..] {
        ["START", session_id] => Session::start(session_id),
        ["STOP"] => Session::stop(),
        ["STATUS"] => Session::status(),
        _ => Some(format!("Unknown command {}", cmd)),
    }
}
