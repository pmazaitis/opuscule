// The menu module
//
// The menu module handles the menu state of the ui.
//
// Interface
//
// 

use crate::common::{Playable, OpResult, OpusId, OpComponent};

use r3bl_rs_utils::{
  tree_memory_arena::{Arena, HasId, ResultUidList},
  utils::{style_primary, style_prompt},
};

use uuid::Uuid;
type MenuId = Uuid;

#[derive(Debug, Clone)]
enum Node {
    Opus{name: String, id:OpusId},
    Label{name: String},
    // Command
}

struct Menu {
    menu_tree: Arena<Node>,
    curr_path: Vec<MenuId>
}



impl Menu {
    fn new() -> Self {
        let mut menu_tree = Arena::<Node>::new();
        let root_node_id = MenuId::new_v4();
        let root_node = Node::Opus{name: "Root".to_string(), id: OpusId {component: OpComponent::Opuscule, id: root_node_id}};
        Menu {menu_tree.add_new_node(root_node, ), curr_path: vec!(root_node_id)}
    }
}
