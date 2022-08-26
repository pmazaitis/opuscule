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

// We need something to scan the direcotry and look for files to load in:
// {.pls, .m3u, .url}

// Create a hash map with ids

// function to accept an OpusId for this component and send it to the sink.
