use nix::sys::mman::{shm_open, mmap, shm_unlink, MapFlags, ProtFlags};
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use nix::unistd::ftruncate;
use std::num::NonZeroUsize;

fn main() {
    let _ = shm_unlink("myshmem");

    let fd = match shm_open(
        "myshmem",
        OFlag::O_CREAT | OFlag::O_RDWR,
        Mode::S_IWUSR | Mode::S_IRUSR
    ) {
        Ok(v) => v,
        Err(_) => panic!("Panic!"),
    };

    match ftruncate(&fd, 4096) {
        Ok(_) => {},
        Err(e) => panic!("Panic: {}", e),
    };

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
        let wptr = ptr.as_ptr() as *mut u8;
        wptr.write(42);
    }

    unsafe {
        let ptr = ptr.as_ptr() as *const u8;
        let value = ptr.read();
        println!("{}", value);
    }

}
