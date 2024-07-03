#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led1 = Output::new(p.PC0, Level::High, Speed::Low);
    let mut led2 = Output::new(p.PC7, Level::High, Speed::Low);
    


    loop {
        info!("high");
        led1.set_high();
        led2.set_low();
        
        Timer::after_millis(500).await;

        info!("low");
        led1.set_low();
        led2.set_high();
        
        Timer::after_millis(500).await;
    }
}