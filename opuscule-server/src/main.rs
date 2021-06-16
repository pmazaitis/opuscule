// use std::io::BufRead;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

const ADDR: &'static str = "127.0.0.1:8080";

// set up a watch channel for commands up, and a broadcast channel for commands down?
// how to send to a specific channel?

#[tokio::main]
async fn main() {
    // Set up a TcpListener object
    let listener = TcpListener::bind(&ADDR).await.unwrap();

    // tokio broadcast channel; can we expose these?
    let (tx, _rx) = broadcast::channel(10);

    //We need to put accept into a loop for catching new clients
    loop {
        // call accept to accept new incoming connection for a stream and addr
        let (mut socket, addr) = listener.accept().await.unwrap();

        // We need to clone this so that each clent gets one
        let tx = tx.clone();
        // use this method to pull the receiver out of transmit (!?)
        let mut rx = tx.subscribe();

        // shunt off each new client to a new task (what ahppens if we do this witha funciton?)
        let _join_handle = tokio::spawn(async move {
            //handle read and write independantly
            let (reader, mut writer) = socket.split();

            // each client, respond to all messages
            //a place to put the data from the socket
            let mut reader = BufReader::new(reader);

            let mut line = String::new();

            loop {
                // let's use select to decide what to do - return id =  future => block of code
                // select awaits futures implicitly
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        // Test to see if stream has ended
                        if result.unwrap() == 0 {break;}
                        // send to all clients
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        // send out to this specific ui_client
                        let (msg, other_addr) = result.unwrap();
                        println!("Sending from {:?}", other_addr);
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                } // end tokio select
                  // pull in
                  //let bytes_read = reader.read_line(&mut line).await.unwrap();

                // write out
                // let msg = rx.recv().await.unwrap();
            } // end loop for echoing a specific client
        }); // end spawn
    } // end accept loop
}
