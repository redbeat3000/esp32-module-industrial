#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{
    gpio::{Output, PushPull},
    pac,
    prelude::*,
};

mod relays;
mod pins;

use relays::RelayController;
use pins::GpioPins;

#[entry]
fn main() -> ! {
    // Initialize peripherals
    let dp = pac::Peripherals::take().unwrap();
    
    // Setup clocks
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.sysclk(84.MHz()).freeze();
    
    // Initialize GPIO pins
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
    let gpiod = dp.GPIOD.split();
    
    let gpio_pins = GpioPins::new(gpioa, gpiob, gpioc, gpiod);
    let mut relay_controller = RelayController::new(gpio_pins);
    
    // Main control loop
    let mut counter = 0;
    loop {
        let bank = counter % 4;
        let state = (counter / 4) % 2 == 0;
        
        let mask = if state { 0xFF } else { 0x00 };
        let _ = relay_controller.set_bank(bank, mask);
        
        counter += 1;
        
        // Simple delay
        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }
    }
}