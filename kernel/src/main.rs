#![no_main]
#![no_std]

use bootloader_api::entry_point;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    kernel::init(boot_info);
    log::info!("Starting loop");
    kernel::hlt_loop()
}

use core::{borrow::BorrowMut, panic::PanicInfo};
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::panic_handler(info)
}
