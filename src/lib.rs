#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

use multiboot2::BootInformation;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    print_initial_message();

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    print_available_memory_regions(&boot_info);
    print_kernel_elf64_sections(&boot_info);
    print_kernel_elf64_range(&boot_info);
    println!("multiboot_start: 0x{:x}, multiboot_end: 0x{:x}",
             multiboot_information_address, multiboot_information_address + (boot_info.total_size as usize));

    loop {}
}

fn print_initial_message() {
    vga_buffer::clear_screen();
    println!("Hello {}!", "world.");
}

fn print_available_memory_regions(boot_info: &BootInformation) {
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required.");

    println!("available memory regions:");
    for area in memory_map_tag.memory_areas() {
        println!("   start: 0x{:x}, length 0x{:x}", area.base_addr, area.length);
    }
}

fn print_kernel_elf64_sections(boot_info: &BootInformation) {
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required.");

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        let write_flag   = if section.flags & 0x1 != 0 { 'w' } else { '-' };
        let read_flag    = if section.flags & 0x2 != 0 { 'r' } else { '-' };
        let execute_flag = if section.flags & 0x4 != 0 { 'x' } else { '-' };
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: {}{}{}",
                 section.addr, section.size, write_flag, read_flag, execute_flag);
    }
}

fn print_kernel_elf64_range(boot_info: &BootInformation) {
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required.");
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr).max().unwrap();

    println!("kernel_start: 0x{:x}, kernel_end: 0x{:x}", kernel_start, kernel_end);
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("   {}", fmt);
    loop {}
}
