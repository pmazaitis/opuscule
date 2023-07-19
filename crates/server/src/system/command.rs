
use crate::state::menu::{MenuItem};
use trees::{Tree, tr};
use crate::common::{ComponentCategory, OpInternalCommand};
use std::fmt;

pub struct SystemMenu {
    
}

impl SystemMenu {
    pub fn new() -> Self {
        SystemMenu{}
    }
    pub fn get_menu(self) -> Tree<MenuItem> {
        tr(MenuItem::Category(ComponentCategory::System))
            /tr(MenuItem::SystemCommand(OpInternalCommand::Restart))
            /tr(MenuItem::SystemCommand(OpInternalCommand::Shutdown))        
    }
}

#[derive(Debug)]
pub enum SystemCommandType {
    ShutdownServer,
    RestartServer,
}
impl fmt::Display for SystemCommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemCommandType::ShutdownServer => write!(f, "Shutdown"),
            SystemCommandType::RestartServer => write!(f, "Restart"),
        }
    }
}
