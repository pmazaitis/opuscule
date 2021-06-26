use super::{OpUICommand, OpUICommandType};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::mpsc,
    sync::watch,
};

use mpsc::Sender;
use watch::Receiver;

pub async fn handle_ui_clients(
    server_addr: String,
    send_command_channel: Sender<String>,
    recd_state_channel: Receiver<String>,
) -> ! {
    // Set up a TcpListener object
    let listener = TcpListener::bind(&server_addr).await.unwrap();

    // We need to loop to catch each new client as it comes in
    loop {
        let (mut ui_socket, ui_client_addr) = listener.accept().await.unwrap();

        // clone the channels for commands and state
        let send_command_channel = send_command_channel.clone();
        let mut recd_state_channel = recd_state_channel.clone();

        let _ui_client_handle = tokio::spawn(async move {
            //handle read and write independantly
            let (ui_client_reader, mut ui_client_writer) = ui_socket.split();

            let mut reader = BufReader::new(ui_client_reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        // Test to see if stream has ended
                        //if result.unwrap() == 0 {break;}

                        match result {
                            Ok(res) => {
                                if res == 0 {break;}
                            }
                            Err(_) => {
                                let e_message = "Badly formed command!";
                                match ui_client_writer.write_all(e_message.as_bytes()).await {
                                    Ok(_) => {}
                                    Err(_) => {}
                                }
                            }
                        }

                        // process json and send up to the server

                        match serde_json::from_str(&line.as_str()) {
                            Ok(cti) => {
                                let cmd_type_in: OpUICommandType  = cti;
                                let cmd: OpUICommand = OpUICommand {addr: ui_client_addr, command: cmd_type_in};
                                let cmd_string: String = serde_json::to_string(&cmd).unwrap();
                                send_command_channel.send(cmd_string).await.unwrap();
                            }
                            Err(_err) => {
                                let e_message = "Badly formed command!";
                                match ui_client_writer.write_all(e_message.as_bytes()).await {
                                    Ok(_) => {}
                                    Err(_) => {}
                                }
                            }
                        }
                        line.clear();
                    }
                    _result = recd_state_channel.changed() => {
                        let new_state = recd_state_channel.borrow().clone();
                        debug!("got state back for the clients:{:#?}", &(*new_state));
                        // ui_client_writer.write_all(&(*new_state).as_bytes());
                        // ui_client_writer.write_all(&(*new_state).as_bytes()).await.unwrap();
                        match ui_client_writer.write_all(&(*new_state).as_bytes()).await {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
            }
        });
    }
}
