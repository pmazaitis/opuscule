use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

const ADDR: &'static str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    // Set up a TcpListener object
    let listener = TcpListener::bind(&ADDR).await.unwrap();
    // call accept to accept new incoming connection for a stream and addr
    let (mut socket, _addr) = listener.accept().await.unwrap();

    loop {
        // each client, respond to all messages
        //a place to put the data from the socket
        let mut buffer = [0u8; 1024];
        // pull in
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        // send out
        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}
