use tokio::net::unix::SocketAddr;
use tokio::{sync::mpsc, sync::watch};

mod ui_clients;

// We eventually want to get this from config
const ADDR: &'static str = "127.0.0.1:8080";

enum OpUICommandType {
    Play,
    Stop,
    Pause,
    Favorite(u8),
}
struct OpUICommand {
    addr: SocketAddr,
    command: OpUICommandType,
}

enum OpResult {
    OpState,
    OpError,
}

#[tokio::main]
async fn main() {
    // Set up channels here

    //Channels
    // We offer the ui_clients module the tx here, so we can get the commands it receives
    let (ui_cmds_tx, mut ui_cmds_rx) = mpsc::channel::<String>(10);
    // We offer the ui_clients module the rx so it can distribute state
    let (state_tx, state_rx) = watch::channel::<String>(String::from("NOOP"));

    let ui_client_server = ui_clients::handle_ui_clients(String::from(ADDR), ui_cmds_tx, state_rx);
    tokio::spawn(ui_client_server);

    loop {
        // tokio::select! {
        //     result = ui_cmds_rx.recv() => {
        //         // Test to see if stream has ended
        //         // if result.unwrap() == 0 {break;}
        //         // send to all clients
        //         // cmds_tx.send((line.clone(), addr)).unwrap();
        //         println!("Got in server loop {:?}", result.unwrap());
        //         let new_state = format!("Broadcasting result: {}", result.unwrap());

        //         state_tx.send(new_state).unwrap();
        //     }
        // }
        let result = ui_cmds_rx.recv().await.unwrap();
        println!("Got in server loop {:?}", result);
        state_tx.send(result).unwrap();
        println!("State sent");
    }
}
