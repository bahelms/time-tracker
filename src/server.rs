use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

mod session;
use session::Session;

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
        ["START", session_id] => Session::start(session_id),
        ["STOP"] => Session::stop(),
        _ => {
            println!("Unknown command {}", cmd);
        }
    }
}
