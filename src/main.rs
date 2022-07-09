#![allow(dead_code)]
#![allow(unused)]

mod clients;
mod common;
mod components;
mod state;

use rodio::{OutputStream, Sink};

// use components::internal_testing::internal_testing_sine::InternalSine;

use components::internal_testing::nullcomp::NullCompActorHandler;

use components::internal_testing::nullcomp::NullCompOpus;

// , NullCompOpus};

use common::OpComponentCommand;

#[macro_use]
extern crate machine;

// use tokio::time::{sleep, Duration};
use tokio::{sync::mpsc, sync::watch};
#[allow(unused_imports)] // FIXME remove this when we know what tracing options we need
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;

use std::collections::HashMap;
use std::{thread, time};

use state::state_controller::Controller;

use common::{OpUICommand, OpUICommandType};

#[allow(unused_imports)]
use common::OpResult;

#[tokio::main]
async fn main() -> ! {
    // Tracing
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let op_config = state::configure::OpSettings::new();

    let server_addr = op_config.get_server_address();

    //Channels
    // We offer the ui_clients module the tx here, so we can get the commands it receives
    let (ui_cmds_tx, mut ui_cmds_rx) = mpsc::channel::<String>(10);
    // We offer the ui_clients module the rx so it can distribute state
    let (ui_status_tx, ui_state_rx) = watch::channel::<String>(String::from("NOOP"));
    // We offer the components the tx here, so we can get the state they generate
    // let (internal_state_tx, mut internal_state_rx) = mpsc::channel::<OpComponentCommand>(10);

    let (internal_state_tx, mut internal_state_rx) = mpsc::channel::<String>(10);
    // We offer the components the rx so the controller can accept commands from the controller
    let (internal_cmds_tx, internal_cmds_rx) = watch::channel::<String>(String::from("NOOP"));

    // Controller to manage the player state
    let mut op_controller = Controller::new();

    // Spin up UI server to handle user interface clients connecting over the net
    let ui_client_controller =
        clients::ui_client_controller::handle_ui_clients(server_addr, ui_cmds_tx, ui_state_rx);
    tokio::spawn(ui_client_controller);

    // Spin up the rodio subsystem to create and manage audio streams for each component
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Initialize and start components

    // InternalTesting component

    // let comp_in_test = InternalSine::new(
    //     Sink::try_new(&stream_handle).unwrap(),
    //     internal_state_tx.clone(),
    //     internal_cmds_rx.clone(),
    // );

    // comp_in_test.play_test_melody();
    // comp_in_test.play_test_melody_reverse();
    // comp_in_test.load(1);
    // comp_in_test.load(2);
    // comp_in_test.load(1);
    // comp_in_test.play();
    // // let internal_test_handle = tokio::spawn(comp_in_test.run());

    // //sleep(Duration::from_millis(1000)).await;

    // thread::sleep(time::Duration::from_secs(5));

    // comp_in_test.load(2);

    // # NullComp

    // ## Create Test Operai

    let mut null_comp_menu: HashMap<u32, NullCompOpus> = HashMap::new();

    null_comp_menu.insert(1, NullCompOpus::new(1, String::from("test 1")));
    null_comp_menu.insert(2, NullCompOpus::new(2, String::from("test 2")));

    // ## Create Null Comp Actor

    let mut comp_null =
        NullCompActorHandler::new(internal_cmds_rx.clone(), internal_state_tx.clone());

    // Main loop

    loop {
        tokio::select! {
            ui_command = ui_cmds_rx.recv() => {

                let rec_command = serde_json::from_str(ui_command.unwrap().as_str()).unwrap();

                debug!("Sending test to internal command channel");
                internal_cmds_tx.send("test".to_string());


                let status = op_controller.handle_command(rec_command);

                let status_ser = serde_json::to_string(&status).unwrap();

                ui_status_tx.send(status_ser).unwrap();
            }
            // check internal command bus here
            int_state_update = internal_state_rx.recv() => {

            }
        }
    }
}
