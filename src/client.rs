use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
pub async fn start_session(session_id: String, server: String) {
    let mut stream = connect(server).await;
    let command = format!("START:{}", session_id);
    stream
        .write_all(command.as_bytes())
        .await
        .expect("Failed to write to server");
}

#[tokio::main]
pub async fn stop_session(server: String) {
    let mut stream = connect(server).await;
    stream
        .write_all("STOP".as_bytes())
        .await
        .expect("Failed to write to server");
}

async fn connect(server: String) -> TcpStream {
    TcpStream::connect(server)
        .await
        .expect("Failed to connect to server")
}
