// mod vfs;
mod cli;
mod ipc;
mod proto;
// mod vfsd;
// mod proto;
// mod ipc;

use crate::cli::handle_cli;
// use crate::vfs::VirtualFileSystem;

// static WORK_FILE: &str = "./fs/data.vfs";

// Пример використання
fn main() {
    // let mut fs = VirtualFileSystem::new();
    // fs.add_file("hello.txt", "Привіт, світ!".as_bytes());
    // fs.add_file("log.txt", "Це лог файл".as_bytes());

    // fs.log_list_info();
    //
    // fs.show_file("hello.txt");
    handle_cli();
    // fs.log_list_info();
    // fs.show_file("test.txt");
    // fs.save_to_file(WORK_FILE).unwrap();
    // fs.load_from_file(WORK_FILE).unwrap();
    // fs.log_list_info();
}