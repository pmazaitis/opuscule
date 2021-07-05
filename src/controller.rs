use crate::common::{OpUICommand, OpUICommandType};

machine!(
    #[derive(Clone, Debug, PartialEq, Copy)]
    enum AudioState {
        Playing,
        Paused,
        Stopped,
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct RequestStop;

#[derive(Clone, Debug, PartialEq)]
pub struct RequestPause;

#[derive(Clone, Debug, PartialEq)]
pub struct RequestPlay;

transitions!(AudioState,
  [
    (Playing, RequestPause) => [Paused, Stopped],
    (Playing, RequestStop) => Stopped,
    (Playing, RequestPlay) => Playing,
    (Paused, RequestPlay) => Playing,
    (Paused, RequestStop) => Stopped,
    (Paused, RequestPause) => Paused,
    (Stopped, RequestPlay) => Playing,
    (Stopped, RequestStop) => Stopped,
    (Stopped, RequestPause) => Stopped
  ]
);

impl Playing {
    pub fn on_request_pause(self, _: RequestPause) -> AudioState {
        // FIXME - test if current Opus is pausable to determine how this branches
        if true {
            println!("State moving to Paused inside the machine");
            AudioState::paused()
        } else {
            println!("State moving to Stopped inside the machine");
            AudioState::stopped()
        }
    }
    pub fn on_request_stop(self, _: RequestStop) -> Stopped {
        println!("State moving to Stopped inside the machine");
        Stopped {}
    }
    pub fn on_request_play(self, _: RequestPlay) -> Playing {
        println!("Maintaining Playing inside the machine");
        Playing {}
    }
}

impl Paused {
    pub fn on_request_play(self, _: RequestPlay) -> Playing {
        println!("State moving to Playing inside the machine");
        Playing {}
    }
    pub fn on_request_stop(self, _: RequestStop) -> Stopped {
        println!("State moving to Stopped inside the machine");
        Stopped {}
    }
    pub fn on_request_pause(self, _: RequestPause) -> Paused {
        println!("Maintaining Paused inside the machine");
        Paused {}
    }
}

impl Stopped {
    pub fn on_request_play(self, _: RequestPlay) -> Playing {
        println!("State moving to Playing inside the machine");
        Playing {}
    }
    pub fn on_request_stop(self, _: RequestStop) -> Stopped {
        println!("Maintaining Stopped inside the machine");
        Stopped {}
    }
    pub fn on_request_pause(self, _: RequestPause) -> Stopped {
        println!("Maintaining Stopped inside the machine");
        Stopped {}
    }
}

pub struct Controller {
    state: AudioState,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            state: AudioState::Stopped(Stopped {}),
        }
    }

    pub fn handle_command(&mut self, rc: OpUICommand) -> OpUICommand {
        match rc.command {
            OpUICommandType::Play => {
                println!("Got Play");
                self.state = self.state.on_request_play(RequestPlay);
            }
            OpUICommandType::Stop => {
                println!("Got Stop");
                self.state = self.state.on_request_stop(RequestStop);
            }
            OpUICommandType::Pause => {
                println!("Got Pause");
                self.state = self.state.on_request_pause(RequestPause);
            }
            OpUICommandType::Advance => {
                println!("Got Advance");
            }
            OpUICommandType::Retreat => {
                println!("Got Retreat");
            }
            OpUICommandType::Select => {
                println!("Got Select");
            }
            OpUICommandType::Escape => {
                println!("Got Escape");
            }
            OpUICommandType::Favorite { slot: favid } => {
                println!("Got Fav: {}", favid);
            }
            OpUICommandType::Mute { set_to: muted } => match muted {
                Some(m) => println!("Setting mute to {}", m),
                None => println!("Toggling mute"),
            },
            OpUICommandType::Random { set_to: randomized } => match randomized {
                Some(r) => println!("Setting random to {}", r),
                None => println!("Toggling random"),
            },
            OpUICommandType::Repeat { set_to: repeating } => match repeating {
                Some(r) => println!("Setting repeat to {}", r),
                None => println!("Toggling repeat"),
            },
            OpUICommandType::Refresh => {
                println!("Got Refresh request");
            }
            OpUICommandType::Next => {
                println!("Got Nexnt");
            }
            OpUICommandType::Previous => {
                println!("Got Previous");
            }
            OpUICommandType::Louder => {
                println!("Got Louder");
            }
            OpUICommandType::Softer => {
                println!("Got Softer");
            }
        }
        //return (rc, astate);
        return rc;
    }
}
