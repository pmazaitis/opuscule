// The favorites module
//
// The favorites module keeps track of operai that have designated as favorites
//
// Interface
//
// 


use crate::state::menu::{MenuItem,MenuItemKind,MenuId};
use trees::{Tree, tr};

pub struct FavoritesMenu {
    
}

impl FavoritesMenu {
    pub fn new() -> Self {
        FavoritesMenu{}
    }
    pub fn get_menu(self) -> Tree<MenuItem> {
        // let id = MenuId::new_v4();
        // Tree::new(MenuItem::new(MenuItemKind::Text, "System".to_string(), id));
        tr(MenuItem::new(MenuItemKind::Text, "Favorites".to_string(),MenuId::new_v4()))
            /( 
               tr(MenuItem::new(MenuItemKind::SystemCommand, "One".to_string(),MenuId::new_v4()))
              /tr(MenuItem::new(MenuItemKind::SystemCommand, "Two".to_string(),MenuId::new_v4()))
            )
    }
}
