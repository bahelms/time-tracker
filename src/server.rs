use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
pub async fn start(location: String) {
    let listener = TcpListener::bind(&location)
        .await
        .expect("Failed to bind to addr");

    println!("Time Tracker is running on {}", location);

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
        _ => println!("Unknown command {}", cmd),
    }
}

fn start_session(id: &str) {
    println!("Starting session for {}", id);

    // let time = Utc::now().to_rfc3339();
    // let parsed = chrono::DateTime::parse_from_rfc3339("2023-10-05T20:48:07.921875828+00:00");
    // match parsed {
    //     Ok(expr) => {
    //         println!("Tracking {} - {} - parsed {}", id, time, expr);
    //     }
    //     Err(e) => println!("Error {}", e),
    // }
}
