// src/relays.rs
use core::sync::atomic::{AtomicU32, Ordering};
use heapless::Vec;

pub struct RelayBank {
    states: AtomicU32,
    output_pins: Vec<&'static mut dyn OutputPin, 32>,
}

impl RelayBank {
    pub fn new(pins: Vec<&'static mut dyn OutputPin, 32>) -> Self {
        Self {
            states: AtomicU32::new(0),
            output_pins: pins,
        }
    }

    pub fn set_relay(&mut self, relay: u8, state: bool) -> Result<(), &'static str> {
        if relay >= 32 {
            return Err("Relay index out of bounds");
        }

        let mask = 1 << relay;
        let current_states = self.states.load(Ordering::Acquire);
        
        let new_states = if state {
            current_states | mask
        } else {
            current_states & !mask
        };

        // Update hardware
        self.output_pins[relay as usize].set_state(state.into())?;
        
        // Update atomic state
        self.states.store(new_states, Ordering::Release);
        
        Ok(())
    }

    pub fn set_bank(&mut self, mask: u32) -> Result<(), &'static str> {
        for i in 0..32 {
            let state = (mask & (1 << i)) != 0;
            self.output_pins[i as usize].set_state(state.into())?;
        }
        self.states.store(mask, Ordering::Release);
        Ok(())
    }

    pub fn get_states(&self) -> u32 {
        self.states.load(Ordering::Acquire)
    }

    pub fn get_relay(&self, relay: u8) -> Option<bool> {
        if relay >= 32 {
            None
        } else {
            let states = self.states.load(Ordering::Acquire);
            Some((states & (1 << relay)) != 0)
        }
    }
}

// Thread-safe relay controller
pub struct RelayController {
    banks: [RelayBank; 4],
}

impl RelayController {
    pub fn new() -> Self {
        // Initialize with dummy pins for simulation
        // In real hardware, these would be actual GPIO pins
        let dummy_bank = RelayBank::new(Vec::new());
        Self {
            banks: [dummy_bank; 4],
        }
    }

    pub fn set_relay(&mut self, bank: u8, relay: u8, state: bool) -> Result<(), &'static str> {
        if bank >= 4 {
            return Err("Bank index out of bounds");
        }
        self.banks[bank as usize].set_relay(relay, state)
    }

    pub fn set_bank(&mut self, bank: u8, mask: u8) -> Result<(), &'static str> {
        if bank >= 4 {
            return Err("Bank index out of bounds");
        }
        self.banks[bank as usize].set_bank(mask as u32)
    }

    pub fn get_bank_states(&self, bank: u8) -> Option<u8> {
        if bank >= 4 {
            None
        } else {
            Some(self.banks[bank as usize].get_states() as u8)
        }
    }
}