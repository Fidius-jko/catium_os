pub mod x86_64;

pub fn init() {
    #[cfg(target_arch = "x86_64")]
    {
        log::info!("Initing idt");
        x86_64::idt::init_idt();
        log::info!("Initing GDT");
        x86_64::gdt::init();
        unsafe { x86_64::idt::PICS.lock().initialize() };
        crate::x86_64EX::instructions::interrupts::enable();
        log::info!("Initing done!");
    }
}
