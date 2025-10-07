use stm32f4xx_hal::{
    gpio::{gpioa, gpiob, gpioc, gpiod, Output, PushPull, DynamicPin},
};

pub struct GpioPins {
    // Bank 0: PA0-PA7 - use DynamicPin which handles all pin types
    pub bank0: [DynamicPin; 8],
    // Bank 1: PB0-PB7  
    pub bank1: [DynamicPin; 8],
    // Bank 2: PC0-PC7
    pub bank2: [DynamicPin; 8],
    // Bank 3: PD0-PD7
    pub bank3: [DynamicPin; 8],
}

impl GpioPins {
    pub fn new(
        gpioa: gpioa::Parts,
        gpiob: gpiob::Parts,
        gpioc: gpioc::Parts,
        gpiod: gpiod::Parts,
    ) -> Self {
        // Initialize all 32 pins as outputs and convert to DynamicPin
        let bank0 = [
            gpioa.pa0.into_push_pull_output().into(),
            gpioa.pa1.into_push_pull_output().into(),
            gpioa.pa2.into_push_pull_output().into(),
            gpioa.pa3.into_push_pull_output().into(),
            gpioa.pa4.into_push_pull_output().into(),
            gpioa.pa5.into_push_pull_output().into(),
            gpioa.pa6.into_push_pull_output().into(),
            gpioa.pa7.into_push_pull_output().into(),
        ];
        
        let bank1 = [
            gpiob.pb0.into_push_pull_output().into(),
            gpiob.pb1.into_push_pull_output().into(),
            gpiob.pb2.into_push_pull_output().into(),
            gpiob.pb3.into_push_pull_output().into(),
            gpiob.pb4.into_push_pull_output().into(),
            gpiob.pb5.into_push_pull_output().into(),
            gpiob.pb6.into_push_pull_output().into(),
            gpiob.pb7.into_push_pull_output().into(),
        ];
        
        let bank2 = [
            gpioc.pc0.into_push_pull_output().into(),
            gpioc.pc1.into_push_pull_output().into(),
            gpioc.pc2.into_push_pull_output().into(),
            gpioc.pc3.into_push_pull_output().into(),
            gpioc.pc4.into_push_pull_output().into(),
            gpioc.pc5.into_push_pull_output().into(),
            gpioc.pc6.into_push_pull_output().into(),
            gpioc.pc7.into_push_pull_output().into(),
        ];
        
        let bank3 = [
            gpiod.pd0.into_push_pull_output().into(),
            gpiod.pd1.into_push_pull_output().into(),
            gpiod.pd2.into_push_pull_output().into(),
            gpiod.pd3.into_push_pull_output().into(),
            gpiod.pd4.into_push_pull_output().into(),
            gpiod.pd5.into_push_pull_output().into(),
            gpiod.pd6.into_push_pull_output().into(),
            gpiod.pd7.into_push_pull_output().into(),
        ];
        
        Self {
            bank0,
            bank1,
            bank2,
            bank3,
        }
    }
    
    pub fn set_bank0_relay(&mut self, relay: u8, state: bool) {
        if (relay as usize) < self.bank0.len() {
            if state {
                let _ = self.bank0[relay as usize].set_high();
            } else {
                let _ = self.bank0[relay as usize].set_low();
            }
        }
    }
    
    pub fn set_bank1_relay(&mut self, relay: u8, state: bool) {
        if (relay as usize) < self.bank1.len() {
            if state {
                let _ = self.bank1[relay as usize].set_high();
            } else {
                let _ = self.bank1[relay as usize].set_low();
            }
        }
    }
    
    pub fn set_bank2_relay(&mut self, relay: u8, state: bool) {
        if (relay as usize) < self.bank2.len() {
            if state {
                let _ = self.bank2[relay as usize].set_high();
            } else {
                let _ = self.bank2[relay as usize].set_low();
            }
        }
    }
    
    pub fn set_bank3_relay(&mut self, relay: u8, state: bool) {
        if (relay as usize) < self.bank3.len() {
            if state {
                let _ = self.bank3[relay as usize].set_high();
            } else {
                let _ = self.bank3[relay as usize].set_low();
            }
        }
    }
}