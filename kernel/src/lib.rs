#![feature(abi_x86_interrupt)]
#![no_std]

use log::*;
pub use x86_64 as x86_64EX;

mod arch;
mod framebuffer;

pub fn init(boot_info: &'static mut bootloader_api::BootInfo) {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        framebuffer::init_framebuffer_utils(framebuffer);
    } else {
        panic!("Framebuffer is must for logging!");
    }

    arch::init();

    info!("Kernel is init");
}

use core::panic::PanicInfo;

pub fn panic_handler(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
