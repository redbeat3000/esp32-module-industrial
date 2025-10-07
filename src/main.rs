#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use stm32f4xx_hal::{
    gpio::{Output, PushPull},
    pac,
    prelude::*,
};

mod relays;
mod pins;

use relays::RelayController;
use pins::GpioPins;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("STM32F407 Relay Controller Starting...");
    
    // Initialize peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripheral::take().unwrap();
    
    // Setup clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
    
    // Initialize GPIO pins
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    
    let gpio_pins = GpioPins::new(gpioa, gpiob, gpioc, gpiod);
    let mut relay_controller = RelayController::new(gpio_pins);
    
    info!("Relay Controller Initialized");
    info!("Starting main control loop...");
    
    // Main control loop
    let mut counter = 0;
    loop {
        let bank = counter % 4;
        let state = (counter / 4) % 2 == 0;
        
        info!("Setting bank {} to {}", bank, state);
        
        let mask = if state { 0xFF } else { 0x00 };
        if let Err(e) = relay_controller.set_bank(bank, mask) {
            error!("Error setting bank {}: {}", bank, e);
        }
        
        counter += 1;
        Timer::after(Duration::from_secs(1)).await;
    }
}