
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SequenceData {
    pub name: String,
    pub music_file: Option<String>,
    pub frame_dur_ms: u32,
    pub num_frames: u32,
    pub data: Vec<Vec<u16>> // Received in column-major order, stored in frame-major order
}
