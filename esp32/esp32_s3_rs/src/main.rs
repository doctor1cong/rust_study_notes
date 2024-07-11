
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use log::info;

fn main()->anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led_r= PinDriver::output(peripherals.pins.gpio9)?;
    let mut led_g= PinDriver::output(peripherals.pins.gpio10)?;

    loop{
        led_r.set_high()?;
        led_g.set_high()?;
        info!("led rg high");
        FreeRtos::delay_ms(600);
        led_r.set_low()?;
        led_g.set_low()?;
        info!("led rg low");
        FreeRtos::delay_ms(600);
    }
}
