use stm32f4xx_hal::{
    gpio::{gpioa, gpiob, gpioc, gpiod, Output, PushPull},
    prelude::*,
};

pub struct GpioPins {
    // Bank 0: PA0-PA7
    pub bank0: [gpioa::PAx<Output<PushPull>>; 8],
    // Bank 1: PB0-PB7  
    pub bank1: [gpiob::PBx<Output<PushPull>>; 8],
    // Bank 2: PC0-PC7
    pub bank2: [gpioc::PCx<Output<PushPull>>; 8],
    // Bank 3: PD0-PD7
    pub bank3: [gpiod::PDx<Output<PushPull>>; 8],
}

impl GpioPins {
    pub fn new(
        gpioa: gpioa::Parts,
        gpiob: gpiob::Parts,
        gpioc: gpioc::Parts,
        gpiod: gpiod::Parts,
    ) -> Self {
        // Initialize all 32 pins as outputs
        let bank0 = [
            gpioa.pa0.into_push_pull_output(),
            gpioa.pa1.into_push_pull_output(),
            gpioa.pa2.into_push_pull_output(),
            gpioa.pa3.into_push_pull_output(),
            gpioa.pa4.into_push_pull_output(),
            gpioa.pa5.into_push_pull_output(),
            gpioa.pa6.into_push_pull_output(),
            gpioa.pa7.into_push_pull_output(),
        ];
        
        let bank1 = [
            gpiob.pb0.into_push_pull_output(),
            gpiob.pb1.into_push_pull_output(),
            gpiob.pb2.into_push_pull_output(),
            gpiob.pb3.into_push_pull_output(),
            gpiob.pb4.into_push_pull_output(),
            gpiob.pb5.into_push_pull_output(),
            gpiob.pb6.into_push_pull_output(),
            gpiob.pb7.into_push_pull_output(),
        ];
        
        let bank2 = [
            gpioc.pc0.into_push_pull_output(),
            gpioc.pc1.into_push_pull_output(),
            gpioc.pc2.into_push_pull_output(),
            gpioc.pc3.into_push_pull_output(),
            gpioc.pc4.into_push_pull_output(),
            gpioc.pc5.into_push_pull_output(),
            gpioc.pc6.into_push_pull_output(),
            gpioc.pc7.into_push_pull_output(),
        ];
        
        let bank3 = [
            gpiod.pd0.into_push_pull_output(),
            gpiod.pd1.into_push_pull_output(),
            gpiod.pd2.into_push_pull_output(),
            gpiod.pd3.into_push_pull_output(),
            gpiod.pd4.into_push_pull_output(),
            gpiod.pd5.into_push_pull_output(),
            gpiod.pd6.into_push_pull_output(),
            gpiod.pd7.into_push_pull_output(),
        ];
        
        Self {
            bank0,
            bank1,
            bank2,
            bank3,
        }
    }
    
    pub fn set_bank0_relay(&mut self, relay: u8, state: bool) {
        if relay < 8 {
            if state {
                self.bank0[relay as usize].set_high();
            } else {
                self.bank0[relay as usize].set_low();
            }
        }
    }
    
    pub fn set_bank1_relay(&mut self, relay: u8, state: bool) {
        if relay < 8 {
            if state {
                self.bank1[relay as usize].set_high();
            } else {
                self.bank1[relay as usize].set_low();
            }
        }
    }
    
    pub fn set_bank2_relay(&mut self, relay: u8, state: bool) {
        if relay < 8 {
            if state {
                self.bank2[relay as usize].set_high();
            } else {
                self.bank2[relay as usize].set_low();
            }
        }
    }
    
    pub fn set_bank3_relay(&mut self, relay: u8, state: bool) {
        if relay < 8 {
            if state {
                self.bank3[relay as usize].set_high();
            } else {
                self.bank3[relay as usize].set_low();
            }
        }
    }
}