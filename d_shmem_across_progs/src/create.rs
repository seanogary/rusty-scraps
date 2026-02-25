use std::env;
use nix::sys::mman::{shm_open};
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use nix::unistd::ftruncate;

fn main() {
    let args: Vec<String> = env::args().collect();
    let shared_mem_name = args[1].as_str();

    let fd = shm_open(
        shared_mem_name,
        OFlag::O_CREAT | OFlag::O_RDWR,
        Mode::S_IWUSR | Mode::S_IRUSR
    ).unwrap();
    ftruncate(&fd, 4096).unwrap();

    println!("Created 4096 byte shared memory named {}.", shared_mem_name);

}