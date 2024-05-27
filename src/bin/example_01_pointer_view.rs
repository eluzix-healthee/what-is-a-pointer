use libc::{mmap, munmap, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use pointer_talk::perf_metrics::VirtualAddress;
use std::error::Error;
use std::ptr;

fn allocate_and_print_pointer() {
    let total_size = 1024 * 1024 * 1024;

    let addr = unsafe {
        mmap(
            ptr::null_mut(),
            total_size,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANON,
            -1,
            0,
        )
    };

    if addr == MAP_FAILED {
        eprintln!("mmap failed");
        return;
    }

    let va = VirtualAddress::from_pointer(addr as usize);
    va.print();

    unsafe { munmap(addr, total_size) };
}

fn main() -> Result<(), Box<dyn Error>> {
    allocate_and_print_pointer();
    allocate_and_print_pointer();
    allocate_and_print_pointer();

    Ok(())
}
