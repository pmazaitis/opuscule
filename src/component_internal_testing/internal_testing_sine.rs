use rodio::source::TakeDuration;
use rodio::source::{Amplify, SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use tokio::sync::{mpsc::Sender, watch::Receiver};

use tracing::{debug, error, info, trace, warn};

use crate::InternalCommand;

use crate::common::OpStatusMetaData;

// On start up, we need to:
//
// 1. gather all of our available playables
// 1. arrange them in a tree structure for the menu
// 1. offer the menu chunk tot he player

// Then, we need to accept UUIDs and send the appropriate data to the sink

pub struct InternalSine {
    play_queue: VecDeque<TestSineWave>,
    source_queue: VecDeque<TestSineWave>,
    sink: Sink,
    status_tx: Sender<InternalCommand>,
    cmd_rx: Receiver<String>,
    catalog: HashMap<u32, TestSineWave>,
}

impl InternalSine {
    pub fn new(
        it_sink: Sink,
        internal_tx: Sender<InternalCommand>,
        internal_rx: Receiver<String>,
    ) -> Self {
        let mut catalog = HashMap::new();

        catalog.insert(1, TestSineWave::test_melody_01());
        catalog.insert(2, TestSineWave::test_melody_01_reverse());

        Self {
            play_queue: VecDeque::new(),
            source_queue: VecDeque::new(),
            sink: it_sink,
            status_tx: internal_tx,
            cmd_rx: internal_rx,
            catalog: catalog,
        }
    }

    pub async fn run(&self) {
        let mut rx = self.cmd_rx.clone();
        loop {
            tokio::select! {
                _internal_command = rx.changed() => {
                    let new_command = rx.borrow().clone();
                    debug!("got state back for the clients:{:#?}", &(*new_command));}
            }
        }
    }

    pub fn load(&self, id: u32) {
        match self.catalog.get(&id) {
            Some(o) => {
                for note in o.playable_list.clone() {
                    self.sink.append(
                        SineWave::new(note.frequency)
                            .take_duration(Duration::from_secs_f32(note.duration))
                            .amplify(0.20),
                    );
                }
            }
            None => {}
        }
    }

    pub fn play_test_melody(&self) {
        let lookup: u32 = 2;
        let test_melody = self.catalog.get(&lookup).unwrap().playable_list.clone();
        for note in test_melody {
            self.sink.append(
                SineWave::new(note.frequency)
                    .take_duration(Duration::from_secs_f32(note.duration))
                    .amplify(0.20),
            );
        }
    }
    // pub fn play_test_melody_reverse(&self) {
    //     let test_melody_reverse = TestSineWave::test_melody_reverse();
    //     for item in test_melody_reverse.playable_list {
    //         let (freq, duration) = item;
    //         self.sink.append(
    //             SineWave::new(freq)
    //                 .take_duration(Duration::from_secs_f32(duration))
    //                 .amplify(0.20),
    //         );
    //     }
    // }
    pub async fn handle_queue() {}
}

#[derive(Copy, Clone, Debug)]
struct Note {
    frequency: u32,
    duration: f32,
}
impl Note {
    fn new(frequency: u32, duration: f32) -> Self {
        Self {
            frequency,
            duration,
        }
    }
}

#[derive(Clone, Debug)]
struct TestSineWave {
    playable_list: Vec<Note>,
    repeat: bool,
    random: bool,
    metadata: OpStatusMetaData,
}

impl TestSineWave {
    fn test_melody_01() -> Self {
        TestSineWave {
            playable_list: vec![
                Note::new(440, 0.25),
                Note::new(532, 0.25),
                Note::new(330, 0.25),
                Note::new(440, 0.25),
            ],
            repeat: false,
            random: false,
            metadata: OpStatusMetaData {
                opus_tit2: "Test Sine Wave".to_string(),
                ..Default::default()
            },
        }
    }
    fn test_melody_01_reverse() -> Self {
        TestSineWave {
            playable_list: vec![
                Note::new(440, 0.25),
                Note::new(330, 0.25),
                Note::new(532, 0.25),
                Note::new(440, 0.25),
            ],
            repeat: false,
            random: false,
            metadata: OpStatusMetaData {
                opus_tit2: "Test Sine Wave Reverse".to_string(),
                ..Default::default()
            },
        }
    }
}

// impl ItPlayable {
//     fn song_1() -> Vec<SineWave> {
//         let playable_list = Vec::new();
//         playable_list.push(SineWave::new(440));
//         playable_list.push(SineWave::new(532));
//         playable_list.push(SineWave::new(330));
//         playable_list.push(SineWave::new(440));
//         return playable_list;
//     }
// }

// pub fn song(it_sink: Sink) {
//     // Add a dummy source of the sake of the example.
//     let source = SineWave::new(440)
//         .take_duration(Duration::from_secs_f32(0.25))
//         .amplify(0.20);
//     it_sink.append(source);
//     let source = SineWave::new(523)
//         .take_duration(Duration::from_secs_f32(0.25))
//         .amplify(0.20);
//     it_sink.append(source);
//     let source = SineWave::new(330)
//         .take_duration(Duration::from_secs_f32(0.25))
//         .amplify(0.20);
//     it_sink.append(source);
//     let source = SineWave::new(440)
//         .take_duration(Duration::from_secs_f32(0.25))
//         .amplify(0.20);
//     it_sink.append(source);

//     // The sound plays in a separate thread. This call will block the current thread until the sink
//     // has finished playing all its queued sounds.
//     it_sink.sleep_until_end();
// }
