

#[derive(Debug)]
#[allow(dead_code)]
enum ClientCommandType {
    Play,
    Stop,
    Jump(u8),
}

#[derive(Debug)]
struct ClientCommand {
    addr: String,
    command: ClientCommandType,
}