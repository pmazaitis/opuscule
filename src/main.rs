mod common;
mod controller;
mod ui_clients;

#[macro_use]
extern crate machine;

use tokio::{sync::mpsc, sync::watch};
#[allow(unused_imports)] // FIXME remove this when we know what tracing options we need
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;

use controller::Controller;

use common::{OpUICommand, OpUICommandType};

#[allow(unused_imports)]
use common::OpResult;

// We eventually want to get this from config
const ADDR: &'static str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> ! {
    // Tracing
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    // State machine to manage the player state
    // let audio_state = controller::AudioState::Stopped(Stopped {});

    let mut con = Controller::new();

    //Channels
    // We offer the ui_clients module the tx here, so we can get the commands it receives
    let (ui_cmds_tx, mut ui_cmds_rx) = mpsc::channel::<String>(10);
    // We offer the ui_clients module the rx so it can distribute state
    let (status_tx, state_rx) = watch::channel::<String>(String::from("NOOP"));

    // Spin up server to handle user interface clients connecting over the net
    let ui_client_server = ui_clients::handle_ui_clients(String::from(ADDR), ui_cmds_tx, state_rx);
    tokio::spawn(ui_client_server);

    // Initialize and start components

    // Main loop

    loop {
        tokio::select! {
            result = ui_cmds_rx.recv() => {

                let rec_command = serde_json::from_str(result.unwrap().as_str()).unwrap();

                debug!("UI client command received in server loop: {:?}", &rec_command);

                let status = con.handle_command(rec_command);

                let status_ser = serde_json::to_string(&status).unwrap();

                status_tx.send(status_ser).unwrap();
            }
        }
    }
}
