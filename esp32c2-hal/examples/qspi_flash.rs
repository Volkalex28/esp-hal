//! SPI write and read a flash chip
//!
//! Folowing pins are used:
//! SCLK            GPIO4
//! MISOI/IO0       GPIO5
//! MOSI/IO1        GPIO6
//! IO2             GPIO7
//! IO3             GPIO8
//! CS              GPIO9
//!
//! Depending on your target and the board you are using you have to change the
//! pins.
//!
//! Connect a flash chip (GD25Q64C was used) and make sure QE in the status
//! register is set.

#![no_std]
#![no_main]

use esp32c2_hal::{
    clock::ClockControl,
    dma::{Dma, DmaPriority},
    dma_buffers,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    spi::{
        master::{prelude::*, Address, Command, Spi},
        SpiDataMode,
        SpiMode,
    },
    Delay,
};
use esp_backtrace as _;
use esp_println::{print, println};

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
    let sio3 = io.pins.gpio8;
    let cs = io.pins.gpio9;

    let dma = Dma::new(peripherals.DMA);
    let dma_channel = dma.channel0;

    let (tx_buffer, mut tx_descriptors, rx_buffer, mut rx_descriptors) = dma_buffers!(256, 320);

    let mut spi = Spi::new_half_duplex(peripherals.SPI2, 100u32.kHz(), SpiMode::Mode0, &clocks)
        .with_pins(
            Some(sclk),
            Some(mosi),
            Some(miso),
            Some(sio2),
            Some(sio3),
            Some(cs),
        )
        .with_dma(dma_channel.configure(
            false,
            &mut tx_descriptors,
            &mut rx_descriptors,
            DmaPriority::Priority0,
        ));

    let mut delay = Delay::new(&clocks);

    // DMA buffer require a static life-time
    let mut zero_buf = zero_buffer();
    let send = tx_buffer;
    let mut receive = rx_buffer;

    // write enable
    let transfer = spi
        .write(
            SpiDataMode::Single,
            Command::Command8(0x06, SpiDataMode::Single),
            Address::None,
            0,
            zero_buf,
        )
        .unwrap();
    (zero_buf, spi) = transfer.wait().unwrap();
    delay.delay_ms(250u32);

    // erase sector
    let transfer = spi
        .write(
            SpiDataMode::Single,
            Command::Command8(0x20, SpiDataMode::Single),
            Address::Address24(0x000000, SpiDataMode::Single),
            0,
            zero_buf,
        )
        .unwrap();
    (zero_buf, spi) = transfer.wait().unwrap();
    delay.delay_ms(250u32);

    // write enable
    let transfer = spi
        .write(
            SpiDataMode::Single,
            Command::Command8(0x06, SpiDataMode::Single),
            Address::None,
            0,
            zero_buf,
        )
        .unwrap();
    (_, spi) = transfer.wait().unwrap();
    delay.delay_ms(250u32);

    // write data / program page
    send.fill(b'!');
    send[0..][..5].copy_from_slice(&b"Hello"[..]);
    let transfer = spi
        .write(
            SpiDataMode::Quad,
            Command::Command8(0x32, SpiDataMode::Single),
            Address::Address24(0x000000, SpiDataMode::Single),
            0,
            send,
        )
        .unwrap();
    (_, spi) = transfer.wait().unwrap();
    delay.delay_ms(250u32);

    loop {
        // quad fast read
        let transfer = spi
            .read(
                SpiDataMode::Quad,
                Command::Command8(0xeb, SpiDataMode::Single),
                Address::Address32(0x000000 << 8, SpiDataMode::Quad),
                4,
                receive,
            )
            .unwrap();

        // here we could do something else while DMA transfer is in progress
        // the buffers and spi is moved into the transfer and we can get it back via
        // `wait`
        (receive, spi) = transfer.wait().unwrap();

        println!("{:x?}", &receive);
        for b in &mut receive.iter() {
            if *b >= 32 && *b <= 127 {
                print!("{}", *b as char);
            } else {
                print!(".");
            }
        }
        println!();

        delay.delay_ms(250u32);
    }
}

fn zero_buffer() -> &'static mut [u8; 0] {
    static mut BUFFER: [u8; 0] = [0u8; 0];
    unsafe { &mut BUFFER }
}
