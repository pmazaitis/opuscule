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

struct Menu {
    menu_tree: Arena<OpusId>,
    curr_path: Vec<MenuId>
}

impl Menu {
    fn new() -> Self {
        let mut menu_tree = Arena::<OpusId>::new();
        let root_node_id = MenuId::new_v4();
        let root_node = OpusId {component: OpComponent::Opuscule, id: root_node_id};
        Menu {menu_tree, curr_path: vec!(root_node_id)}
    }
}
