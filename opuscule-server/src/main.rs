// use std::io::BufRead;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

const ADDR: &'static str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    // Set up a TcpListener object
    let listener = TcpListener::bind(&ADDR).await.unwrap();

    //We need to put accept into a loop for catching new clients
    loop {
        // call accept to accept new incoming connection for a stream and addr
        let (mut socket, _addr) = listener.accept().await.unwrap();

        // shunt off each new client to a new task (what ahppens if we do this witha funciton?)
        tokio::spawn(async move {
            //handle read and write independantly
            let (reader, mut writer) = socket.split();

            // each client, respond to all messages
            //a place to put the data from the socket
            let mut reader = BufReader::new(reader);

            let mut line = String::new();

            loop {
                // pull in
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                // Test to see if stream has ended
                if bytes_read == 0 {
                    break;
                }
                // send out
                writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            } // end loop for echoing a specific client
        }); // end spawn
    } // end accept loop
}
