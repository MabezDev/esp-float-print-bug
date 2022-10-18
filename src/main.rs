#![no_std]
#![no_main]

use esp32s2_hal::{clock::ClockControl, pac::Peripherals, prelude::*, timer::TimerGroup, Rtc};
use esp_backtrace as _;
use xtensa_atomic_emulation_trap as _;
use esp_println::println;

#[xtensa_lx_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // this works!
    let v = 16.0f64;
    println!("16.0f64 literal is={v}");

    // any of these crash when opt-level > 0
    let v = 16.0f32;
    println!("16.0f32 literal is={v}");
    let v = 3.14f64;
    println!("3.14f64 literal is={v}");
    let v = 3.14f32;
    println!("3.14f32 literal is={v}");

    loop {}
}
