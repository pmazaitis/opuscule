// The idea here is to have an internal set of channels so that
// components can receive guidance from the controller, and also
// send back updates/data, etc.

enum InternalRecipient {
    All,
    InternalTestingSine,
    InternalTestingMp3,
}

enum InternalCommandType {
    Stop,
    Pause,
    Play,
}

pub struct InternalCommand {
    recipient: InternalRecipient,
    command: InternalCommandType,
}
