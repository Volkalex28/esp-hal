# Changelog

All notable changes to this project will be documented in this file.

Please note that only changes to the `esp-hal-common` package are tracked in this CHANGELOG.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add initial support for the ESP32-P4 (#1101)
- Implement `embedded_hal::pwm::SetDutyCycle` trait for `ledc::channel::Channel` (#1097) 
- ESP32-P4: Add initial GPIO support (#1109)
- ESP32-P4: Add initial support for interrupts (#1112)
- ESP32-P4: Add efuse reading support (#1114)
- ESP32-S3: Added LCD_CAM I8080 driver (#1086)
- Allow for splitting of the USB Serial JTAG peripheral into tx/rx components (#1024)
- `RngCore` trait is implemented (#1122)
- Support Rust's `stack-protector` feature (#1135)

### Fixed

- Fix embassy-time tick rate not set when using systick as the embassy timebase (#1124)
- Fix `get_raw_core` on Xtensa (#1126)
- Fix docs.rs documentation builds (#1129)

### Changed

- DmaDescriptor struct to better model the hardware (#1054)
- DMA descriptor count no longer needs to be multiplied by 3 (#1054)
- RMT channels no longer take the channel number as a generic param (#959)
- The `esp-hal-common` package is now called `esp-hal` (#1131)
- Refactor the `Trace` driver to be generic around its peripheral (#1140)

### Removed

### Breaking

- `ADC` and `DAC` drivers now take virtual peripherals in their constructors, instead of splitting `APB_SARADC`/`SENS` (#1100)
- The `DAC` driver's constructor is now `new` instead of `dac`, to be more consistent with other APIs (#1100)
- The DMA peripheral is now called `Dma` for devices with both PDMA and GDMA controllers (#1125)
- The `ADC` driver's constructor is now `new` instead of `adc`, to be more consistent with other APIs (#1133)

## [0.15.0] - 2024-01-19

### Added

- ESP32-C6: Properly initialize PMU (#974)
- Implement overriding base mac address (#1044)
- Add `rt-riscv` and `rt-xtensa` features to enable/disable runtime support (#1057)
- ESP32-C6: Implement deep sleep (#918)
- Add `embedded-io` feature to each chip-specific HAL (#1072)
- Add `embassy-time-driver` to `esp-hal-common` due to updating `embassy-time` to `v0.3.0` (#1075)
- ESP32-S3: Added support for 80Mhz PSRAM (#1069)
- ESP32-C3/S3: Add workaround for USB pin exchange on usb-serial-jtag (#1104).
- ESP32C6: Added LP_UART initialization (#1113)
### Changed

- Set up interrupts for the DMA and async enabled peripherals only when `async` feature is provided (#1042)
- Update to `1.0.0` releases of the `embedded-hal-*` packages (#1068)
- Update `embassy-time` to `0.3.0` and embassy-executor to `0.5.0` release due to the release of the `embedded-hal-*` packages (#1075)
- No longer depend on `embassy-time` (#1092)
- Update to latest `smart-leds-trait` and `smart-leds` packages (#1094)

### Fixed

- ESP32: correct gpio 32/33 in errata36() (#1053)
- ESP32: make gpio 4 usable as analog pin (#1078)
- Fix double &mut for the `SetDutyCycle` impl on `PwmPin` (#1033)
- ESP32/ESP32-S3: Fix stack-top calculation for app-core (#1081)
- ESP32/ESP32-S2/ESP32-S3: Fix embassy-time-timg0 driver (#1091)
- ESP32: ADC readings are no longer inverted (#1093)

### Removed

### Breaking

- Unify the low-power peripheral names (`RTC_CNTL` and `LP_CLKRST` to `LPWR`) (#1064)

## [0.14.1] - 2023-12-13

### Fixed

- Fix SHA for all targets (#1021)

## [0.14.0] - 2023-12-12

### Added

- ESP32-C6: LP core clock is configurable (#907)
- Derive `Clone` and `Copy` for `EspTwaiFrame` (#914)
- A way to configure inverted pins (#912)
- Added API to check a GPIO-pin's interrupt status bit (#929)
- A `embedded_io_async::Read` implementation for `UsbSerialJtag` (#889)
- `RtcClock::get_xtal_freq`, `RtcClock::get_slow_freq` (#957)
- Added Rx Timeout functionality to async Uart (#911)
- RISC-V: Thread-mode and interrupt-mode executors, `#[main]` macro (#947)
- A macro to make it easier to create DMA buffers and descriptors (#935)
- I2C timeout is configurable (#1011)
- ESP32-C6/ESP32-H2: `flip-link` feature gives zero-cost stack overflow protection (#1008)

### Changed

- Improve DMA documentation & clean up module (#915)
- Only allow a single version of `esp-hal-common` to be present in an application (#934)
- ESP32-C3/C6 and ESP32-H2 can now use the `zero-rtc-bss` feature to enable `esp-hal-common/rv-zero-rtc-bss` (#867)
- Reuse `ieee802154_clock_enable/disable()` functions for BLE and rename `ble_ieee802154_clock_enable()` (#953)
- The `embedded-io` trait implementations are now gated behind the `embedded-io` feature (#964)
- Simplifed RMT channels and channel creators (#958)
- Reworked construction of I2S driver instances (#983)
- ESP32-S2/S3: Don't require GPIO 18 to create a USB peripheral driver instance (#990)
- Updated to latest release candidate (`1.0.0-rc.2`) for `embedded-hal{-async,-nb}` (#994)
- Explicit panic when hitting the `DefaultHandler` (#1005)
- Relevant interrupts are now auto enabled in `embassy::init` (#1014).

### Fixed

- ESP32-C2/C3 examples: fix build error (#899)
- ESP32-S3: Fix GPIO interrupt handler crashing when using GPIO48. (#898)
- Fixed short wait times in embassy causing hangs (#906)
- Make sure to clear LP/RTC RAM before loading code (#916)
- Async RMT channels can be used concurrently (#925)
- Xtensa: Allow using `embassy-executor`'s thread-mode executor if neither `embassy-executor-thread`, nor `embassy-executor-interrupt` is enabled. (#937)
- Uart Async: Improve interrupt handling and irq <--> future communication (#977)
- RISC-V: Fix stack allocation (#988)
- ESP32-C6: Fix used RAM (#997)
- ESP32-H2: Fix used RAM (#1003)
- Fix SPI slave DMA dma\_read and dma\_write (#1013)
- ESP32-C6/H2: Fix disabling of interrupts (#1040)

### Removed

- Direct boot support has been removed (#903).
- Removed the `mcu-boot` feature from `esp32c3-hal` (#938)
- Removed SpiBusController and SpiBusDevice in favour of embedded-hal-bus and embassy-embedded-hal implementataions. (#978)

### Breaking

- `Spi::new`/`Spi::new_half_duplex` takes no gpio pin now, instead you need to call `with_pins` to setup those (#901).
- ESP32-C2, ESP32-C3, ESP32-S2: atomic emulation trap has been removed. (#904) (#985)
    - When upgrading you must either remove [these lines](https://github.com/esp-rs/riscv-atomic-emulation-trap#usage) from your `.cargo/config.toml`.
    - Usage of `core::sync::atomic::*` in dependent crates should be replaced with [portable-atomic](https://github.com/taiki-e/portable-atomic).
- RSA driver now takes `u32` words instead of `u8` bytes. The expected slice length is now 4 times shorter. (#981)

## [0.13.1] - 2023-11-02

### Fixed

- ESP32-C3: Make sure BLE and WiFi are not powered down when esp-wifi needs them (#891)
- ESP32-C6/H2: Fix setting UART baud rate (#893)

## [0.13.0] - 2023-10-31

### Added

- Implement SetFrequencyCycle and PwmPin from embedded_hal for PwmPin of MCPWM. (#880)
- Added `embassy-time-systick` to ESP32-S2 (#827)
- Implement enabling/disabling BLE clock on ESP32-C6 (#784)
- Async support for RMT (#787)
- Implement `defmt::Format` for more types (#786)
- Add new_no_miso to Spi FullDuplexMode (#794)
- Add UART support for splitting into TX and RX (#754)
- Async support for I2S (#801)
- Async support for PARL_IO (#807)
- ETM driver, GPIO ETM (#819)
- (G)DMA AES support (#821)
- SYSTIMER ETM functionality (#828)
- Adding async support for RSA peripheral(doesn't work properly for `esp32` chip - issue will be created)(#790)
- Added sleep support for ESP32-C3 with timer and GPIO wakeups (#795)
- Support for ULP-RISCV including Delay and GPIO (#840, #845)
- Add bare-bones SPI slave support, DMA only (#580, #843)
- Embassy `#[main]` convenience macro (#841)
- Add a `defmt` feature to the `esp-hal-smartled` package (#846)
- Support 16MB octal PS-RAM for ESP32-S3 (#858)
- RISCV TRACE Encoder driver for ESP32-C6 / ESP32-H2 (#864)
- `embedded_hal` 1 `InputPin` and `embedded_hal_async` `Wait` impls for open drain outputs (#905)

### Changed

- Bumped MSRV to 1.67 (#798)
- Optimised multi-core critical section implementation (#797)
- Changed linear- and curve-calibrated ADC to provide readings in mV (#836)

### Fixed

- S3: Allow powering down RC_FAST_CLK (#796)
- UART/ESP32: fix calculating FIFO counter with `get_rx_fifo_count()` (#804)
- Xtensa targets: Use ESP32Reset - not Reset (#823)
- Examples should now work with the `defmt` feature (#810)
- Fixed a race condition causing SpiDma to stop working unexpectedly (#869)
- Fixed async uart serial, and updated the embassy_serial examples (#871).
- Fix ESP32-S3 direct-boot (#873)
- Fix ESP32-C6 ADC (#876)
- Fix ADC Calibration not being used on ESP32-S2 and ESP32-S3 (#1000)

### Removed

- `Pin::is_pcore_interrupt_set` (#793)
- `Pin::is_pcore_non_maskable_interrupt_set` (#793)
- `Pin::is_acore_interrupt_set` (#793)
- `Pin::is_acore_non_maskable_interrupt_set` (#793)
- `Pin::enable_hold` (#793)
- Removed the generic return type for ADC reads (#792)

### Breaking

- `Uart::new` now takes the `&Clocks` struct to ensure baudrate is correct for CPU/APB speed. (#808)
- `Uart::new_with_config` takes an `Config` instead of `Option<Config>`. (#808)
- `Alarm::set_period` takes a period (duration) instead of a frequency (#812)
- `Alarm::interrupt_clear` is now `Alarm::clear_interrupt` to be consistent (#812)
- The `PeripheralClockControl` struct is no longer public, drivers no longer take this as a parameter (#817)
- Unify the system peripheral, `SYSTEM`, `DPORT` and `PCR` are now all exposed as `SYSTEM` (#832).
- Unified the ESP32's and ESP32-C2's xtal frequency features (#831)
- Replace any underscores in feature names with dashes (#833)
- The `spi` and `spi_slave` modules have been refactored into the `spi`, `spi::master`, and `spi::slave` modules (#843)
- The `WithDmaSpi2`/`WithDmaSpi3` structs are no longer generic around the inner peripheral type (#853)
- The `SarAdcExt`/`SensExt` traits are now collectively named `AnalogExt` instead (#857)
- Replace the `radio` module with peripheral singleton structs (#852)
- The SPI traits are no longer re-exported in the main prelude, but from preludes in `spi::master`/`spi::slave` instead (#860)
- The `embedded-hal-1` and `embedded-hal-async` traits are no longer re-exported in the prelude (#860)

## [0.12.0] - 2023-09-05

### Added

- Implement RTCIO pullup, pulldown and hold control for Xtensa MCUs (#684)
- S3: Implement RTCIO wakeup source (#690)
- Add PARL_IO driver for ESP32-C6 / ESP32-H2 (#733, #760)
- Implement `ufmt_write::uWrite` trait for USB Serial JTAG (#751)
- Add HMAC peripheral support (#755)
- Add multicore-aware embassy executor for Xtensa MCUs (#723, #756).
- Add interrupt-executor for Xtensa MCUs (#723, #756).
- Add missing `Into<Gpio<Analog, GPIONUN>>` conversion (#764)
- Updated `clock` module documentation (#774)
- Add `log` feature to enable log output (#773)
- Add `defmt` feature to enable log output (#773)
- A new macro to load LP core code on ESP32-C6 (#779)
- Add `ECC`` peripheral driver (#785)
- Initial LLD support for Xtensa chips (#861).

### Changed

- Update the `embedded-hal-*` packages to `1.0.0-rc.1` and implement traits from `embedded-io` and `embedded-io-async` (#747)
- Moved AlignmentHelper to its own module (#753)
- Disable all watchdog timers by default at startup (#763)
- `log` crate is now opt-in (#773)

### Fixed

- Fix `psram` availability lookup in `esp-hal-common` build script (#718)
- Fix wrong `dram_seg` length in `esp32s2-hal` linker script (#732)
- Fix setting alarm when a timer group is used as the alarm source. (#730)
- Fix `Instant::now()` not counting in some cases when using TIMG0 as the timebase (#737)
- Fix number of ADC attenuations for ESP32-C6 (#771)
- Fix SHA registers access (#805)

### Breaking

- `CpuControl::start_app_core()` now takes an `FnOnce` closure (#739)

## [0.11.0] - 2023-08-10

### Added

- Add initial LP-IO support for ESP32-C6 (#639)
- Implement sleep with some wakeup methods for `esp32` (#574)
- Add a new RMT driver (#653, #667, #695)
- Implemented calibrated ADC API for ESP32-S3 (#641)
- Add MCPWM DeadTime configuration (#406)
- Implement sleep with some wakeup methods for `esp32-s3` (#660, #689, #696)
- Add feature enabling directly hooking the interrupt vector table (#621)
- Add `ClockControl::max` helper for all chips (#701)
- Added module-level documentation for all peripherals (#680)
- Implement sleep with some wakeup methods for `esp32-s3` (#660)
- Add `FlashSafeDma` wrapper for eh traits which ensure correct DMA transfer from source data in flash (ROM) (#678)

### Changed

- Update `embedded-hal-*` alpha packages to their latest versions (#640)
- Implement the `Clone` and `Copy` traits for the `Rng` driver (#650)
- Use all remaining memory as core-0's stack (#716)

### Fixed

- Fixed Async Uart `read` when `set_at_cmd` is not used (#652)
- USB device support is working again (#656)
- Add missing interrupt status read for esp32s3, which fixes USB-SERIAL-JTAG interrupts (#664)
- GPIO interrupt status bits are now properly cleared (#670)
- Increase frequency resolution in `set_periodic` (#686)
- Fixed ESP32-S2, ESP32-S3, ESP32-C2, ESP32-C3 radio clock gating (#679, #681)
- Partially fix ESP32 radio clocks (#709)
- Fixed "ESP32/ESP32-S2 RMT transmission with with data.len() > RMT_CHANNEL_RAM_SIZE results in TransmissionError" #707 (#710)

### Removed

- Remove the `allow-opt-level-z` feature from `esp32c3-hal` (#654)
- Remove the old `pulse_control` driver (#694)

### Breaking

- `DmaTransfer::wait` and `I2sReadDmaTransfer::wait_receive` now return `Result` (#665)
- `gpio::Pin` is now object-safe (#687)

## [0.10.0] - 2023-06-04

### Added

- Add `WithDmaSpi3` to prelude for ESP32S3 (#623)
- Add bare-bones PSRAM support for ESP32 (#506)
- Add initial support for the ESP32-H2 (#513, #526, #527, #528, #530, #538, #544, #548, #551, #556, #560, #566, #549, #564, #569, #576, #577, #589, #591, #597)
- Add bare-bones PSRAM support for ESP32-S3 (#517)
- Add async support to the I2C driver (#519)
- Implement Copy and Eq for EspTwaiError (#540)
- Add LEDC hardware fade support (#475)
- Added support for multicore async GPIO (#542)
- Add a fn to poll DMA transfers (#559)
- Add unified field-based efuse access (#567)
- Move `esp-riscv-rt` into esp-hal (#578)
- Add CRC functions from ESP ROM (#587)
- Add a `debug` feature to enable the PACs' `impl-register-debug` feature (#596)
- Add initial support for `I2S` in ESP32-H2 (#597)
- Add octal PSRAM support for ESP32-S3 (#610)
- Add MD5 functions from ESP ROM (#618)
- Add embassy async `read` support for `uart` (#620)
- Add bare-bones support to run code on ULP-RISCV / LP core (#631)
- Add ADC calibration implementation for a riscv chips (#555)
- Add `async` implementation for `USB Serial/JTAG`(#632)

### Changed

- Simplify the `Delay` driver, derive `Clone` and `Copy` (#568)
- DMA types can no longer be constructed by the user (#625)
- Move core interrupt handling from Flash to RAM for RISC-V chips (ESP32-H2, ESP32-C2, ESP32-C3, ESP32-C6) (#541)
- Change LED pin to GPIO2 in ESP32 blinky example (#581)
- Update ESP32-H2 and ESP32-C6 clocks and remove `i2c_clock` for all chips but ESP32 (#592)
- Use both timers in `TIMG0` for embassy time driver when able (#609)
- Re-work `RadioExt` implementations, add support for ESP32-H2 (#627)
- Improve examples documentation (#533)
- esp32h2-hal: added README (#585)
- Update `esp-hal-procmacros` package dependencies and features (#628)

### Fixed

- Corrected the expected DMA descriptor counts (#622, #625)
- DMA is supported for SPI3 on ESP32-S3 (#507)
- `change_bus_frequency` is now available on `SpiDma` (#529)
- Fixed a bug where a GPIO interrupt could erroneously fire again causing the next `await` on that pin to instantly return `Poll::Ok` (#537)
- Set `vecbase` on core 1 (ESP32, ESP32-S3) (#536)
- ESP32-S3: Move PSRAM related function to RAM (#546)
- ADC driver will now apply attenuation values to the correct ADC's channels. (#554)
- Sometimes half-duplex non-DMA SPI reads were reading garbage in non-release mode (#552)
- ESP32-C3: Fix GPIO5 ADC channel id (#562)
- ESP32-H2: Fix direct-boot feature (#570)
- Fix Async GPIO not disabling interupts on chips with multiple banks (#572)
- ESP32-C6: Support FOSC CLK calibration for ECO1+ chip revisions (#593)
- Fixed CI by pinning the log crate to 0.4.18 (#600)
- ESP32-S3: Fix calculation of PSRAM start address (#601)
- Fixed wrong variable access (FOSC CLK calibration for ESP32-C6 #593)
- Fixed [trap location in ram](https://github.com/esp-rs/esp-hal/pull/605#issuecomment-1604039683) (#605)
- Fix rom::crc docs (#611)
- Fixed a possible overlap of `.data` and `.rwtext` (#616)
- Avoid SDA/SCL being low while configuring pins for I2C (#619)

### Breaking

- Simplified user-facing SpiDma and I2s types (#626)
- Significantly simplified user-facing GPIO pin types. (#553)
- No longer re-export the `soc` module and the contents of the `interrupt` module at the package level (#607)

## [0.9.0] - 2023-05-02

### Added

- Add bare-bones PSRAM support for ESP32-S2 (#493)
- Add `DEBUG_ASSIST` functionality (#484)
- Add RSA peripheral support (#467)
- Add PeripheralClockControl argument to `timg`, `wdt`, `sha`, `usb-serial-jtag` and `uart` constructors (#463)
- Added API to raise and reset software interrupts (#426)
- Implement `embedded_hal_nb::serial::*` traits for `UsbSerialJtag` (#498)

### Fixed

- Fix `get_wakeup_cause` comparison error (#472)
- Use 192 as mclk_multiple for 24-bit I2S (#471)
- Fix `CpuControl::start_app_core` signature (#466)
- Move `rwtext` after other RAM data sections (#464)
- ESP32-C3: Disable `usb_pad_enable` when setting GPIO18/19 to input/output (#461)
- Fix 802.15.4 clock enabling (ESP32-C6) (#458)
- ESP32-S3: Disable usb_pad_enable when setting GPIO19/20 to input/output (#645)

### Changed

- Update `embedded-hal-async` and `embassy-*` dependencies (#488)
- Update to `embedded-hal@1.0.0-alpha.10` and `embedded-hal-nb@1.0.0-alpha.2` (#487)
- Let users configure the LEDC output pin as open-drain (#474)
- Use bitflags to decode wakeup cause (#473)
- Minor linker script additions (#470)
- Minor documentation improvements (#460)

### Removed

- Remove unnecessary generic from `UsbSerialJtag` driver (#492)
- Remove `#[doc(inline)]` from esp-hal-common re-exports (#490)

## [0.8.0] - 2023-03-27

## [0.7.1] - 2023-02-22

## [0.7.0] - 2023-02-21

## [0.5.0] - 2023-01-26

## [0.4.0] - 2022-12-12

## [0.3.0] - 2022-11-17

## [0.2.0] - 2022-09-13

## [0.1.0] - 2022-08-05

[Unreleased]: https://github.com/esp-rs/esp-hal/compare/v0.15.0...HEAD
[0.15.0]: https://github.com/esp-rs/esp-hal/compare/v0.14.1...v0.15.0
[0.14.1]: https://github.com/esp-rs/esp-hal/compare/v0.14.0...v0.14.1
[0.14.0]: https://github.com/esp-rs/esp-hal/compare/v0.13.1...v0.14.0
[0.13.1]: https://github.com/esp-rs/esp-hal/compare/v0.13.0...v0.13.1
[0.13.0]: https://github.com/esp-rs/esp-hal/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/esp-rs/esp-hal/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/esp-rs/esp-hal/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/esp-rs/esp-hal/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/esp-rs/esp-hal/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/esp-rs/esp-hal/compare/v0.7.1...v0.8.0
[0.7.1]: https://github.com/esp-rs/esp-hal/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/esp-rs/esp-hal/compare/v0.5.0...v0.7.0
[0.5.0]: https://github.com/esp-rs/esp-hal/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/esp-rs/esp-hal/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/esp-rs/esp-hal/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/esp-rs/esp-hal/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/esp-rs/esp-hal/releases/tag/v0.1.0
