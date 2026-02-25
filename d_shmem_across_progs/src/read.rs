use std::env;
use nix::sys::mman::{shm_open, mmap, MapFlags, ProtFlags};
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use std::num::NonZeroUsize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let shared_mem_name = args[1].as_str();

    let fd = shm_open(
        shared_mem_name,
        OFlag::O_RDWR,
        Mode::S_IWUSR | Mode::S_IRUSR
    ).unwrap();

    let ptr = match unsafe {
        mmap(
            None,
            NonZeroUsize::new(4096).unwrap(),
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            fd,
            0,
        )
    } {
        Ok(v) => v,
        Err(e) => panic!("Panic: {}", e),
    };

unsafe {
    let ptr = ptr.as_ptr() as *const u8;
    let slice = std::slice::from_raw_parts(ptr, 4096);
    let end = slice.iter().position(|&b| b == 0).unwrap_or(4096);
    let s = std::str::from_utf8(&slice[..end]).unwrap();
    println!("{}", s);
}

}