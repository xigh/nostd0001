#![allow(internal_features)]

#![feature(lang_items)]

#![no_std]

#![no_main]

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
