
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SequenceData {
    pub name: String,
    // layoutid: u32,
    // music_file: Option<String>,
    pub data: Vec<Vec<u16>>
}


