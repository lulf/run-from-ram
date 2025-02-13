#![no_std]
#![no_main]

use core::arch::asm;

use cortex_m_rt::{entry, exception};

// NOTE: Required to put __pre_init in flash and not RAM
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = ".vector_table_flash"))]
unsafe extern "C" fn __pre_init() {}

#[entry]
unsafe fn main() -> ! {
    unsafe {
        let current_pc: u32;
        asm!("mov {}, pc", out(reg) current_pc);
        unsafe extern "C" {
            static _ram_start: u32;
        }
        let ram_start = &_ram_start as *const u32 as u32;
        // Verify that we're running from RAM
        assert!(current_pc >= ram_start);
        blinky_debug(100);
        loop {}
    }
}

#[allow(unused)]
unsafe fn blinky_debug(blinks: usize) {
    use embassy_nrf::gpio::{Level, Output, OutputDrive};
    let mut led = Output::new(
        embassy_nrf::peripherals::P0_14::steal(),
        Level::High,
        OutputDrive::Standard,
    );
    let mut n = 0;
    while n < blinks {
        led.set_low();
        for i in 0..1000000 {
            cortex_m::asm::nop();
        }
        led.set_high();
        for i in 0..1000000 {
            cortex_m::asm::nop();
        }
        n += 1;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        asm!("udf #0");
        core::hint::unreachable_unchecked();
    }
}

#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = ".HardFault.user"))]
unsafe extern "C" fn HardFault() {
    cortex_m::peripheral::SCB::sys_reset();
}

#[exception]
unsafe fn DefaultHandler(_: i16) -> ! {
    const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32;
    let irqn = core::ptr::read_volatile(SCB_ICSR) as u8 as i16 - 16;

    panic!("DefaultHandler #{:?}", irqn);
}
