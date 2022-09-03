

pub mod audio;
pub mod menu; 
pub mod now_playing;

use crate::settings::Settings;
use crate::common::{OpUICommand, OpUICommandType};
use menu::Menu;
use audio::{AudioState, Stop, Pause, Play};


pub struct State {
    menu: Menu,
    machine: AudioState,
}

impl State {
    pub fn new(&s: &Settings) -> Self {
        State{}
    }
    pub fn handle_command(&mut self, rc: OpUICommand) -> OpUICommand {
        match rc.command {
            OpUICommandType::Play => {
                println!("Got Play");
                self.machine = self.machine.on_play(Play);
            }
            OpUICommandType::Stop => {
                println!("Got Stop");
                self.machine = self.machine.on_stop(Stop);
            }
            OpUICommandType::Pause => {
                println!("Got Pause");
                self.machine = self.machine.on_pause(Pause);
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
                println!("Got Next");
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
        rc
    }

}
