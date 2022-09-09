// The menu module
//
// The menu module handles the menu state of the ui.
//
// Interface
//
// 

// TODOs
//
// Have a select put a command on the internal command bus (can be a NOOP)


use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::common::{OpusId, ComponentCategory, OpInternalCommand};
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
    SystemCommand{label: String}   
}

impl fmt::Display for MenuItem {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MenuItem::Root                    => write!(f, ""),
      MenuItem::Category(c)               => write!(f, "{}",c),
      MenuItem::Text { label }          => write!(f, "{}",label),
      MenuItem::Opus { label, id: _ }      => write!(f, "{}",label),
      MenuItem::SystemCommand { label } => write!(f, "{}",label),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]pub struct MenuStatus {
  menu_labels: Vec<String>,
  cursor_index: u32,
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

    pub fn next_child(&mut self) -> Result<String, MenuError> {     
      if *self.path.last().unwrap() < (self.get_current_menu_node().degree() as u32 - 1) {
        let pathfinal = self.path.last_mut().unwrap();
        *pathfinal += 1;
        self.print_menu(self.get_current_menu_node());
        Ok("Menu advanced".to_string())
      } else {
        Err(MenuError::OutOfBounds)
      }
    }
    
    pub fn previous_child(&mut self) -> Result<String, MenuError> {
      // Error if last element in child index is 0
      if *self.path.last().unwrap() > 0 {
        let pathfinal = self.path.last_mut().unwrap();
        *pathfinal -= 1;
        self.print_menu(self.get_current_menu_node());
        Ok("Menu retreated".to_string())
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

    pub fn get_current_menu_node(&self) -> &Node<MenuItem> {
      // This one is working - maybe we don't need IDs?
      let menu_node = self.tree.root();
      
      let mut ns = self.tree.root().iter();
       
      for (m, c) in self.path.iter().tuple_windows() {
          println!("{}--{}", &m, &c);
          // let child_index = c;
          // self.print_menu(menu_node);
          let menu_node = ns.next().unwrap();
      } 
      menu_node      
    }
    
    fn print_menu(&self, mi:&Node<MenuItem>) {
      let mut menu_labels = Vec::new();
      let idx = self.path.last().unwrap();
      
      
      for c in mi.iter() {
        
        menu_labels.push(c.data().to_string());
      }
      
      println!("Index: {}, Menu: {:?}",idx, menu_labels);
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

}

