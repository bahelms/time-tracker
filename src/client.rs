use std::io::{BufRead, Write};
use std::net::TcpStream;

pub fn start_session(session_id: String, server: String) {
    let mut stream = connect(server);
    let command = format!("START:{}\n", session_id);
    send_command(&mut stream, command);
    read_response(stream);
}

pub fn stop_session(server: String) {
    let mut stream = connect(server);
    send_command(&mut stream, "STOP\n".to_string());
    read_response(stream);
}

pub fn status(server: String) {
    let mut stream = connect(server);
    send_command(&mut stream, "STATUS\n".to_string());
    read_response(stream);
}

fn connect(server: String) -> TcpStream {
    TcpStream::connect(server).expect("Failed to connect to server")
}

fn send_command(stream: &mut TcpStream, command: String) {
    stream
        .write_all(command.as_bytes())
        .expect("Failed to write to server");
}

fn read_response(stream: TcpStream) {
    let mut reader = std::io::BufReader::new(stream);
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .expect("Reading message failed");
    println!("{}", response);
}
