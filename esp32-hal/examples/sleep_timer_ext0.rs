//! Demonstrates deep sleep with timer and ext0 (using gpio27) wakeup

#![no_std]
#![no_main]

use core::time::Duration;

use esp32_hal::{
    clock::ClockControl,
    entry,
    peripherals::Peripherals,
    prelude::*,
    rtc_cntl::{
        get_reset_reason,
        get_wakeup_cause,
        sleep::{Ext0WakeupSource, TimerWakeupSource, WakeupLevel},
        SocResetReason,
    },
    Cpu,
    Delay,
    Rtc,
    IO,
};
use esp_backtrace as _;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.LPWR);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut ext0_pin = io.pins.gpio27;

    println!("up and runnning!");
    let reason = get_reset_reason(Cpu::ProCpu).unwrap_or(SocResetReason::ChipPowerOn);
    println!("reset reason: {:?}", reason);
    let wake_reason = get_wakeup_cause();
    println!("wake reason: {:?}", wake_reason);

    let mut delay = Delay::new(&clocks);

    let timer = TimerWakeupSource::new(Duration::from_secs(30));
    let ext0 = Ext0WakeupSource::new(&mut ext0_pin, WakeupLevel::High);
    println!("sleeping!");
    delay.delay_ms(100u32);
    rtc.sleep_deep(&[&timer, &ext0], &mut delay);
}
