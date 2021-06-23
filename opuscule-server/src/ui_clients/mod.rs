// #![allow(dead_code)]
// #![allow(unused)]

use std::net;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::mpsc,
    sync::watch,
};

use mpsc::Sender;
use watch::Receiver;

pub async fn handle_ui_clients(
    addr: String,
    send_command: Sender<String>,
    recd_state: Receiver<String>,
) -> ! {
    // Set up a TcpListener object
    let listener = TcpListener::bind(&addr).await.unwrap();

    // We need to loop to catch each new client as it comes in
    loop {
        let (mut ui_socket, _addr) = listener.accept().await.unwrap();

        // clone the channels for commands and state
        let send_command = send_command.clone();
        let mut recd_state = recd_state.clone();

        let _ui_client_handle = tokio::spawn(async move {
            //handle read and write independantly
            let (ui_client_reader, mut ui_client_writer) = ui_socket.split();

            let mut reader = BufReader::new(ui_client_reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        // Test to see if stream has ended
                        if result.unwrap() == 0 {break;}
                        // send to all clients
                        send_command.send(line.clone()).await.unwrap();
                        println!("Got in client loop {}", line.clone());

                        line.clear();
                    }
                    _result = recd_state.changed() => {
                        let new_state = recd_state.borrow().clone();
                        println!("got state back for the clients:{}", &(*new_state));
                        // ui_client_writer.write_all(&(*new_state).as_bytes());
                        ui_client_writer.write_all(&(*new_state).as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
