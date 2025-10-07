use defmt::*;
use embedded_hal::digital::OutputPin;

use crate::pins::GpioPins;

pub struct RelayController {
    gpio_pins: GpioPins,
    bank_states: [u8; 4],
}

impl RelayController {
    pub fn new(gpio_pins: GpioPins) -> Self {
        info!("Initializing Relay Controller with 4 banks of 8 relays");
        Self {
            gpio_pins,
            bank_states: [0; 4],
        }
    }
    
    pub fn set_relay(&mut self, bank: u8, relay: u8, state: bool) -> Result<(), &'static str> {
        if bank >= 4 {
            return Err("Bank index must be 0-3");
        }
        if relay >= 8 {
            return Err("Relay index must be 0-7");
        }
        
        // Update hardware
        match bank {
            0 => self.gpio_pins.set_bank0_relay(relay, state),
            1 => self.gpio_pins.set_bank1_relay(relay, state),
            2 => self.gpio_pins.set_bank2_relay(relay, state),
            3 => self.gpio_pins.set_bank3_relay(relay, state),
            _ => return Err("Invalid bank"),
        }
        
        // Update software state
        let bank_state = &mut self.bank_states[bank as usize];
        if state {
            *bank_state |= 1 << relay;
        } else {
            *bank_state &= !(1 << relay);
        }
        
        info!("Set bank {}, relay {} to {}", bank, relay, state);
        Ok(())
    }
    
    pub fn set_bank(&mut self, bank: u8, mask: u8) -> Result<(), &'static str> {
        if bank >= 4 {
            return Err("Bank index must be 0-3");
        }
        
        // Update all relays in bank
        for relay in 0..8 {
            let state = (mask & (1 << relay)) != 0;
            self.set_relay(bank, relay, state)?;
        }
        
        self.bank_states[bank as usize] = mask;
        info!("Set bank {} to mask: {:08b}", bank, mask);
        Ok(())
    }
    
    pub fn get_bank_states(&self, bank: u8) -> Option<u8> {
        if bank >= 4 {
            None
        } else {
            Some(self.bank_states[bank as usize])
        }
    }
    
    pub fn get_relay(&self, bank: u8, relay: u8) -> Option<bool> {
        if bank >= 4 || relay >= 8 {
            None
        } else {
            Some((self.bank_states[bank as usize] & (1 << relay)) != 0)
        }
    }
    
    pub fn get_all_states(&self) -> u32 {
        let mut result = 0u32;
        for (i, &state) in self.bank_states.iter().enumerate() {
            result |= (state as u32) << (i * 8);
        }
        result
    }
}