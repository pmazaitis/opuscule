
use crate::state::menu::{MenuItem};
use trees::{Tree, tr};
use crate::common::ComponentCategory;
use std::fmt;

pub struct SystemMenu {
    
}

impl SystemMenu {
    pub fn new() -> Self {
        SystemMenu{}
    }
    pub fn get_menu(self) -> Tree<MenuItem> {
        // let id = MenuId::new_v4();
        // Tree::new(MenuItem::new(MenuItemKind::Text, "System".to_string(), id));
        tr(MenuItem::Category(ComponentCategory::System))
            /( 
               tr(MenuItem::SystemCommand(SystemCommandType::RestartServer))
              /tr(MenuItem::SystemCommand(SystemCommandType::ShutdownServer))
            )
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
