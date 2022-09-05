// The menu module
//
// The menu module handles the menu state of the ui.
//
// Interface
//
// 

use serde::{Deserialize, Serialize};
use crate::common::{Playable, OpResult, OpusId, SystemComponent, SystemCommand};
use trees::{Tree, Node};
use uuid::Uuid;

// Menu

#[derive(Debug)]
pub enum MenuError {
  OutOfBounds,
}

pub type MenuId = Uuid;

#[derive(Debug)]
pub enum MenuItemKind {
    Root,
    Text,            
    Opus{id:OpusId},         
    SystemCommand    
}

#[derive(Debug)]
pub struct MenuItem {
  kind: MenuItemKind,
  label: String,
  id: MenuId
}

impl MenuItem {
    pub fn new(kind: MenuItemKind, label:String, id: MenuId) -> Self {
        MenuItem{kind, label, id}
    }
    pub fn get_menuitem_id(&self) -> MenuId {
      self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]pub struct MenuStatus {
  menu_labels: Vec<String>,
  cursor_index: usize,
}

#[derive(Debug)]
pub(crate) struct Menu {
    pub tree: Tree<MenuItem>,
    pub cursor: Vec<MenuId>,
    pub child_index: Vec<usize>
}

impl Menu {
    pub fn new() -> Self {
        let id = MenuId::new_v4();
        let tree = Tree::new(MenuItem{kind: MenuItemKind::Root, label: "ROOT".to_string(), id});
        Menu{tree, cursor: vec![id], child_index: vec![0]}

    }
    
    pub fn add_component(&mut self, st: Tree<MenuItem>) {
      self.tree.push_back(st);
      self.initialize_cursor();
    }
    
    fn initialize_cursor(&mut self) {
      // Preserve root node
      self.cursor.truncate(1);      
      //find ID of first child, add it to menu cursor
      // self.cursor.push(self.tree.front().unwrap().data().get_menuitem_id());
      self.child_index = vec![0];
    }
    // TODO: next for menu
    // fn get_menu_item_under_cursor(&self) -> MenuItem {
    //   // 
    // }
    // 
    pub fn next_child(&mut self) -> Result<String, MenuError> {
      //Manipulate cursor
      Err(MenuError::OutOfBounds)
    }
    
    pub fn previous_child(&mut self) -> Result<String, MenuError> {
      Err(MenuError::OutOfBounds)
    }
    
    pub fn select_child(&mut self) -> Result<String, MenuError> {
      // match 
      Err(MenuError::OutOfBounds)
    }
    
    pub fn escape_to_parent(&mut self) -> Result<String, MenuError> {
      Err(MenuError::OutOfBounds)
    }
    pub fn show_children(&self) {
      for c in self.tree.iter() {
        println!("{:?}", c.data().label);
      }
    }
    
    pub fn get_menu_status(&self) -> MenuStatus {
      let mut menu_labels = Vec::new();
      
      for c in self.tree.iter() {
        menu_labels.push(c.data().label.clone());
      }
      
      MenuStatus {
        menu_labels,
        cursor_index: self.child_index.last().copied().unwrap()
      }
    }

}

