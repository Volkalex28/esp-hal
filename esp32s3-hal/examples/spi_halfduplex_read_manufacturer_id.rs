//! SPI read manufacturer id from flash chip
//!
//! Folowing pins are used:
//! SCLK            GPIO4
//! MISO/IO0        GPIO5
//! MOSI/IO1        GPIO6
//! IO2             GPIO7
//! IO3             GPIO15
//! CS              GPIO16
//!
//! Depending on your target and the board you are using you have to change the
//! pins.
//!
//! Connect a flash chip (GD25Q64C was used) and make sure QE in the status
//! register is set.

#![no_std]
#![no_main]

use esp32s3_hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    spi::{
        master::{Address, Command, HalfDuplexReadWrite, Spi},
        SpiDataMode,
        SpiMode,
    },
    Delay,
};
use esp_backtrace as _;
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let sclk = io.pins.gpio4;
    let miso = io.pins.gpio5;
    let mosi = io.pins.gpio6;
    let sio2 = io.pins.gpio7;
    let sio3 = io.pins.gpio15;
    let cs = io.pins.gpio16;

    let mut spi = Spi::new_half_duplex(peripherals.SPI2, 100u32.kHz(), SpiMode::Mode0, &clocks)
        .with_pins(
            Some(sclk),
            Some(mosi),
            Some(miso),
            Some(sio2),
            Some(sio3),
            Some(cs),
        );

    let mut delay = Delay::new(&clocks);

    loop {
        // READ MANUFACTURER ID FROM FLASH CHIP
        let mut data = [0u8; 2];
        spi.read(
            SpiDataMode::Single,
            Command::Command8(0x90, SpiDataMode::Single),
            Address::Address24(0x000000, SpiDataMode::Single),
            0,
            &mut data,
        )
        .unwrap();
        println!("Single {:x?}", data);
        delay.delay_ms(250u32);

        // READ MANUFACTURER ID FROM FLASH CHIP
        let mut data = [0u8; 2];
        spi.read(
            SpiDataMode::Dual,
            Command::Command8(0x92, SpiDataMode::Single),
            Address::Address32(0x000000_00, SpiDataMode::Dual),
            0,
            &mut data,
        )
        .unwrap();
        println!("Dual {:x?}", data);
        delay.delay_ms(250u32);

        // READ MANUFACTURER ID FROM FLASH CHIP
        let mut data = [0u8; 2];
        spi.read(
            SpiDataMode::Quad,
            Command::Command8(0x94, SpiDataMode::Single),
            Address::Address32(0x000000_00, SpiDataMode::Quad),
            4,
            &mut data,
        )
        .unwrap();
        println!("Quad {:x?}", data);
        delay.delay_ms(1500u32);
    }
}
