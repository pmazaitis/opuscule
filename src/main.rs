#![allow(dead_code)]
#![allow(unused)]

mod common;
mod component_internal_testing;
mod controller;
mod player;
mod ui_clients;

use rodio::{OutputStream, Sink};

use component_internal_testing::internal_testing_sine::InternalSine;

use player::internal_command_protocol::InternalCommand;

#[macro_use]
extern crate machine;

// use tokio::time::{sleep, Duration};
use tokio::{sync::mpsc, sync::watch};
#[allow(unused_imports)] // FIXME remove this when we know what tracing options we need
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;

use std::{thread, time};

use controller::Controller;

use common::{OpUICommand, OpUICommandType};

#[allow(unused_imports)]
use common::OpResult;

#[tokio::main]
async fn main() -> ! {
    // Tracing
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let op_config = player::configure::OpSettings::new();

    let server_addr = op_config.get_server_address();

    //Channels
    // We offer the ui_clients module the tx here, so we can get the commands it receives
    let (ui_cmds_tx, mut ui_cmds_rx) = mpsc::channel::<String>(10);
    // We offer the ui_clients module the rx so it can distribute state
    let (ui_status_tx, ui_state_rx) = watch::channel::<String>(String::from("NOOP"));
    // We offer the components the tx here, so we can get the state they generate
    let (internal_state_tx, mut internal_state_rx) = mpsc::channel::<InternalCommand>(10);
    // We offer the components the rx so the controller can accept commands from the controller
    let (internal_cmds_tx, internal_cmds_rx) = watch::channel::<String>(String::from("NOOP"));

    // Controller to manage the player state
    let mut op_controller = Controller::new();

    // Spin up UI server to handle user interface clients connecting over the net
    let ui_client_server = ui_clients::handle_ui_clients(server_addr, ui_cmds_tx, ui_state_rx);
    tokio::spawn(ui_client_server);

    // Spin up the rodio subsystem to manage streams
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Initialize and start components

    // InternalTesting component
    // let it_sink = Sink::try_new(&stream_handle).unwrap();

    // let comp_in_test = InternalSine::new(it_sink);

    let comp_in_test = InternalSine::new(
        Sink::try_new(&stream_handle).unwrap(),
        internal_state_tx,
        internal_cmds_rx,
    );

    comp_in_test.play_test_melody();
    // comp_in_test.play_test_melody_reverse();
    comp_in_test.load(1);

    // let internal_test_handle = tokio::spawn(comp_in_test.run());

    //sleep(Duration::from_millis(1000)).await;

    thread::sleep(time::Duration::from_secs(5));

    comp_in_test.load(2);

    // Main loop

    loop {
        tokio::select! {
            result = ui_cmds_rx.recv() => {

                let rec_command = serde_json::from_str(result.unwrap().as_str()).unwrap();

                debug!("UI client command received in server loop: {:?}", &rec_command);

                let status = op_controller.handle_command(rec_command);

                let status_ser = serde_json::to_string(&status).unwrap();

                ui_status_tx.send(status_ser).unwrap();
            }
        }
    }
}
