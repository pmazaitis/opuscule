mod common;
mod component_internal_testing;
mod controller;
mod player;
mod ui_clients;

// use justconfig::item::ValueExtractor;
// use justconfig::processors::{Explode, Trim};
// use justconfig::sources::defaults::Defaults;
// use justconfig::sources::env::Env;
// use justconfig::sources::text::ConfigText;
// use justconfig::validators::Range;
// use justconfig::ConfPath;
// use justconfig::Config;
// use std::fs::File;

// use rodio::source::{SineWave, Source};
// use rodio::{Decoder, OutputStream, Sink};
// use std::io::BufReader;
// use std::time::Duration;

use component_internal_testing::internal_testing::CompInternalTesting;

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

#[tokio::main]
async fn main() -> ! {
    // Tracing
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let op_config = player::configure::OpSettings::new();

    // TODO Move this functionality into the configure module
    // use justconfig::item::ValueExtractor;
    // use justconfig::processors::Trim;
    // let server_addr: String = settings
    //     .get(settings.root().push("server").push("address"))
    //     .trim()
    //     .value()
    //     .expect("Could not get the server addr from the conf file");

    let server_addr = op_config.get_server_address();

    // State machine to manage the player state
    let mut con = Controller::new();

    //Channels
    // We offer the ui_clients module the tx here, so we can get the commands it receives
    let (ui_cmds_tx, mut ui_cmds_rx) = mpsc::channel::<String>(10);
    // We offer the ui_clients module the rx so it can distribute state
    let (status_tx, state_rx) = watch::channel::<String>(String::from("NOOP"));

    // Spin up server to handle user interface clients connecting over the net
    let ui_client_server = ui_clients::handle_ui_clients(server_addr, ui_cmds_tx, state_rx);
    tokio::spawn(ui_client_server);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Initialize and start components

    // InternalTesting component
    let it_sink = Sink::try_new(&stream_handle).unwrap();

    let comp_in_test = CompInternalTesting::new(it_sink);

    comp_in_test.play_test_melody();
    comp_in_test.play_test_melody_reverse();

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
