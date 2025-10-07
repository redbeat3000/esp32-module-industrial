use stm32f4xx_hal::{
    gpio::{gpioa, gpiob, gpioc, gpiod, Output, PushPull},
};

pub struct GpioPins {
    // Bank 0: PA0-PA7 - use individual pin types instead of array
    pub bank0_pa0: gpioa::PA0<Output<PushPull>>,
    pub bank0_pa1: gpioa::PA1<Output<PushPull>>,
    pub bank0_pa2: gpioa::PA2<Output<PushPull>>,
    pub bank0_pa3: gpioa::PA3<Output<PushPull>>,
    pub bank0_pa4: gpioa::PA4<Output<PushPull>>,
    pub bank0_pa5: gpioa::PA5<Output<PushPull>>,
    pub bank0_pa6: gpioa::PA6<Output<PushPull>>,
    pub bank0_pa7: gpioa::PA7<Output<PushPull>>,
    
    // Bank 1: PB0-PB7
    pub bank1_pb0: gpiob::PB0<Output<PushPull>>,
    pub bank1_pb1: gpiob::PB1<Output<PushPull>>,
    pub bank1_pb2: gpiob::PB2<Output<PushPull>>,
    pub bank1_pb3: gpiob::PB3<Output<PushPull>>,
    pub bank1_pb4: gpiob::PB4<Output<PushPull>>,
    pub bank1_pb5: gpiob::PB5<Output<PushPull>>,
    pub bank1_pb6: gpiob::PB6<Output<PushPull>>,
    pub bank1_pb7: gpiob::PB7<Output<PushPull>>,
    
    // Bank 2: PC0-PC7
    pub bank2_pc0: gpioc::PC0<Output<PushPull>>,
    pub bank2_pc1: gpioc::PC1<Output<PushPull>>,
    pub bank2_pc2: gpioc::PC2<Output<PushPull>>,
    pub bank2_pc3: gpioc::PC3<Output<PushPull>>,
    pub bank2_pc4: gpioc::PC4<Output<PushPull>>,
    pub bank2_pc5: gpioc::PC5<Output<PushPull>>,
    pub bank2_pc6: gpioc::PC6<Output<PushPull>>,
    pub bank2_pc7: gpioc::PC7<Output<PushPull>>,
    
    // Bank 3: PD0-PD7
    pub bank3_pd0: gpiod::PD0<Output<PushPull>>,
    pub bank3_pd1: gpiod::PD1<Output<PushPull>>,
    pub bank3_pd2: gpiod::PD2<Output<PushPull>>,
    pub bank3_pd3: gpiod::PD3<Output<PushPull>>,
    pub bank3_pd4: gpiod::PD4<Output<PushPull>>,
    pub bank3_pd5: gpiod::PD5<Output<PushPull>>,
    pub bank3_pd6: gpiod::PD6<Output<PushPull>>,
    pub bank3_pd7: gpiod::PD7<Output<PushPull>>,
}

impl GpioPins {
    pub fn new(
        gpioa: gpioa::Parts,
        gpiob: gpiob::Parts,
        gpioc: gpioc::Parts,
        gpiod: gpiod::Parts,
    ) -> Self {
        Self {
            // Bank 0
            bank0_pa0: gpioa.pa0.into_push_pull_output(),
            bank0_pa1: gpioa.pa1.into_push_pull_output(),
            bank0_pa2: gpioa.pa2.into_push_pull_output(),
            bank0_pa3: gpioa.pa3.into_push_pull_output(),
            bank0_pa4: gpioa.pa4.into_push_pull_output(),
            bank0_pa5: gpioa.pa5.into_push_pull_output(),
            bank0_pa6: gpioa.pa6.into_push_pull_output(),
            bank0_pa7: gpioa.pa7.into_push_pull_output(),
            
            // Bank 1
            bank1_pb0: gpiob.pb0.into_push_pull_output(),
            bank1_pb1: gpiob.pb1.into_push_pull_output(),
            bank1_pb2: gpiob.pb2.into_push_pull_output(),
            bank1_pb3: gpiob.pb3.into_push_pull_output(),
            bank1_pb4: gpiob.pb4.into_push_pull_output(),
            bank1_pb5: gpiob.pb5.into_push_pull_output(),
            bank1_pb6: gpiob.pb6.into_push_pull_output(),
            bank1_pb7: gpiob.pb7.into_push_pull_output(),
            
            // Bank 2
            bank2_pc0: gpioc.pc0.into_push_pull_output(),
            bank2_pc1: gpioc.pc1.into_push_pull_output(),
            bank2_pc2: gpioc.pc2.into_push_pull_output(),
            bank2_pc3: gpioc.pc3.into_push_pull_output(),
            bank2_pc4: gpioc.pc4.into_push_pull_output(),
            bank2_pc5: gpioc.pc5.into_push_pull_output(),
            bank2_pc6: gpioc.pc6.into_push_pull_output(),
            bank2_pc7: gpioc.pc7.into_push_pull_output(),
            
            // Bank 3
            bank3_pd0: gpiod.pd0.into_push_pull_output(),
            bank3_pd1: gpiod.pd1.into_push_pull_output(),
            bank3_pd2: gpiod.pd2.into_push_pull_output(),
            bank3_pd3: gpiod.pd3.into_push_pull_output(),
            bank3_pd4: gpiod.pd4.into_push_pull_output(),
            bank3_pd5: gpiod.pd5.into_push_pull_output(),
            bank3_pd6: gpiod.pd6.into_push_pull_output(),
            bank3_pd7: gpiod.pd7.into_push_pull_output(),
        }
    }
    
    pub fn set_bank0_relay(&mut self, relay: u8, state: bool) {
        match relay {
            0 => self.set_pin_state(&mut self.bank0_pa0, state),
            1 => self.set_pin_state(&mut self.bank0_pa1, state),
            2 => self.set_pin_state(&mut self.bank0_pa2, state),
            3 => self.set_pin_state(&mut self.bank0_pa3, state),
            4 => self.set_pin_state(&mut self.bank0_pa4, state),
            5 => self.set_pin_state(&mut self.bank0_pa5, state),
            6 => self.set_pin_state(&mut self.bank0_pa6, state),
            7 => self.set_pin_state(&mut self.bank0_pa7, state),
            _ => (),
        }
    }
    
    pub fn set_bank1_relay(&mut self, relay: u8, state: bool) {
        match relay {
            0 => self.set_pin_state(&mut self.bank1_pb0, state),
            1 => self.set_pin_state(&mut self.bank1_pb1, state),
            2 => self.set_pin_state(&mut self.bank1_pb2, state),
            3 => self.set_pin_state(&mut self.bank1_pb3, state),
            4 => self.set_pin_state(&mut self.bank1_pb4, state),
            5 => self.set_pin_state(&mut self.bank1_pb5, state),
            6 => self.set_pin_state(&mut self.bank1_pb6, state),
            7 => self.set_pin_state(&mut self.bank1_pb7, state),
            _ => (),
        }
    }
    
    pub fn set_bank2_relay(&mut self, relay: u8, state: bool) {
        match relay {
            0 => self.set_pin_state(&mut self.bank2_pc0, state),
            1 => self.set_pin_state(&mut self.bank2_pc1, state),
            2 => self.set_pin_state(&mut self.bank2_pc2, state),
            3 => self.set_pin_state(&mut self.bank2_pc3, state),
            4 => self.set_pin_state(&mut self.bank2_pc4, state),
            5 => self.set_pin_state(&mut self.bank2_pc5, state),
            6 => self.set_pin_state(&mut self.bank2_pc6, state),
            7 => self.set_pin_state(&mut self.bank2_pc7, state),
            _ => (),
        }
    }
    
    pub fn set_bank3_relay(&mut self, relay: u8, state: bool) {
        match relay {
            0 => self.set_pin_state(&mut self.bank3_pd0, state),
            1 => self.set_pin_state(&mut self.bank3_pd1, state),
            2 => self.set_pin_state(&mut self.bank3_pd2, state),
            3 => self.set_pin_state(&mut self.bank3_pd3, state),
            4 => self.set_pin_state(&mut self.bank3_pd4, state),
            5 => self.set_pin_state(&mut self.bank3_pd5, state),
            6 => self.set_pin_state(&mut self.bank3_pd6, state),
            7 => self.set_pin_state(&mut self.bank3_pd7, state),
            _ => (),
        }
    }
    
    fn set_pin_state<P: embedded_hal::digital::OutputPin>(&mut self, pin: &mut P, state: bool) {
        if state {
            let _ = pin.set_high();
        } else {
            let _ = pin.set_low();
        }
    }
}