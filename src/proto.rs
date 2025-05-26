use bincode::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub enum VfsCommand {
    CreateVolume { path: String },
    LoadVolume { path: String },
    SaveVolume { path: String },
    AddFile { name: String, content: Vec<u8> },
    ReadFile { name: String },
    ListFiles,
}

#[derive(Debug, Encode, Decode)]
pub enum VfsResponse {
    Ok(String),
    Error(String),
    FileList(Vec<String>),
    FileContent(Vec<u8>),
}