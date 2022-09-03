use tokio::sync::{mpsc::Sender, watch::Receiver};

use tracing::{debug, error, info, trace, warn};

use crate::common::{OpComponent, OpComponentCategory, OpComponentCommand, OpComponentError, OpStatusMetaData, AudioComponent, Playable, OpResult, OpusId};

// This is a module to model and test the component system.
//
// A component must support the following:
//
// - (menu)  Present a menu subtree of available nodes on request
// - (load)  Accept a command to load a playable into the local queue, with a UUID as an argument
// - (play)  Accept a command to begin playing from the local queue
// - (stop)  Accept a command to stop playing from the local queue
// - (clear) Accept a command to clear the local queue
//
// A component can support the following, if reasonable to do so:
//
// - (pause)  Accept a command to pause the current queue, and start from the paused position
// - (random) Set a flag to (destructively) randomize the local queue
// - (repeat) Set a flag to repeat on an empty queue

// Phase I - get the message bus working

pub struct NullCompOpus {
    id: u32,
    list_of_playables: String,
    repeat: bool,
    random: bool,
    //metadata: OpStatusMetaData,
}

impl NullCompOpus {
    pub fn new(id: u32, list_of_playables: String) -> Self {
        Self {
            id,
            list_of_playables,
            repeat: false,
            random: false,
            //metadata: Op
        }
    }
}

/*
What if we try this:

on creating a new struct, the event handlers are async move'd into a
concurrently running block?
*/

#[derive(Clone)]
pub struct NullCompActorHandler {
    incoming_command_channel: Receiver<String>,
    outgoing_status_channel: Sender<String>,
}

impl AudioComponent for NullCompActorHandler {
    fn play() -> OpResult {
        OpResult::OpStatus
    }
    
    fn pause() -> OpResult {
        OpResult::OpStatus
    }
    
    fn stop() -> OpResult {
        OpResult::OpStatus
    }
    
    fn status() -> OpResult {
        OpResult::OpStatus
    }
    
    fn load(opus: OpusId) -> OpResult {
        OpResult::OpStatus
    }

    fn clear() -> OpResult {
        OpResult::OpStatus
    }
    
    fn get_playables_menu() -> Result<String, OpComponentError> {
       Ok("Whee".to_string())
    }
    

}

impl NullCompActorHandler {
    pub fn new(
        incoming_command_channel: Receiver<String>,
        outgoing_status_channel: Sender<String>,
    ) -> Self {
        // let (sender, receiver) = mpsc::channel(8);
        // let actor = MyActor::new(receiver);
        // tokio::spawn(run_my_actor(actor));
        Self {
            incoming_command_channel,
            outgoing_status_channel,
        }
    }

    pub async fn handle_messages(&mut self) {
        loop {
            // tokio::select! {
            //     result = self.incoming_command_channel.changed() {
            //         self.outgoing_status_channel.send(result);
            //     }
            //     _result
            // }
            while self.incoming_command_channel.changed().await.is_ok() {
                let result: String = self.incoming_command_channel.borrow().to_string().clone();
                debug!("NullComp command received in loop: {:?}", &result);
                self.outgoing_status_channel.send(result);
            }
        }
    }
}

// struct NullCompActor {}


//     fn play() -> OpResult;
// fn pause() -> OpResult;
// fn stop() -> OpResult;
// fn status() -> OpResult;
// fn load(opus: Opus) -> OpResult;
// fn toggle_repeat() -> OpResult;
// fn set_repeat(status:bool) -> OpResult;
// fn toggle_random() -> OpResult;
// fn set_random(status:bool) -> OpResult;
// fn clear_queue() -> OpResult;
// fn get_playables_menu() -> Result<String, OpComponentError>;
