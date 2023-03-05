#![no_std] // We can't use std lib if we wanna write bare-metal(low lv) software
#![no_main] // We gonna use no_main macro so we can use whatever func we want as our entry point

mod vga_buff; // import vga text mode module

extern crate panic_halt; // without std libs no panic handler so we gonna use panic_halt crate but we can make our own with panic_handler macro

#[no_mangle] // to disable standard symbol name mangling
             // extern "C" make function adhere to the C calling convention
             // return type ! mean there is no return
pub extern "C" fn kernel_main() -> ! {
    println!("We've been {}", "Fucked"); // Yea I know println macro is in std lib it's right but I made my own macro in vga_buff
    loop {} // in bare-metal there is no end(stop) so it's done loop forever
}
