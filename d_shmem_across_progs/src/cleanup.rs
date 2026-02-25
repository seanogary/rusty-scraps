use std::env;
use nix::sys::mman::{shm_unlink};

fn main() {
    let args: Vec<String> = env::args().collect();
    let shared_mem_name = args[1].as_str();
    let _ = shm_unlink(shared_mem_name);
    println!("Clean shared memory {}", shared_mem_name);
}
