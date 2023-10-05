use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
pub async fn start_session(session_id: String, server: String) {
    let mut stream = TcpStream::connect(server)
        .await
        .expect("Failed to connect to server");

    let command = format!("START:{}", session_id);
    stream
        .write_all(command.as_bytes())
        .await
        .expect("Failed to write to server");
}
