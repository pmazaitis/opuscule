
#[derive(Debug)]
#[allow(dead_code)]
enum StreamingCommandType {
    Play,
    Stop,
}

#[derive(Debug)]
struct StreamingCommand {
    // addr: String,
    command: StreamingCommandType,
}