
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use log::info;

use std::fmt::format;
use std::fmt::Write;
use std::string;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart;
use esp_idf_hal::delay::BLOCK;


fn main()->anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    // let peripherals = Peripherals::take().unwrap();
    // let mut led_r= PinDriver::output(peripherals.pins.gpio9)?;
    // let mut led_g= PinDriver::output(peripherals.pins.gpio10)?;

    // loop{
    //     led_r.set_high()?;
    //     led_g.set_high()?;
    //     info!("led rg high");
    //     FreeRtos::delay_ms(600);
    //     led_r.set_low()?;
    //     led_g.set_low()?;
    //     info!("led rg low");
    //     FreeRtos::delay_ms(600);
    // }

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    
    let config = uart::config::Config::default().baudrate(Hertz(115_200));
    
    let mut uart1: uart::UartDriver = uart::UartDriver::new(
        peripherals.uart1,
        pins.gpio1,
        pins.gpio2,
        Option::<AnyIOPin>::None,
        Option::<AnyIOPin>::None,
        &config
    ).unwrap();


    let mut i:u32 = 0;
    let mut data_arr:u8 =  {5;20};
    let mut str:&str =  "abcdefg\r\n";
    uart1.write_str(&str);

    loop {
        //writeln!(uart, "{:}", format!("count {:}", i)).unwrap();
        // format(args);
        
        
        let mut buf = [0_u8; 1];
        let mut buf_len = buf.len();
        let mut index = 0;
        uart1.read(&mut buf,BLOCK)?;
        uart1.write_char(buf[0] as char);
        // while buf_len > 0 {
        //     let mut r_data: u8 = 0;
        //     info!("uart1 read  {}",buf[index] as char);
        //     index+=1;
        //     buf_len-=1;
        // }

        // info!("Written  {}, read buf {}{}{}{}{}",data_arr, buf[0],buf[1],buf[2],buf[3],buf[4]);
    }
    // loop {
    //     uart.write(&[0xaa])?;

    //     let mut buf = [0_u8; 1];
    //     uart.read(&mut buf, BLOCK)?;

    //     println!("Written 0xaa, read 0x{:02x}", buf[0]);
    // }
}
