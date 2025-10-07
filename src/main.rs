#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;

use stm32f4xx_hal::{
    pac,
    prelude::*,
};

mod relays;
mod pins;

use relays::RelayController;
use pins::GpioPins;

#[entry]
fn main() -> ! {
    info!("STM32F407 Relay Controller Starting...");
    
    // Initialize peripherals
    let dp = pac::Peripherals::take().unwrap();
    
    // Setup clocks - NEW API
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(84.MHz()).freeze();
    
    // Get SYSCFG for GPIO split
    let mut syscfg = dp.SYSCFG.constrain();
    
    // Initialize GPIO pins with SYSCFG reference
    let gpioa = dp.GPIOA.split(&mut syscfg);
    let gpiob = dp.GPIOB.split(&mut syscfg);
    let gpioc = dp.GPIOC.split(&mut syscfg);
    let gpiod = dp.GPIOD.split(&mut syscfg);
    
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
        
        // Simple delay
        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }
    }
}