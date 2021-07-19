use rodio::source::TakeDuration;
use rodio::source::{Amplify, SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};

use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

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

struct CompInternalTesting {
    // play_queue: VecDeque<Source>,
    sink: Sink,
}

impl CompInternalTesting {
    fn new(it_sink: Sink) -> CompInternalTesting {
        CompInternalTesting { sink: it_sink }
    }
    fn next(&self, s: Vec<SineWave>) {
        //self.sink.append(s)
    }
}

struct ItPlayable {
    parts: Vec<SineWave>,
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

// impl ItPlayable {
//     fn play(&self) {
//         // Add a dummy source of the sake of the example.
//         let source = SineWave::new(440)
//             .take_duration(Duration::from_secs_f32(0.25))
//             .amplify(0.20);
//         self.sink.append(source);
//         let source = SineWave::new(523)
//             .take_duration(Duration::from_secs_f32(0.25))
//             .amplify(0.20);
//         self.sink.append(source);
//         let source = SineWave::new(330)
//             .take_duration(Duration::from_secs_f32(0.25))
//             .amplify(0.20);
//         self.sink.append(source);
//         let source = SineWave::new(440)
//             .take_duration(Duration::from_secs_f32(0.25))
//             .amplify(0.20);
//         self.sink.append(source);

//         // The sound plays in a separate thread. This call will block the current thread until the sink
//         // has finished playing all its queued sounds.
//         self.sink.sleep_until_end();
//     }
// }
