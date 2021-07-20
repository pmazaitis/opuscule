use rodio::source::TakeDuration;
use rodio::source::{Amplify, SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

// On start up, we need to:
//
// 1. gather all of our available playables
// 1. arrange them in a tree structure for the menu
// 1. offer the menu chunk tot he player

// Then, we need to accept UUIDs and send the appropriate data to the sink

pub struct CompInternalTesting {
    play_queue: VecDeque<TestSineWave>,
    // now_playing: TestSineWave,
    sink: Sink,
    cmd_tx: (),
    status_rx: (),
    catalog: HashMap<u32, TestSineWave>,
}

impl CompInternalTesting {
    pub fn new(it_sink: Sink) -> CompInternalTesting {
        CompInternalTesting {
            play_queue: VecDeque::new(),
            // now_playing: TestSineWave::new(),
            sink: it_sink,
            cmd_tx: (),
            status_rx: (),
            catalog: HashMap::new(),
        }
    }
    fn next(&self, s: Vec<TestSineWave>) {}
    pub fn play_test_melody(&self) {
        let test_melody = TestSineWave::test_melody();
        for item in test_melody.melody {
            let (freq, duration) = item;
            self.sink.append(
                SineWave::new(freq)
                    .take_duration(Duration::from_secs_f32(duration))
                    .amplify(0.20),
            );
        }
    }
    pub fn play_test_melody_reverse(&self) {
        let test_melody_reverse = TestSineWave::test_melody_reverse();
        for item in test_melody_reverse.melody {
            let (freq, duration) = item;
            self.sink.append(
                SineWave::new(freq)
                    .take_duration(Duration::from_secs_f32(duration))
                    .amplify(0.20),
            );
        }
    }
}

struct TestSineWave {
    melody: Vec<(u32, f32)>,
}

impl TestSineWave {
    fn test_melody() -> Self {
        TestSineWave {
            melody: vec![(440, 0.25), (532, 0.25), (330, 0.25), (440, 0.25)],
        }
    }
    fn test_melody_reverse() -> Self {
        TestSineWave {
            melody: vec![(440, 0.25), (330, 0.25), (532, 0.25), (440, 0.25)],
        }
    }
}

struct TestMp3 {}

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
