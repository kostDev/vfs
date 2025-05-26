use std::env;
use std::fs;
use crate::proto::{ VfsCommand, VfsResponse};
use crate::ipc::send_command;


fn print_help() {
    eprintln!("Використання: vfs <команда> [аргументи]");
    eprintln!("Доступні команди:");
    eprintln!("  ls                          - список файлів");
    eprintln!("  cat <файл>                 - вивести вміст файлу");
    eprintln!("  add <файл>                 - додати файл у VFS");
    eprintln!("  create <шлях>              - створити нову VFS");
    eprintln!("  load <шлях>                - завантажити VFS");
    eprintln!("  save <шлях>                - зберегти VFS");
}

pub fn handle_cli() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "create" => {
            if args.len() < 3 {
                eprintln!("Використання: vfs create <шлях до файлу>");
                return;
            }
            let path = args[2].clone();
            match send_command(VfsCommand::CreateVolume { path }) {
                Ok(VfsResponse::Ok(msg)) => println!("{}", msg),
                Ok(VfsResponse::Error(msg)) => eprintln!("{}", msg),
                Ok(resp) => eprintln!("Несподівана відповідь: {:?}", resp),
                Err(e) => eprintln!("Помилка з'єднання: {}", e),
            }
        }
        "load" => {
            if args.len() < 3 {
                eprintln!("Використання: vfs load <шлях до файлу>");
                return;
            }
            let path = args[2].clone();
            match send_command(VfsCommand::LoadVolume { path }) {
                Ok(VfsResponse::Ok(msg)) => println!("{}", msg),
                Ok(VfsResponse::Error(msg)) => eprintln!("{}", msg),
                Ok(resp) => eprintln!("Несподівана відповідь: {:?}", resp),
                Err(e) => eprintln!("Помилка з'єднання: {}", e),
            }
        }
        "save" => {
            let path = &args[2];
            match send_command(VfsCommand::SaveVolume { path: path.clone() }) {
                Ok(VfsResponse::Ok(msg)) => println!("{}", msg),
                Ok(VfsResponse::Error(msg)) => eprintln!("{}", msg),
                Ok(resp) => eprintln!("Несподівана відповідь: {:?}", resp),
                Err(e) => eprintln!("Помилка з'єднання: {}", e),
            }
        }
        "ls" => {
            match send_command(VfsCommand::ListFiles) {
                Ok(VfsResponse::FileList(files)) => {
                    for file in files {
                        println!("- {}", file);
                    }
                }
                Ok(resp) => eprintln!("Несподівана відповідь: {:?}", resp),
                Err(e) => eprintln!("Помилка з'єднання: {}", e),
            }
        }
        "cat" => {
            if args.len() < 3 {
                eprintln!("Використання: vfs cat <файл>");
                return;
            }
            let name = &args[2];
            match send_command(VfsCommand::ReadFile { name: name.clone() }) {
                Ok(VfsResponse::FileContent(content)) => {
                    println!("{}", String::from_utf8_lossy(&content));
                }
                Ok(VfsResponse::Error(msg)) => eprintln!("{}", msg),
                Ok(resp) => eprintln!("Несподівана відповідь: {:?}", resp),
                Err(e) => eprintln!("Помилка з'єднання: {}", e),
            }
        }
        "add" => {
            if args.len() < 3 {
                eprintln!("Використання: vfs add <файл>");
                return;
            }
            let filename = &args[2];
            match fs::read(filename) {
                Ok(content) => {
                    let cmd = VfsCommand::AddFile { name: filename.clone(), content };
                    match send_command(cmd) {
                        Ok(VfsResponse::Ok(msg)) => println!("{}", msg),
                        Ok(VfsResponse::Error(msg)) => eprintln!("{}", msg),
                        Ok(resp) => eprintln!("Несподівана відповідь: {:?}", resp),
                        Err(e) => eprintln!("Помилка з'єднання: {}", e),
                    }
                }
                Err(err) => eprintln!("Помилка читання '{}': {}", filename, err),
            }
        }
        _ => {
            eprintln!("Невідома команда: {}", args[1]);
        }
    }
}