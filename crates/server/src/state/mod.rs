

pub mod audio;
pub mod menu; 
pub mod now_playing;

// use crate::settings::Settings;
use crate::clients::ui_client_messages::{OpUICommand, OpUICommandType, OpResult, OpStatus, OpPlayState};
use menu::Menu;
use audio::{AudioState, Stop, Pause, Play, Stopped};
use crate::system::command::SystemMenu;
use crate::system::favorites::FavoritesMenu;

pub struct State{
    menu: Menu,
    machine: AudioState,
}

impl State {
    pub fn new(_s: String) -> Self {
        // use settings to establish which components we have
        
        let mut menu = Menu::new();
        
        // Pull in the opuscule menus, favorites first
        let favorites_menu = FavoritesMenu::new();
        menu.add_component(favorites_menu.get_menu());
        
        // Pull in menus for requested components
        // match s {
        //     _ =>  {}
        // }
        
        // The final menu is the system menu
        let system_menu = SystemMenu::new();
        menu.add_component(system_menu.get_menu());
        
        // println!("Root menu: {:?}", &menu);
        // menu.show_children();
        
        println!("*** Menu Status: {}",menu.get_menu_status());
        // 
        // menu.next_child();
        // 
        // println!("*** Menu Status: {}",menu.get_menu_status());
        // 
        // menu.next_child();
        // 
        // println!("*** Menu Status: {}",menu.get_menu_status());
        // 
        // menu.next_child();
        // 
        // println!("*** Menu Status: {}",menu.get_menu_status());
        // 
        // menu.previous_child();
        // 
        // println!("*** Menu Status: {}",menu.get_menu_status());
        // 
        // menu.next_child();
        // 
        // println!("*** Menu Status: {}",menu.get_menu_status());
        
        let machine = AudioState::Stopped(Stopped {});
        
        State{
            menu,
            machine
        }
    }
    pub fn handle_ui_command(&mut self, rc: OpUICommand) -> OpUICommand {
        match rc.command {
            OpUICommandType::Play => {
                println!("Got Play");
                self.machine = self.machine.on_play(Play);
            }
            OpUICommandType::Stop => {
                println!("Got Stop");
                self.machine = self.machine.on_stop(Stop);
            }
            OpUICommandType::Pause => {
                println!("Got Pause");
                self.machine = self.machine.on_pause(Pause);
            }
            OpUICommandType::Advance => {
                println!("Got Advance");
                self.menu.next_child().unwrap();
                println!("*** Menu Status: {}",self.menu.get_menu_status());
            }
            OpUICommandType::Retreat => {
                println!("Got Retreat");
                self.menu.previous_child().unwrap();
                println!("*** Menu Status: {}",self.menu.get_menu_status());
            }
            OpUICommandType::Select => {
                println!("Got Select");
                self.menu.select_child().unwrap();
                println!("*** Menu Status: {}",self.menu.get_menu_status());
                println!("{:?}", self.menu);
            }
            OpUICommandType::Escape => {
                println!("Got Escape");
                self.menu.escape_to_parent().unwrap();
                println!("*** Menu Status: {}",self.menu.get_menu_status());
            }
            // Make this argument optional for those times we just want to toggle fav status of current opus
            OpUICommandType::Favorite { slot: favid } => {
                println!("Got Fav: {}", favid);
            }
            OpUICommandType::Mute { set_to: muted } => match muted {
                Some(m) => println!("Setting mute to {}", m),
                None => println!("Toggling mute"),
            },
            OpUICommandType::Random { set_to: randomized } => match randomized {
                Some(r) => println!("Setting random to {}", r),
                None => println!("Toggling random"),
            },
            OpUICommandType::Repeat { set_to: repeating } => match repeating {
                Some(r) => println!("Setting repeat to {}", r),
                None => println!("Toggling repeat"),
            },
            OpUICommandType::Refresh => {
                println!("Got Refresh request");
            }
            OpUICommandType::Next => {
                println!("Got Next");
            }
            OpUICommandType::Previous => {
                println!("Got Previous");
            }
            OpUICommandType::Louder => {
                println!("Got Louder");
            }
            OpUICommandType::Softer => {
                println!("Got Softer");
            }
        }
        rc
    }
    
    fn generate_status(self) -> OpStatus {
        
        let playstate = OpPlayState {audio: self.machine.get_state().unwrap()};
        let menu = self.menu.get_menu_status();
        
        OpStatus::new(
            playstate, menu
        )
        
    }

}
