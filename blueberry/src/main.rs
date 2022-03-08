// See
// - https://github.com/esp-rs/esp32-hal/blob/master/examples/leds.rs
// - https://github.com/esp-rs/esp-idf-hal/blob/master/examples/rmt_neopixel.rs

use log::info;

use core::time::Duration;
use embedded_hal::delay::blocking::DelayUs;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::{FixedLengthSignal, PinState, Pulse, Transmit};

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("booted");

    let peripherals = Peripherals::take().unwrap();
    let led = peripherals
        .pins
        .gpio48
        .into_output()
        .expect("could not get led GPIO");
    let channel = peripherals.rmt.channel0;
    let config = TransmitConfig::new().clock_divider(1);
    let mut tx = Transmit::new(led, channel, &config).unwrap();

    let rgbs = [0xff0000, 0xffff00, 0x00ffff, 0x00ff00, 0xa000ff];
    loop {
        info!("outer");
        for rgb in rgbs {
            info!("inner");

            let ticks_hz = tx.counter_clock().unwrap();
            let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(350)).unwrap();
            let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(800)).unwrap();
            let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &ns(700)).unwrap();
            let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &ns(600)).unwrap();

            let mut signal = FixedLengthSignal::<24>::new();
            for i in 0..24 {
                let bit = 2_u32.pow(i) & rgb != 0;
                let (high_pulse, low_pulse) = if bit { (t1h, t1l) } else { (t0h, t0l) };
                signal.set(i as usize, &(high_pulse, low_pulse)).unwrap();
            }
            tx.start_blocking(&signal).unwrap();
            Ets.delay_ms(1000).unwrap();
        }
    }
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}
