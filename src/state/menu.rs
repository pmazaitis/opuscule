
use trees::Tree;

// The menu module
//
// The menu module handles the menu state of the ui.
//
// Interface
//
// menu.





use crate::common::{Playable, OpResult, OpusID};

enum OpMenuNode {
    Opus(OpusID),
    Label(MenuLabel),
    Root,
    // Command()
}


struct MenuLabel {
    title: String,
    short_title: Option<String>,
    
}


struct Menu<'m>  {

}

impl Menu<'_> {
    // Generate a new Menu object to keep track of:
    //  - Entire menu tree of the appliance
    //  - Current position in the menu tree
    //
    fn new() {

    }
    
    // Add component tree 
    fn add_component(tree: Tree) {
        
    }
    
    fn advance() {
        
    }
    
    fn retreat() {
        
    }
    
    fn select() {
        
    }
    
    fn escape() {
        
    }
    
    
}
