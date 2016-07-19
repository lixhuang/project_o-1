#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![no_std]

#[macro_use]
mod vga_buffer;

extern crate rlibc;
extern crate spin;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: We have a very small stack without guard page
/*
    let hello = b"Hello World!";
    let color_byte = 0x1f;

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // Write 'Hello World!' to the center of the VGA text buffer
    //let buffer_ptr = (0xb8000 + 1988) as *mut _;
    //unsafe { *buffer_ptr = hello_colored };
*/
	//use core::fmt::Write;
	//write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337);
    //vga_buffer::print_smth();
    vga_buffer::clear_screen();
    println!("Hello {}!", "world");
    print!("yeah!");
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
