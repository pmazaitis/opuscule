#![allow(dead_code)]
use rodio::source::SineWave;
// FIXME remove when done working through the data structures
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use uuid::Uuid;
type OpusSerialId = Uuid;

// Messages /////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpUICommand {
    pub addr: SocketAddr,
    pub command: OpUICommandType,
}

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
    Refresh,
}

#[derive(Debug, Clone)]
pub struct OpInternalCommand {
    pub recipient: OpComponent,
    pub command: OpInternalCommandType,
}

#[derive(Debug, Clone)]
pub enum OpInternalCommandType {
    Pause,
    Play,
    GetCatalog,
    Load { id: OpusId },
    ClearOpus,
    Reload,
    ClearQueue,
    Repeat(Option<bool>),
    Shuffle(Option<bool>),
}

#[derive(Debug, Clone)]
pub struct OpComponentCommand {
    pub command: OpComponentCommandType,
}

#[derive(Debug, Clone)]
pub enum OpComponentCommandType {
    CatalogUpdate,
    Stopped,
    Priority(OpusId),
}

pub enum SystemCommand {
    Quit,
    RestartServer,
}


// Component Structure ////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpComponentCategory {
    System,
    Testing,
    Library,
    Stream,
    Soundscape,
    Radio,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpComponent {
    Opuscule,
    NullComp,
    SineWave,
    Mp3,
    Local,
    Subsonic,
    Custom,
    Shoutcast,
    Spotify,
    Oobler,
    Boodler,
    FM,
    WX,
}

pub fn get_component_category(opcomp: OpComponent) -> OpComponentCategory {
    match opcomp {
        OpComponent::Opuscule => OpComponentCategory::System,
        OpComponent::NullComp => OpComponentCategory::Testing,
        OpComponent::SineWave => OpComponentCategory::Testing,
        OpComponent::Mp3 => OpComponentCategory::Testing,
        OpComponent::Local => OpComponentCategory::Library,
        OpComponent::Subsonic => OpComponentCategory::Library,
        OpComponent::Custom => OpComponentCategory::Stream,
        OpComponent::Shoutcast => OpComponentCategory::Stream,
        OpComponent::Spotify => OpComponentCategory::Stream,
        OpComponent::Oobler => OpComponentCategory::Soundscape,
        OpComponent::Boodler => OpComponentCategory::Soundscape,
        OpComponent::FM => OpComponentCategory::Radio,
        OpComponent::WX => OpComponentCategory::Radio,
    }
}

// Opus Structure //////////////////////


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpusId {
    pub(crate) component: OpComponent,
    pub(crate) id: OpusSerialId,
}



////// Structures for returning status ////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub enum OpResult {
    OpStatus,
    OpError,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpStatus {
    playstate: OpPlayState,
    messages: Vec<OpMessage>,
    errors: Vec<OpError>,
    menu: OpStatusMenu,
    now_playing: OpStatusMetaData,
    volume: OpStatusVolume,
    indicators: OpStatusIndicators,
}

#[derive(Clone)]
pub enum OpusType {
    NullComp,
}

#[derive(Serialize, Deserialize, Debug)]
struct OpPlayState {}

#[derive(Serialize, Deserialize, Debug)]
struct OpMessage {}

#[derive(Serialize, Deserialize, Debug)]
struct OpError {}

#[derive(Serialize, Deserialize, Debug)]
struct OpStatusMenu {
    path: String, // TODO more meaningful type here?
    items: Vec<String>,
    cursor: u8,
}
// for full desciptions, see: https://id3.org/id3v2.4.0-frames
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpStatusMetaData {
    pub opus_aenc: Option<String>, // Audio encryption
    pub opus_apic: Option<String>, // Attached picture
    pub opus_aspi: Option<String>, // Audio seek point index
    pub opus_comm: Option<String>, // Comments
    pub opus_comr: Option<String>, // Commercial frame
    pub opus_encr: Option<String>, // Encryption method registration
    pub opus_equ2: Option<String>, // Equalisation (2)
    pub opus_etco: Option<String>, // Event timing codes
    pub opus_geob: Option<String>, // General encapsulated object
    pub opus_grid: Option<String>, // Group identification registration
    pub opus_link: Option<String>, // Linked information
    pub opus_mcdi: Option<String>, // Music CD identifier
    pub opus_mllt: Option<String>, // MPEG location lookup table
    pub opus_owne: Option<String>, // Ownership frame
    pub opus_priv: Option<String>, // Private frame
    pub opus_pcnt: Option<String>, // Play counter
    pub opus_popm: Option<String>, // Popularimeter
    pub opus_poss: Option<String>, // Position synchronisation frame
    pub opus_rbuf: Option<String>, // Recommended buffer size
    pub opus_rva2: Option<String>, // Relative volume adjustment (2)
    pub opus_rvrb: Option<String>, // Reverb
    pub opus_seek: Option<String>, // Seek frame
    pub opus_sign: Option<String>, // Signature frame
    pub opus_sylt: Option<String>, // Synchronised lyric/text
    pub opus_sytc: Option<String>, // Synchronised tempo codes
    pub opus_talb: Option<String>, // Album/Movie/Show title
    pub opus_tbpm: Option<String>, // BPM (beats per minute)
    pub opus_tcom: Option<String>, // Composer
    pub opus_tcon: Option<String>, // Content type
    pub opus_tcop: Option<String>, // Copyright message
    pub opus_tden: Option<String>, // Encoding time
    pub opus_tdly: Option<String>, // Playlist delay
    pub opus_tdor: Option<String>, // Original release time
    pub opus_tdrc: Option<String>, // Recording time
    pub opus_tdrl: Option<String>, // Release time
    pub opus_tdtg: Option<String>, // Tagging time
    pub opus_tenc: Option<String>, // Encoded by
    pub opus_text: Option<String>, // Lyricist/Text writer
    pub opus_tflt: Option<String>, // File type
    pub opus_tipl: Option<String>, // Involved people list
    pub opus_tit1: Option<String>, // Content group description
    pub opus_tit2: String,         // Title/songname/content description
    pub opus_tit3: Option<String>, // Subtitle/Description refinement
    pub opus_tkey: Option<String>, // Initial key
    pub opus_tlan: Option<String>, // Language(s)
    pub opus_tlen: Option<String>, // Length
    pub opus_tmcl: Option<String>, // Musician credits list
    pub opus_tmed: Option<String>, // Media type
    pub opus_tmoo: Option<String>, // Mood
    pub opus_toal: Option<String>, // Original album/movie/show title
    pub opus_tofn: Option<String>, // Original filename
    pub opus_toly: Option<String>, // Original lyricist(s)/text writer(s)
    pub opus_tope: Option<String>, // Original artist(s)/performer(s)
    pub opus_town: Option<String>, // File owner/licensee
    pub opus_tpe1: Option<String>, // Lead performer(s)/Soloist(s)
    pub opus_tpe2: Option<String>, // Band/orchestra/accompaniment
    pub opus_tpe3: Option<String>, // Conductor/performer refinement
    pub opus_tpe4: Option<String>, // Interpreted, remixed, or otherwise modified by
    pub opus_tpos: Option<String>, // Part of a set
    pub opus_tpro: Option<String>, // Produced notice
    pub opus_tpub: Option<String>, // Publisher
    pub opus_trck: Option<String>, // Track number/Position in set
    pub opus_trsn: Option<String>, // Internet radio station name
    pub opus_trso: Option<String>, // Internet radio station owner
    pub opus_tsoa: Option<String>, // Album sort order
    pub opus_tsop: Option<String>, // Performer sort order
    pub opus_tsot: Option<String>, // Title sort order
    pub opus_tsrc: Option<String>, // ISRC (international standard recording code)
    pub opus_tsse: Option<String>, // Software/Hardware and settings used for encoding
    pub opus_tsst: Option<String>, // Set subtitle
    pub opus_txxx: Option<String>, // User defined text information frame
    pub opus_ufid: Option<String>, // Unique file identifier
    pub opus_user: Option<String>, // Terms of use
    pub opus_uslt: Option<String>, // Unsynchronised lyric/text transcription
    pub opus_wcom: Option<String>, // Commercial information
    pub opus_wcop: Option<String>, // Copyright/Legal information
    pub opus_woaf: Option<String>, // Official audio file webpage
    pub opus_woar: Option<String>, // Official artist/performer webpage
    pub opus_woas: Option<String>, // Official audio source webpage
    pub opus_wors: Option<String>, // Official Internet radio station homepage
    pub opus_wpay: Option<String>, // Payment
    pub opus_wpub: Option<String>, // Publishers official webpage
    pub opus_wxxx: Option<String>, // User defined URL link frame
}

impl Default for OpStatusMetaData {
    fn default() -> Self {
        Self {
            opus_aenc: None,                   // Audio encryption
            opus_apic: None,                   // Attached picture
            opus_aspi: None,                   // Audio seek point index
            opus_comm: None,                   // Comments
            opus_comr: None,                   // Commercial frame
            opus_encr: None,                   // Encryption method registration
            opus_equ2: None,                   // Equalisation (2)
            opus_etco: None,                   // Event timing codes
            opus_geob: None,                   // General encapsulated object
            opus_grid: None,                   // Group identification registration
            opus_link: None,                   // Linked information
            opus_mcdi: None,                   // Music CD identifier
            opus_mllt: None,                   // MPEG location lookup table
            opus_owne: None,                   // Ownership frame
            opus_priv: None,                   // Private frame
            opus_pcnt: None,                   // Play counter
            opus_popm: None,                   // Popularimeter
            opus_poss: None,                   // Position synchronisation frame
            opus_rbuf: None,                   // Recommended buffer size
            opus_rva2: None,                   // Relative volume adjustment (2)
            opus_rvrb: None,                   // Reverb
            opus_seek: None,                   // Seek frame
            opus_sign: None,                   // Signature frame
            opus_sylt: None,                   // Synchronised lyric/text
            opus_sytc: None,                   // Synchronised tempo codes
            opus_talb: None,                   // Album/Movie/Show title
            opus_tbpm: None,                   // BPM (beats per minute)
            opus_tcom: None,                   // Composer
            opus_tcon: None,                   // Content type
            opus_tcop: None,                   // Copyright message
            opus_tden: None,                   // Encoding time
            opus_tdly: None,                   // Playlist delay
            opus_tdor: None,                   // Original release time
            opus_tdrc: None,                   // Recording time
            opus_tdrl: None,                   // Release time
            opus_tdtg: None,                   // Tagging time
            opus_tenc: None,                   // Encoded by
            opus_text: None,                   // Lyricist/Text writer
            opus_tflt: None,                   // File type
            opus_tipl: None,                   // Involved people list
            opus_tit1: None,                   // Content group description
            opus_tit2: "NO TITLE".to_string(), // Title/songname/content description // ???: Do we want this to fail?
            opus_tit3: None,                   // Subtitle/Description refinement
            opus_tkey: None,                   // Initial key
            opus_tlan: None,                   // Language(s)
            opus_tlen: None,                   // Length
            opus_tmcl: None,                   // Musician credits list
            opus_tmed: None,                   // Media type
            opus_tmoo: None,                   // Mood
            opus_toal: None,                   // Original album/movie/show title
            opus_tofn: None,                   // Original filename
            opus_toly: None,                   // Original lyricist(s)/text writer(s)
            opus_tope: None,                   // Original artist(s)/performer(s)
            opus_town: None,                   // File owner/licensee
            opus_tpe1: None,                   // Lead performer(s)/Soloist(s)
            opus_tpe2: None,                   // Band/orchestra/accompaniment
            opus_tpe3: None,                   // Conductor/performer refinement
            opus_tpe4: None,                   // Interpreted, remixed, or otherwise modified by
            opus_tpos: None,                   // Part of a set
            opus_tpro: None,                   // Produced notice
            opus_tpub: None,                   // Publisher
            opus_trck: None,                   // Track number/Position in set
            opus_trsn: None,                   // Internet radio station name
            opus_trso: None,                   // Internet radio station owner
            opus_tsoa: None,                   // Album sort order
            opus_tsop: None,                   // Performer sort order
            opus_tsot: None,                   // Title sort order
            opus_tsrc: None,                   // ISRC (international standard recording code)
            opus_tsse: None,                   // Software/Hardware and settings used for encoding
            opus_tsst: None,                   // Set subtitle
            opus_txxx: None,                   // User defined text information frame
            opus_ufid: None,                   // Unique file identifier
            opus_user: None,                   // Terms of use
            opus_uslt: None,                   // Unsynchronised lyric/text transcription
            opus_wcom: None,                   // Commercial information
            opus_wcop: None,                   // Copyright/Legal information
            opus_woaf: None,                   // Official audio file webpage
            opus_woar: None,                   // Official artist/performer webpage
            opus_woas: None,                   // Official audio source webpage
            opus_wors: None,                   // Official Internet radio station homepage
            opus_wpay: None,                   // Payment
            opus_wpub: None,                   // Publishers official webpage
            opus_wxxx: None,                   // User defined URL link frame
        }
    }
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

pub trait Component {
    fn play() -> OpResult;
    fn pause() -> OpResult;
    fn stop() -> OpResult;
    fn status() -> OpResult;
    fn load(opus: OpusId) -> OpResult;
    fn clear() -> OpResult;
    fn get_playables_menu() -> Result<String, OpComponentError>;
    // .get_playables_json() -> Result<String, E>
}

pub trait Playable {
    // fn play() -> OpResult;
    // fn pause() -> OpResult;
    // fn stop() -> OpResult;
    fn toggle_repeat() -> OpResult;
    fn set_repeat(status:bool) -> OpResult;
    fn toggle_random() -> OpResult;
    fn set_random(status:bool) -> OpResult;
    // .get_playables_json() -> Result<String, E>
}

// Errors

pub enum OpComponentError {
    Load,
    Play,
    Pause,
    
}
