// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
// use tokio::sync::broadcast;

#[tokio::main]
pub async fn start(address: String, port: String) {
    let location = format!("{}:{}", address, port);
    let listener = TcpListener::bind(&location)
        .await
        .expect("Failed to bind to addr");

    println!("Time Tracker -- listening on {}", location);

    // let (tx, _) = broadcast::channel(32);
    loop {
        let (_stream, addr) = listener.accept().await.unwrap();
        println!("Connection accepted: {}", addr);
        // let publisher = tx.clone();
        // let consumer = tx.subscribe();
        // tokio::spawn(async move {
        //     handle_stream(stream, publisher, consumer, addr).await;
        // });
    }
}
