mod vfs;
mod proto;

use std::os::unix::net::UnixListener;
use std::io::{Read, Write};
use std::fs;
use std::path::Path;

use crate::vfs::VirtualFileSystem;
use crate::proto::{VfsCommand, VfsResponse};

const SOCKET_PATH: &str = "/tmp/vfs.sock";

fn main() -> std::io::Result<()> {
    if Path::new(SOCKET_PATH).exists() {
        fs::remove_file(SOCKET_PATH)?;
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;
    println!("[vfsd] socket open: {}", SOCKET_PATH);

    let mut vfs = VirtualFileSystem::new();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = Vec::new();
                if stream.read_to_end(&mut buffer).is_ok() {
                    if let Ok(cmd) = bincode::decode_from_slice::<VfsCommand, _>(&buffer, bincode::config::standard()) {
                        let (command, _) = cmd;
                        let response = unsafe { handle_command(&mut vfs, command) };
                        let encoded = bincode::encode_to_vec(&response, bincode::config::standard()).unwrap();
                        let _ = stream.write_all(&encoded);
                    }
                }
            }
            Err(e) => eprintln!("[vfsd] Error connection: {}", e),
        }
    }

    Ok(())
}

unsafe fn handle_command(vfs: &mut VirtualFileSystem, cmd: VfsCommand) -> VfsResponse {
    match cmd {
        // volume op
        VfsCommand::CreateVolume { path } => {
            match vfs.create_volume(&path) {
                Ok(_) =>  VfsResponse::Ok(format!("Створено нову VFS у '{}'.", path)),
                Err(e) => VfsResponse::Error(format!("Помилка створення: {}", e)),
            }
        }

        VfsCommand::LoadVolume { path } => {
            match vfs.load_volume(&path) {
                Ok(_) => VfsResponse::Ok("Завантажено".to_string()),
                Err(e) => VfsResponse::Error(format!("Помилка завантаження: {}", e)),
            }
        }

        VfsCommand::SaveVolume { path } => {
            match vfs.save_volume(&path) {
                Ok(_) => VfsResponse::Ok("Збережено".to_string()),
                Err(e) => VfsResponse::Error(format!("Помилка збереження: {}", e)),
            }
        }
        // files op
        VfsCommand::AddFile { name, content } => {
            vfs.add_file(&name, &content);
            VfsResponse::Ok(format!("Файл '{}' додано", name))
        }
        VfsCommand::ReadFile { name } => {
            match vfs.read_file(&name) {
                Some(content) => VfsResponse::FileContent(content.to_vec()),
                None => VfsResponse::Error(format!("Файл '{}' не знайдено", name)),
            }
        }
        VfsCommand::ListFiles => {
            let list = vfs
                .list_files()
                .iter()
                .map(|s| s.to_string())
                .collect();
            VfsResponse::FileList(list)
        }

    }
}