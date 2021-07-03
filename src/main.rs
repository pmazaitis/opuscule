mod audio_state;
mod command_handler;
mod common;
mod ui_clients;

use audio_state::{AudioStateContext, Stopped};
// use futures::prelude::*;
// use serde::{Deserialize, Serialize};
// use std::net::SocketAddr;
use tokio::{sync::mpsc, sync::watch};
// use tokio_serde_json::{ReadJson, WriteJson};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;

extern crate finny;
use finny::FsmFactory;

use command_handler::handle_command;

use common::{OpUICommand, OpUICommandType};

use common::OpResult;

// We eventually want to get this from config
const ADDR: &'static str = "127.0.0.1:8080";

#[tokio::main]
async fn main() {
    // Tracing
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    // State machine to manage the player state
    let mut as_fsm = audio_state::AudioStateFsm::new(AudioStateContext::default()).unwrap();
    as_fsm.start().unwrap();

    //Channels
    // We offer the ui_clients module the tx here, so we can get the commands it receives
    let (ui_cmds_tx, mut ui_cmds_rx) = mpsc::channel::<String>(10);
    // We offer the ui_clients module the rx so it can distribute state
    let (status_tx, state_rx) = watch::channel::<String>(String::from("NOOP"));

    // Spin up server to handle user interface clients connecting over the net
    let ui_client_server = ui_clients::handle_ui_clients(String::from(ADDR), ui_cmds_tx, state_rx);
    tokio::spawn(ui_client_server);

    // Initialize and start components

    loop {
        tokio::select! {
            result = ui_cmds_rx.recv() => {

                let rec_command: OpUICommand = serde_json::from_str(result.unwrap().as_str()).unwrap();

                debug!("Command received in server loop: {:?}", &rec_command);

                let rec_command = handle_command(rec_command);

                let rec_command_ser = serde_json::to_string(&rec_command).unwrap();

                status_tx.send(rec_command_ser).unwrap();
            }
        }
    }
}
