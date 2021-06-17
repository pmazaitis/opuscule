use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{unix::SocketAddr, TcpListener},
    sync::broadcast::{self, Receiver, Sender},
};

const ADDR: &'static str = "127.0.0.1:8080";

// Ui Clients Module
//
// Set up a loop to capture clients that connect
// expose an awaitable handle for the server logic to capture events
// expose a handle for distributing new state
#[derive(Clone, Debug)]
struct UiMessages {
    recvd_messages: Sender<(String, SocketAddr)>,
    send_ui_state: Receiver<(String, SocketAddr)>,
}

pub async fn run_ui_client_server() {
    // Set up a TcpListener object
    let listener = TcpListener::bind(&ADDR).await.unwrap();

    // TODO: reanme these to ui_rx, etc., and have another channel for server communication?

    // tokio broadcast channel; can we expose these?
    let (external_tx, _rx) = broadcast::channel(10);

    let server_tx = external_tx.clone();
    let mut server_rx = external_tx.subscribe();

    // let ui_messages = UiMessages {
    //     recvd_messages: server_tx,
    //     send_ui_state: server_rx,
    // };

    //We need to put accept into a loop for catching new clients
    loop {
        // call accept to accept new incoming connection for a stream and addr
        let (mut socket, addr) = listener.accept().await.unwrap();

        // We need to clone this so that each client gets one
        let external_tx = external_tx.clone();
        // use this method to pull the receiver out of transmit (!?)
        let mut external_rx = external_tx.subscribe();

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
                        external_tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = external_rx.recv() => {
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

// #[derive(Debug)]
// #[allow(dead_code)]
// struct MyActor {
//     receiver: String,

// }

// #[derive(Debug)]
// #[allow(dead_code)]
// enum ClientCommandType {
//     Play,
//     Stop,
//     Jump(u8),
// }

// #[derive(Debug)]
// struct ClientCommand {
//     addr: String,
//     //command: ClientCommandType,
//     command: String,
// }

// #![allow(unused)]

// /// the ui_clients server instance.
// pub struct UiClients {
//     listener: TcpListener,
//     connections: Arc<Mutex<VecMap<ClientConn>>>,
//     ids: Arc<Mutex<VecMap<u8>>>, // If we have more than u8 clients, there's a problem
//     new_r: Receiver<usize>,
//     thread: JoinHandle<()>,
// }

// impl UiClients {
//     pub fn new<A>(addr: A) -> io::Result<UiCLients>
//     where
//         A: ToSocketAddrs,
//     {
//         let listener = TcpListener::bind(&addr).await.unwrap();
//         let connections = Arc::new(Mutex::new(VecMap::new()));
//         let names = Arc::new(Mutex::new(VecMap::new()));
//         let (new_s, new_r) = channel();
//     }
// }
