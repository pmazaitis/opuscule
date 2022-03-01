use tokio::sync::{mpsc, oneshot};
use tokio::sync::{mpsc::Sender, watch::Receiver};

use tracing::{debug, error, info, trace, warn};

use crate::common::{OpComponent, OpComponentCategory, OpComponentCommand, OpStatusMetaData};

// Phase I - get the message bus working

enum ActorMessage {
    GetUniqueId { respond_to: oneshot::Sender<u32> },
}

struct NullCompActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}

impl NullCompActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        NullCompActor {
            receiver,
            next_id: 0,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;

                // The `let _ =` ignores any errors when sending.
                //
                // This can happen if the `select!` macro is used
                // to cancel waiting for the response.
                let _ = respond_to.send(self.next_id);
            }
        }
    }
}

async fn run_my_actor(mut actor: NullCompActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct NullCompActorHandler {
    incoming_command_channel: Receiver<String>,
    outgoing_status_channel: Sender<String>,
    internal_sender: mpsc::Sender<ActorMessage>,
}

impl NullCompActorHandler {
    pub fn new(
        incoming_command_channel: Receiver<String>,
        outgoing_status_channel: Sender<String>,
    ) -> Self {
        let (internal_sender, internal_receiver) = mpsc::channel(8);
        let actor = NullCompActor::new(internal_receiver);
        tokio::spawn(run_my_actor(actor));
        Self {
            incoming_command_channel,
            outgoing_status_channel,
            internal_sender,
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
