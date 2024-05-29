extern crate libc;

use libc::{mmap, munmap, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::error::Error;
use std::ptr;

use pointer_talk::perf_metrics::{get_page_faults, VirtualAddress};

fn main() -> Result<(), Box<dyn Error>> {
    let page_size = 1024 * 16;
    // let page_size = 4096;
    let page_count = 16384;
    // let page_count = 1000;
    let total_size = page_size * page_count;
    let pid = std::process::id() as i32;

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
        return Ok(());
    }

    let mut prior_over_fault_count: i32 = 0;
    let mut prior_page_index: usize = 0;

    let start_faults_count = get_page_faults(pid);
    unsafe {
        let byte_slice = std::slice::from_raw_parts_mut(addr as *mut u8, total_size);

        for page_index in 0..page_count {
            let write_index = page_size * page_index;
            byte_slice[write_index] = page_index as u8;
            let end_faults_count = get_page_faults(pid);

            let over_fault_count = end_faults_count - start_faults_count;
            if over_fault_count > prior_over_fault_count {
                println!(
                    "Page {}: {} extra faults ({} page size since last PF)",
                    page_index,
                    over_fault_count,
                    page_index - prior_page_index
                );

                if page_index > 0 {
                    let vaddr =
                        VirtualAddress::from_pointer(addr as usize + page_size * prior_page_index);
                    println!("    Previous Pointer: {}", vaddr.format());
                }

                // let vaddr = VirtualAddress::from_pointer(addr as usize + page_size * page_index);
                let vaddr = VirtualAddress::from_pointer(addr as usize + write_index);
                println!("        This Pointer: {}", vaddr.format());

                prior_over_fault_count = over_fault_count;
                prior_page_index = page_index;
            }
        }

        munmap(addr, total_size);
    }

    Ok(())
}
