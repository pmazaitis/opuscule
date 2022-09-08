
use crate::state::menu::{MenuItem};
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
        tr(MenuItem::Text{label: "System".to_string()})
            /( 
               tr(MenuItem::SystemCommand { label: "Restart".to_string() })
              /tr(MenuItem::SystemCommand { label: "Shutdown".to_string() })
            )
    }
}

