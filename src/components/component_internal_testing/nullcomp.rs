use tokio::sync::{mpsc::Sender, watch::Receiver};

use tracing::{debug, error, info, trace, warn};

use crate::common::{OpComponent, OpComponentCategory, OpComponentCommand, OpStatusMetaData};

// Phase I - get the message bus working

#[derive(Clone)]
pub struct NullCompActorHandler {
    incoming_command_channel: Receiver<String>,
    outgoing_status_channel: Sender<String>,
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
