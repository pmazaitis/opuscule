// The favorites module
//
// The favorites module keeps track of operai that have designated as favorites
//
// Interface
//
// 

// Todo: have   a way to insert a favorite so that it's auto-files into the menu tree

use crate::state::menu::MenuItem;
use trees::{Tree, tr};
use crate::common::ComponentCategory;

pub struct FavoritesMenu {
    
}

impl FavoritesMenu {
    pub fn new() -> Self {
        FavoritesMenu{}
    }
    pub fn get_menu(self) -> Tree<MenuItem> {
        // let id = MenuId::new_v4();
        // Tree::new(MenuItem::new(MenuItemKind::Text, "System".to_string(), id));
        tr(MenuItem::Category(ComponentCategory::Favorites))
            /( 
               tr(MenuItem::Text{label: "One".to_string()})
              /tr(MenuItem::Text{label: "Two".to_string()})
            )
    }
}
