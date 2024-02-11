pub mod text;

use bootloader_api::info::FrameBuffer;

use crate::*;
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;

pub static LOGGER: OnceCell<bootloader_x86_64_common::logger::LockedLogger> = OnceCell::uninit();

pub fn init_framebuffer_utils(framebuffer: &'static mut FrameBuffer) {
    for byte in framebuffer.buffer_mut() {
        *byte = 0x90;
    }
    let frame_buffer_info = framebuffer.info().clone();

    // get the framebuffer's mutable raw byte slice
    let raw_frame_buffer = framebuffer.buffer_mut();

    let logger = LOGGER
        .get_or_init(move || LockedLogger::new(raw_frame_buffer, frame_buffer_info, true, false));
    log::set_logger(logger);
    log::set_max_level(LevelFilter::Trace);
    info!("Framebuffer is inited");
}
