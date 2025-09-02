#![no_std]
#![no_main]

mod driver;

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    I2C,
};

use fugit::RateExtU32;

use crate::driver::Tmp102;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up the I2C bus
    let sda_pin = pins.gpio4.into_mode::<bsp::hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio5.into_mode::<bsp::hal::gpio::FunctionI2C>();

    let i2c = I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    // Create the driver instance
    let mut sensor = Tmp102::new(i2c);

    loop {
        if let Ok(temp) = sensor.read_temperature() {
            info!("Temperature: {} C", temp);
        } else {
            info!("Failed to read temperature");
        }
        delay.delay_ms(1000);
    }
}
