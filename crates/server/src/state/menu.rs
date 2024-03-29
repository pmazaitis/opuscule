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

use serde::{Deserialize, Serialize};
use crate::common::{OpusId, ComponentCategory, OpInternalCommand, OpComponent};
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
    Component{label: String},
    Text{label: String},            
    Opus{label: String, id:OpusId},         
    SystemCommand(OpInternalCommand)  
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
struct CurrentNodeAndIndex<'i> {
  node: &'i Node<MenuItem>,
  index: u32
}

// impl <'i>  CurrentNodeAndIndex<'i>  {
//   
// }


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
      // Todo: sort into categories based in componnent's choice
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
        Ok(OpInternalCommand::Noop)
      } else {
        Err(MenuError::OutOfBounds)
      }
    }
    
    pub fn previous_child(&mut self) -> Result<OpInternalCommand, MenuError> {
      // Error if last element in child index is 0
      if *self.path.last().unwrap() > 0 {
        let pathfinal = self.path.last_mut().unwrap();
        *pathfinal -= 1;
        Ok(OpInternalCommand::Noop)
      } else {
        Err(MenuError::OutOfBounds)
      }
    }
    
    pub fn escape_to_parent(&mut self) -> Result<OpInternalCommand, MenuError> {
      // check if path has at least 2 elements
      if self.path.len() > 2 {
        self.path.pop();
        Ok(OpInternalCommand::Noop)
      } else {
        Err(MenuError::OutOfBounds)
      }
    }
    
    pub fn select_child(&mut self) -> Result<OpInternalCommand, MenuError> {
      match self.get_current_menu_node().data() {
        MenuItem::Opus{label, id}    => Ok(OpInternalCommand::Noop),
        MenuItem::SystemCommand(cmd) => Ok(cmd.clone()),
        _ => {
          if !self.get_current_menu_node().has_no_child() {
            self.path.push(0);
            Ok(OpInternalCommand::Noop)
          } else {
            Err(MenuError::OutOfBounds)
          }
        }
      }
    }

    fn get_current_menu_node(&self) -> &Node<MenuItem> {
      let mut menu_node = self.tree.root();
      let mut path_to_node = self.path.clone();
      path_to_node.truncate(self.path.len() - 1);
      let path_to_node = &path_to_node[1..];
       
      for path_index in path_to_node.iter() {
        for (child_index, n) in menu_node.iter().enumerate() {
          if *path_index == child_index as u32 {
            menu_node = n;
          }
        }
      } 

      menu_node      
    }
    
    fn current_node_and_index(&self) -> CurrentNodeAndIndex {
      
      // Prepare for descent. We need:
      // - The path of nodes to follow to the penultimate
      // - the index at the end
      let mut path_to_node = self.path.clone();
      let index = path_to_node.pop().unwrap();
      path_to_node.reverse();
      path_to_node.pop();
      
      fn find_curr(n:&Node<MenuItem>, mut p:Vec<u32>) -> &Node<MenuItem> {
        if p.is_empty() {
          n
        } else {
          let i = p.pop().unwrap();
          find_curr(get_nth_child(i, n), p)
        } 
      }
      
      fn get_nth_child<'i>(child_index:u32, menu_node:&'i Node<MenuItem>) -> &'i Node<MenuItem> {
        for (i, n) in menu_node.iter().enumerate() {
          if i as u32 == child_index {
            return n;
          }
        }
        menu_node
      }
      
      CurrentNodeAndIndex{node: find_curr(self.tree.root(), path_to_node), index}
    }
    
    pub fn get_menu_status(&self) -> MenuStatus {
      let mut menu_labels = Vec::new();
      let cni = self.current_node_and_index();
      
      for c in cni.node.iter() {
        menu_labels.push(c.data().to_string());
      }
      
      MenuStatus {
        menu_labels,
        cursor_index: cni.index
      }
    }
    
    // TODO: adding favorites to the menu - have this be .update_component(c, t)?
    // pub fn add_favorite(op: OpusId) {
    //   
    // }
    // pub fn remove_favorite(op: OpusId) {
    //   
    // }
}

impl fmt::Display for MenuItem {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      MenuItem::Root                    => write!(f, ""),
      MenuItem::Category(c)             => write!(f, "{}", c),
      MenuItem::Component { label }     => write!(f, "{}", label),
      MenuItem::Text { label }          => write!(f, "{}", label),
      MenuItem::Opus { label, id: _ }   => write!(f, "{}", label),
      MenuItem::SystemCommand(sct)      => write!(f, "{}", sct)
    }
  }
}

