
use crate::state::menu::{MenuItem,MenuItemKind,MenuId};
use trees::{Tree, tr};

pub struct SystemMenu {
    
}

impl SystemMenu {
    pub fn new() -> Self {
        SystemMenu{}
    }
    pub fn get_menu(self) -> Tree<MenuItem> {
        // let id = MenuId::new_v4();
        // Tree::new(MenuItem::new(MenuItemKind::Text, "System".to_string(), id));
        tr(MenuItem::new(MenuItemKind::Text, "System".to_string(),MenuId::new_v4()))
            /( 
               tr(MenuItem::new(MenuItemKind::SystemCommand, "Reset".to_string(),MenuId::new_v4()))
              /tr(MenuItem::new(MenuItemKind::SystemCommand, "Shutdown".to_string(),MenuId::new_v4()))
            )
    }
}

