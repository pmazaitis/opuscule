#![allow(dead_code)] // FIXME remove when done working through the data structures
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "OpUICommandType")]
pub enum OpUICommandType {
    Play,
    Stop,
    Pause,
    Favorite { slot: u8 },
    Advance,
    Retreat,
    Select,
    Escape,
    Random { set_to: Option<bool> },
    Repeat { set_to: Option<bool> },
    Next,
    Previous,
    Louder,
    Softer,
    Mute { set_to: Option<bool> },
    // Shutdown,
    Refresh,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpUICommand {
    pub addr: SocketAddr,
    pub command: OpUICommandType,
}

pub enum OpInternalCommandType {
    Stop,
    Play,
}

pub struct OpInternalCommand {
    pub command: OpInternalCommandType,
}

pub enum OpComponent {
    InternalTesting,
    CustomStreaming,
    // Shoutcast
    // PersonalMp3Streams
    // Oobler
    // ...
}

pub struct OpusId {
    component: OpComponent,
    id: u128, // TODO investigate ulid?
}

////// Structures for returning status

#[derive(Serialize, Deserialize, Debug)]
pub enum OpResult {
    OpStatus,
    OpError,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpStatus {
    // 'playstate': self.current_state,
    // 'component': self.menu.current_node.component,
    // 'messages': self.messages.compose_data(),
    menu: OpStatusMenu,
    now_playing: OpStatusNowPlaying,
    volume: OpStatusVolume,
    indicators: OpStatusIndicators,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpStatusMenu {
    path: String, // TODO more meaningful type here?
    items: Vec<String>,
    cursor: u8,
}
// for full desciptions, see: https://id3.org/id3v2.4.0-frames
#[derive(Serialize, Deserialize, Debug)]
pub struct OpStatusNowPlaying {
    opus_aenc: Option<String>, // Audio encryption
    opus_apic: Option<String>, // Attached picture
    opus_aspi: Option<String>, // Audio seek point index
    opus_comm: Option<String>, // Comments
    opus_comr: Option<String>, // Commercial frame
    opus_encr: Option<String>, // Encryption method registration
    opus_equ2: Option<String>, // Equalisation (2)
    opus_etco: Option<String>, // Event timing codes
    opus_geob: Option<String>, // General encapsulated object
    opus_grid: Option<String>, // Group identification registration
    opus_link: Option<String>, // Linked information
    opus_mcdi: Option<String>, // Music CD identifier
    opus_mllt: Option<String>, // MPEG location lookup table
    opus_owne: Option<String>, // Ownership frame
    opus_priv: Option<String>, // Private frame
    opus_pcnt: Option<String>, // Play counter
    opus_popm: Option<String>, // Popularimeter
    opus_poss: Option<String>, // Position synchronisation frame
    opus_rbuf: Option<String>, // Recommended buffer size
    opus_rva2: Option<String>, // Relative volume adjustment (2)
    opus_rvrb: Option<String>, // Reverb
    opus_seek: Option<String>, // Seek frame
    opus_sign: Option<String>, // Signature frame
    opus_sylt: Option<String>, // Synchronised lyric/text
    opus_sytc: Option<String>, // Synchronised tempo codes
    opus_talb: Option<String>, // Album/Movie/Show title
    opus_tbpm: Option<String>, // BPM (beats per minute)
    opus_tcom: Option<String>, // Composer
    opus_tcon: Option<String>, // Content type
    opus_tcop: Option<String>, // Copyright message
    opus_tden: Option<String>, // Encoding time
    opus_tdly: Option<String>, // Playlist delay
    opus_tdor: Option<String>, // Original release time
    opus_tdrc: Option<String>, // Recording time
    opus_tdrl: Option<String>, // Release time
    opus_tdtg: Option<String>, // Tagging time
    opus_tenc: Option<String>, // Encoded by
    opus_text: Option<String>, // Lyricist/Text writer
    opus_tflt: Option<String>, // File type
    opus_tipl: Option<String>, // Involved people list
    opus_tit1: Option<String>, // Content group description
    opus_tit2: String,         // Title/songname/content description
    opus_tit3: Option<String>, // Subtitle/Description refinement
    opus_tkey: Option<String>, // Initial key
    opus_tlan: Option<String>, // Language(s)
    opus_tlen: Option<String>, // Length
    opus_tmcl: Option<String>, // Musician credits list
    opus_tmed: Option<String>, // Media type
    opus_tmoo: Option<String>, // Mood
    opus_toal: Option<String>, // Original album/movie/show title
    opus_tofn: Option<String>, // Original filename
    opus_toly: Option<String>, // Original lyricist(s)/text writer(s)
    opus_tope: Option<String>, // Original artist(s)/performer(s)
    opus_town: Option<String>, // File owner/licensee
    opus_tpe1: Option<String>, // Lead performer(s)/Soloist(s)
    opus_tpe2: Option<String>, // Band/orchestra/accompaniment
    opus_tpe3: Option<String>, // Conductor/performer refinement
    opus_tpe4: Option<String>, // Interpreted, remixed, or otherwise modified by
    opus_tpos: Option<String>, // Part of a set
    opus_tpro: Option<String>, // Produced notice
    opus_tpub: Option<String>, // Publisher
    opus_trck: Option<String>, // Track number/Position in set
    opus_trsn: Option<String>, // Internet radio station name
    opus_trso: Option<String>, // Internet radio station owner
    opus_tsoa: Option<String>, // Album sort order
    opus_tsop: Option<String>, // Performer sort order
    opus_tsot: Option<String>, // Title sort order
    opus_tsrc: Option<String>, // ISRC (international standard recording code)
    opus_tsse: Option<String>, // Software/Hardware and settings used for encoding
    opus_tsst: Option<String>, // Set subtitle
    opus_txxx: Option<String>, // User defined text information frame
    opus_ufid: Option<String>, // Unique file identifier
    opus_user: Option<String>, // Terms of use
    opus_uslt: Option<String>, // Unsynchronised lyric/text transcription
    opus_wcom: Option<String>, // Commercial information
    opus_wcop: Option<String>, // Copyright/Legal information
    opus_woaf: Option<String>, // Official audio file webpage
    opus_woar: Option<String>, // Official artist/performer webpage
    opus_woas: Option<String>, // Official audio source webpage
    opus_wors: Option<String>, // Official Internet radio station homepage
    opus_wpay: Option<String>, // Payment
    opus_wpub: Option<String>, // Publishers official webpage
    opus_wxxx: Option<String>, // User defined URL link frame
}

#[derive(Serialize, Deserialize, Debug)]
struct OpStatusVolume {
    level: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpStatusIndicators {
    power: bool,
    play: bool,
    pause: bool,
    stop: bool,
    repeat: bool,
    shuffle: bool,
    mute: bool,
}

// Traits

trait CanPlay {}

trait CanPause {}

trait CanRandomize {}
