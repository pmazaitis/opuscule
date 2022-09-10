// The menu module
//
// The menu module handles the menu state of the ui.
//
// Interface
//
// 

// TODOs
//
// Reorganize this


use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::common::{OpusId, ComponentCategory, OpInternalCommand, OpInternalCommandType, OpComponent};
use crate::system::command::SystemCommandType;
use trees::{Tree, Node};
use std::fmt;

// Menu

#[derive(Debug)]
pub enum MenuError {
  OutOfBounds,
}

#[derive(Debug)]
pub enum MenuItem {
    Root,
    Category(ComponentCategory),
    Text{label: String},            
    Opus{label: String, id:OpusId},         
    SystemCommand(SystemCommandType)  
}

impl fmt::Display for MenuItem {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MenuItem::Root                    => write!(f, ""),
      MenuItem::Category(c)             => write!(f, "{}", c),
      MenuItem::Text { label }          => write!(f, "{}", label),
      MenuItem::Opus { label, id: _ }   => write!(f, "{}", label),
      MenuItem::SystemCommand(sct)      => write!(f, "{}", sct)
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]pub struct MenuStatus {
  menu_labels: Vec<String>,
  cursor_index: u32,
}

impl fmt::Display for MenuStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut output = String::new();
    for (i, s) in self.menu_labels.iter().enumerate() {
        if i as u32 != self.cursor_index {
          output += " ";
          output += s;
          output += " ";
        } else {
          output += ">";
          output += s;
          output += "<";
        }
    }
    write!(f, "{}", output)
  }
}

#[derive(Debug)]
pub(crate) struct Menu {
    pub tree: Tree<MenuItem>,
    pub path: Vec<u32>
}


impl Menu {
    pub fn new() -> Self {
        let tree = Tree::new(MenuItem::Root);
        Menu{tree, path: vec![0]}
    }
    
    pub fn add_component(&mut self, st: Tree<MenuItem>) {
      // Todo: sort into categories based in componnetn's choice
      self.tree.push_back(st);
      self.initialize_path();
    }
    
    fn initialize_path(&mut self) {     
      self.path = vec![0,0];
    }

    pub fn next_child(&mut self) -> Result<OpInternalCommand, MenuError> {     
      if *self.path.last().unwrap() < (self.get_current_menu_node().degree() as u32 - 1) {
        let pathfinal = self.path.last_mut().unwrap();
        *pathfinal += 1;
        Ok(OpInternalCommand{recipient: OpComponent::None, command: OpInternalCommandType::Noop})
      } else {
        Err(MenuError::OutOfBounds)
      }
    }
    
    pub fn previous_child(&mut self) -> Result<OpInternalCommand, MenuError> {
      // Error if last element in child index is 0
      if *self.path.last().unwrap() > 0 {
        let pathfinal = self.path.last_mut().unwrap();
        *pathfinal -= 1;
        Ok(OpInternalCommand{recipient: OpComponent::None, command: OpInternalCommandType::Noop})
      } else {
        Err(MenuError::OutOfBounds)
      }
    }
    
    pub fn select_child(&mut self) -> Result<String, MenuError> {
      // behavior changes based on what kind of menuItem it is 
      Err(MenuError::OutOfBounds)
    }
    
    pub fn escape_to_parent(&mut self) -> Result<String, MenuError> {
      // check if path has at least 2 elements
      Err(MenuError::OutOfBounds)
    }

    fn get_current_menu_node(&self) -> &Node<MenuItem> {
      let menu_node = self.tree.root();
      
      let mut ns = self.tree.root().iter();
       
      for (m, c) in self.path.iter().tuple_windows() {
          // println!("{}--{}", &m, &c);
          // let child_index = c;
          // self.print_menu(menu_node);
          let menu_node = ns.next().unwrap();
      } 
      menu_node      
    }
    
    pub fn get_menu_status(&self) -> MenuStatus {
      let mut menu_labels = Vec::new();
      
      for c in self.tree.iter() {
        menu_labels.push(c.data().to_string());
      }
      
      MenuStatus {
        menu_labels,
        cursor_index: self.path.last().copied().unwrap()
      }
    }
    
    pub fn add_favorite(op: OpusId) {
      
    }
    pub fn remove_favorite(op: OpusId) {
      
    }
}

