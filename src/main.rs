#![no_main]
#![no_std]

use panic_halt as _;

use nrf52840_hal as hal;

use hal::{pac::{CorePeripherals, Peripherals},
        prelude::*,
        gpio::Level,
        delay::Delay,
        saadc::{Saadc,SaadcConfig},
        uarte::{Uarte,Parity,Baudrate},
        };

use cortex_m_rt::entry;

use core::fmt::Write;

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut led = port0.p0_06.into_push_pull_output(Level::Low);
    
    // set up delay provider
    let mut delay = Delay::new(core.SYST);
    
    // set up ADC and analog pin to read
    let mut adc = Saadc::new(p.SAADC, SaadcConfig::default());
    let mut a0 = port0.p0_04.into_floating_input();

    // define pins for UART
    let rx = port0.p0_25.into_floating_input().degrade();
    let tx = port0.p0_24.into_push_pull_output(Level::Low).degrade();
    let ct = port0.p0_30.into_floating_input().degrade(); // CTS: not used but necessary for configuration, pin may vary
    let rt = port0.p0_28.into_push_pull_output(Level::Low).degrade(); // RTS: not used but necessary for configuration, pin may vary
    
    let pins = hal::uarte::Pins{
            rxd: rx,
            txd: tx,
            cts: Some(ct),
            rts: Some(rt),
            };

    // set up UART
    let mut serial = Uarte::new(p.UARTE0, pins, Parity::EXCLUDED, Baudrate::BAUD9600);

    let mut adc_val: i16 = 0;

    loop {       
        // read a value from ADC and output to serial, toggle lead each time
        adc_val = adc.read(&mut a0).unwrap(); 
        serial.write_fmt(format_args!("Value: {}\n\r", adc_val)).unwrap();

        if led.is_set_high().unwrap() {
            led.set_low().unwrap();
            }
        else {
            led.set_high().unwrap();
            }

        delay.delay_ms(250_u32);              
    }
    
}