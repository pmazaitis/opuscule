#![allow(dead_code)]
// use rodio::source::SineWave;
// FIXME remove when done working through the data structures
use serde::{Deserialize, Serialize};

use uuid::Uuid;
// use trees::Tree;
pub type OpusUuid = Uuid;
use std::fmt;

use crate::clients::ui_client_messages::OpResult;



/// Internal commands (sent from component to component, or from menu)

#[derive(Debug, Clone)]
pub enum OpInternalCommand {
    Pause,
    Play,
    Load {id: OpusId },
    ClearOpus,
    Reload,
    ClearQueue,
    Noop,
    Restart,
    Shutdown
}
impl fmt::Display for OpInternalCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpInternalCommand::Pause        => write!(f, "Pause"),
            OpInternalCommand::Play         => write!(f, "Play"),
            OpInternalCommand::Load{id: _}  => write!(f, "Load"),
            OpInternalCommand::ClearOpus    => write!(f, "Clear"),
            OpInternalCommand::Reload       => write!(f, "Reload"),
            OpInternalCommand::ClearQueue   => write!(f, "Clear Queue"),
            OpInternalCommand::Noop         => write!(f, "Noop"),
            OpInternalCommand::Restart      => write!(f, "Restart"),
            OpInternalCommand::Shutdown     => write!(f, "Shutdown"),
        }
    }
}


/// Component Structure ////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ComponentCategory {
    System,
    Testing,
    Library,
    Stream,
    Soundscape,
    Radio,
    Favorites
}
impl fmt::Display for ComponentCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ComponentCategory::System => write!(f, "System"),
            ComponentCategory::Testing => write!(f, "Testing"),
            ComponentCategory::Library => write!(f, "Library"),
            ComponentCategory::Stream => write!(f, "Stream"),
            ComponentCategory::Soundscape => write!(f, "Soundscape"),
            ComponentCategory::Radio => write!(f, "Radio"),
            ComponentCategory::Favorites => write!(f, "Favorites"),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpComponent {
    Favorites,
    System,
    Opuscule,
    NullComp,
    SineWave,
    Mp3,
    Local,
    Subsonic,
    Custom,
    Shoutcast,
    Spotify,
    Oobler,
    Boodler,
    FM,
    WX,
    All,
    None
}

// Opus Structure //////////////////////


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpusId {
    pub(crate) component: OpComponent,
    pub(crate) id: OpusUuid,
}

/// Traits

pub trait AudioComponent {
    fn play() -> OpResult;
    fn pause() -> OpResult;
    fn stop() -> OpResult;
    fn status() -> OpResult;
    fn load(opus: OpusId) -> OpResult;
    fn clear() -> OpResult;
    fn get_playables_menu() -> Result<String, OpComponentError>;
    // .get_playables_json() -> Result<String, E>
}

pub trait SystemComponent {
    fn get_menu() -> Result<String, OpComponentError>;
}


pub trait Playable {
    // fn play() -> OpResult;
    // fn pause() -> OpResult;
    // fn stop() -> OpResult;
    fn toggle_repeat() -> OpResult;
    fn set_repeat(status:bool) -> OpResult;
    fn toggle_random() -> OpResult;
    fn set_random(status:bool) -> OpResult;
    // .get_playables_json() -> Result<String, E>
}

// Errors

pub enum OpComponentError {
    Load,
    Play,
    Pause,
    
}
