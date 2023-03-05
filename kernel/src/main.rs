#![no_std]
#![no_main]

mod vga_buff;

extern crate panic_halt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main()
}

pub fn kernel_main() -> ! {
    println!("We've been Fucked");
    loop {}
}
