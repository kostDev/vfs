mod cli;
mod ipc;
mod proto;

use crate::cli::handle_cli;
// static WORK_FILE: &str = "./fs/data.vfs";

fn main() {
    // let mut fs = VirtualFileSystem::new();
    // fs.add_file("hello.txt", "Привіт, світ!".as_bytes());
    // fs.log_list_info();
    // fs.show_file("hello.txt");
    // fs.log_list_info();
    // fs.show_file("test.txt");
    // fs.save_to_file(WORK_FILE).unwrap();
    // fs.load_from_file(WORK_FILE).unwrap();
    // fs.log_list_info();
    handle_cli();
}