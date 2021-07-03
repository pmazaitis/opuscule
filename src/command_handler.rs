use super::{OpUICommand, OpUICommandType};
extern crate finny;

#[allow(dead_code)]
struct IndicatorsStatus {
    power: bool,
    play: bool,
    pause: bool,
    stop: bool,
    repeat: bool,
    shuffle: bool,
    mute: bool,
}

#[allow(dead_code)]
struct VolumeStatus {
    level: u8,
    muted: bool,
}

#[allow(dead_code)]
struct NowPlayingStatus {}

pub fn handle_command(rc: OpUICommand) -> OpUICommand {
    match rc.command {
        OpUICommandType::Play => {
            println!("Got Play");
        }
        OpUICommandType::Stop => {
            println!("Got Stop");
        }
        OpUICommandType::Pause => {
            println!("Got Pause");
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
    return rc;
}
