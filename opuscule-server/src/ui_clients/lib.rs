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

/// the ui_clients server instance.
pub struct UiClients {
    listener: TcpListener,
    connections: Arc<Mutex<VecMap<ClientConn>>>,
    ids: Arc<Mutex<VecMap<u8>>>, // If we have more than u8 clients, there's a problem
    new_r: Receiver<usize>,
    thread: JoinHandle<()>,
}

impl UiClients {
    pub fn new<A>(addr: A) -> io::Result<UiCLients>
    where
        A: ToSocketAddrs,
    {
        let listener = TcpListener::bind(&addr).await.unwrap();
        let connections = Arc::new(Mutex::new(VecMap::new()));
        let names = Arc::new(Mutex::new(VecMap::new()));
        let (new_s, new_r) = channel();
    }
}
