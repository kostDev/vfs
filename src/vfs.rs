use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::HashMap;
use chrono::{DateTime, Local};
use bincode::config::standard;
use bincode::{Encode, Decode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct VfsFile {
    pub name: String,
    pub content: Vec<u8>,
    pub size: u64,
    pub created_ts: u64,
}

impl VfsFile {
    pub fn created(&self) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(self.created_ts)
    }
}

#[derive(Debug, Default)]
pub struct VirtualFileSystem {
    files: HashMap<String, VfsFile>,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
    // volume: create_volume, load_volume, save_volume

    pub fn create_volume(&mut self, path: &str) -> std::io::Result<()> {
        let path_obj = Path::new(path);

        if let Some(parent_dir) = path_obj.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let file = std::fs::File::create(path)?; // створити порожній файл
        let encoded = bincode::encode_to_vec(&self.files, standard()).unwrap();
        std::io::Write::write_all(&mut std::io::BufWriter::new(file), &encoded)?;

        Ok(())
    }

    pub fn save_volume(&mut self, path: &str) -> std::io::Result<()> {
        let encoded = bincode::encode_to_vec(&self.files, standard()).unwrap();
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load_volume(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let (decoded, _): (HashMap<String, VfsFile>, _) = bincode::decode_from_slice(&buffer, standard()).unwrap();
        self.files = decoded;
        Ok(())
    }

    pub fn add_file(&mut self, name: &str, content: &[u8]) {
        let file = VfsFile {
            name: name.to_string(),
            content: content.to_vec(),
            size: content.len() as u64,
            created_ts: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };
        self.files.insert(name.to_string(), file);
    }

    pub fn read_file(&self, name: &str) -> Option<&[u8]> {
        self.files.get(name).map(|f| f.content.as_slice())
    }

    pub fn show_file(&mut self, name: &str) {
        if let Some(content) = self.read_file(name) {
            println!("\n{}: {}", name, String::from_utf8_lossy(content));
        }
    }

    pub fn get_file_size(&self, name: &str) -> Option<u64> {
        self.files.get(name).map(|f| f.size)
    }

    pub fn get_file_time_string(&self, name: &str) -> Option<String> {
        self.files.get(name).map(|f| {
            let datetime: DateTime<Local> = f.created().into();
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    pub fn list_files(&self) -> Vec<&String> {
        self.files.keys().collect()
    }

    pub fn log_list_info(&self) {
        println!("{:<20} │ {:>10} │ {:<20}", "Файл", "Розмір", "Час створення");
        println!("{}", "─".repeat(56));

        for name in self.list_files() {
            let size = self.get_file_size(name).unwrap_or(0);
            let time = self.get_file_time_string(name).unwrap_or("???".to_string());

            println!("{:<20} │ {:>10} │ {:<20}", name, format!("{} B", size), time);
        }
    }

    pub fn delete_file(&mut self, name: &str) -> bool {
        self.files.remove(name).is_some()
    }
}