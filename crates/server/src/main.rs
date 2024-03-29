// #![allow(dead_code)]
// #![allow(unused)]

mod clients;
mod common;
// mod components;
mod settings;
mod state;
mod system;

use settings::Settings;

use rodio::OutputStream;

use state::State;

#[macro_use]
extern crate machine;

// use tokio::time::{sleep, Duration};
use tokio::{sync::mpsc, sync::watch};
#[allow(unused_imports)] // FIXME remove this when we know what tracing options we need
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::FmtSubscriber;

#[allow(unused_imports)]
use clients::ui_client_messages::OpResult;

#[tokio::main]
async fn main() -> ! {
    // --- Tracing
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // --- Settings
    let settings = match Settings::new() {
        Ok(s) => s,
        Err(_e) => std::process::exit(-65),
    };

    println!("settings: {:?}", &settings);

    // --- Channels (move these into modules?)
    // We offer the ui_clients module the tx here, so we can get the commands it receives
    let (ui_cmds_tx, mut ui_cmds_rx) = mpsc::channel::<String>(10);
    // We offer the ui_clients module the rx so it can distribute state
    let (ui_status_tx, ui_state_rx) = watch::channel::<String>(String::from("NOOP"));
    // We offer the components the tx here, so we can get the state they generate
    // let (internal_state_tx, mut internal_state_rx) = mpsc::channel::<OpComponentCommand>(10);

    let (_internal_state_tx, mut internal_state_rx) = mpsc::channel::<String>(10);
    // We offer the components the rx so the controller can accept commands from the controller
    let (internal_cmds_tx, _internal_cmds_rx) = watch::channel::<String>(String::from("NOOP"));

    // Controller to manage the player state
    let mut op_state = State::new(settings.enabled_components());

    // Spin up UI server to handle user interface clients connecting over the net
    let ui_client_controller = clients::ui_client_controller::handle_ui_clients(
        settings.server_addr(),
        ui_cmds_tx,
        ui_state_rx,
    );
    tokio::spawn(ui_client_controller);

    // Spin up the rodio subsystem to create and manage audio streams for each component
    let (_stream, _stream_handle) = OutputStream::try_default().unwrap();

    // Main loop

    loop {
        tokio::select! {
            ui_command = ui_cmds_rx.recv() => {

                let rec_command = serde_json::from_str(ui_command.unwrap().as_str()).unwrap();

                debug!("Sending test to internal command channel");
                internal_cmds_tx.send("test".to_string()).unwrap();

                let status = op_state.handle_ui_command(rec_command);

                let status_ser = serde_json::to_string(&status).unwrap();

                ui_status_tx.send(status_ser).unwrap();
            }
            // check internal command bus here
            _int_state_update = internal_state_rx.recv() => {

            }
        }
    }
}
