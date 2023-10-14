use std::io::{BufRead, Write};
use std::net::TcpStream;

pub fn start_session(session_id: String, server: String) {
    connect(server, |mut conn| {
        let command = format!("START:{}\n", session_id);
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

pub fn status(server: String) {
    connect(server, |mut conn| {
        send_command(&mut conn, "STATUS\n".to_string());
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
    let mut response = String::new();
    reader
        .read_line(&mut response)
        .expect("Reading message failed");
    println!("{}", response);
}
