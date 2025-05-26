use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
use std::path::Path;
use crate::proto::{VfsCommand, VfsResponse};
use bincode::{encode_to_vec, decode_from_slice};
use bincode::config::standard;

const SOCKET_PATH: &str = "/tmp/vfs.sock";

pub fn send_command(cmd: VfsCommand) -> std::io::Result<VfsResponse> {
    if !Path::new(SOCKET_PATH).exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Сокет не знайдено. Запусти vfsd."));
    }

    let mut stream = UnixStream::connect(SOCKET_PATH)?;

    let encoded = encode_to_vec(&cmd, standard())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Помилка серіалізації: {}", e)))?;
    stream.write_all(&encoded)?;
    stream.shutdown(std::net::Shutdown::Write).ok();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    let (resp, _): (VfsResponse, _) = decode_from_slice(&buffer, standard())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Помилка десеріалізації: {}", e)))?;

    Ok(resp)
}