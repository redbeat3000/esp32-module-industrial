// src/pins.rs
pub struct RelayPins {
    // Bank 0: GPIO 0-7
    pub bank0: [OutputPin; 8],
    // Bank 1: GPIO 8-15  
    pub bank1: [OutputPin; 8],
    // Bank 2: GPIO 16-23
    pub bank2: [OutputPin; 8],
    // Bank 3: GPIO 24-31  
    pub bank3: [OutputPin; 8],
}

pub struct CommunicationPins {
    // RS485 Modbus RTU
    pub uart_tx: u32,    // GPIO 17
    pub uart_rx: u32,    // GPIO 16
    pub rs485_de: u32,   // GPIO 4 (Driver Enable)
    
    // Status LEDs
    pub status_led: u32, // GPIO 2
    pub fault_led: u32,  // GPIO 15
}

impl Default for CommunicationPins {
    fn default() -> Self {
        Self {
            uart_tx: 17,
            uart_rx: 16, 
            rs485_de: 4,
            status_led: 2,
            fault_led: 15,
        }
    }
}