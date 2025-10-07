// src/modbus.rs
use modbus_rs::{rtu::Server, DataModel, Coils, DiscreteInputs, HoldingRegisters, InputRegisters};
use crate::relays::RelayController;

pub struct ModbusHandler {
    relay_controller: RelayController,
    coils: Coils,
}

impl ModbusHandler {
    pub fn new(relay_controller: RelayController) -> Self {
        let mut coils = Coils::new(32); // 32 coils for our 32 relays
        // Initialize all coils to off
        for i in 0..32 {
            coils.set(i, false).unwrap();
        }

        Self {
            relay_controller,
            coils,
        }
    }

    pub async fn run_modbus_server(&mut self) -> Result<(), modbus_rs::Error> {
        let mut data_model = DataModel::new();
        data_model.coils = self.coils.clone();

        let mut server = Server::new("/dev/ttyUSB0", 9600)?; // Simulated port
        
        loop {
            let request = server.accept().await?;
            
            match request.function_code {
                0x01 => self.handle_read_coils(&request).await,
                0x05 => self.handle_write_single_coil(&request).await,
                0x0F => self.handle_write_multiple_coils(&request).await,
                _ => server.send_exception(0x01).await?, // Illegal function
            }
        }
    }

    async fn handle_read_coils(&mut self, request: &Request) -> Result<(), modbus_rs::Error> {
        let start_addr = request.data[0] as u16 | (request.data[1] as u16) << 8;
        let quantity = request.data[2] as u16 | (request.data[3] as u16) << 8;
        
        if start_addr + quantity > 32 {
            return server.send_exception(0x02).await; // Illegal data address
        }

        let mut response_data = Vec::new();
        for i in start_addr..(start_addr + quantity) {
            let bank = (i / 8) as u8;
            let relay = (i % 8) as u8;
            let state = self.relay_controller.get_relay_state(bank, relay).unwrap_or(false);
            response_data.push(state);
        }

        server.send_response(&response_data).await
    }

    async fn handle_write_single_coil(&mut self, request: &Request) -> Result<(), modbus_rs::Error> {
        let address = request.data[0] as u16 | (request.data[1] as u16) << 8;
        let value = request.data[2] == 0xFF; // Modbus convention: 0xFF00 = ON, 0x0000 = OFF
        
        if address >= 32 {
            return server.send_exception(0x02).await;
        }

        let bank = (address / 8) as u8;
        let relay = (address % 8) as u8;
        
        if let Err(_) = self.relay_controller.set_relay(bank, relay, value) {
            return server.send_exception(0x04).await; // Slave device failure
        }

        // Echo back the same data as confirmation
        server.send_response(&request.data).await
    }

    async fn handle_write_multiple_coils(&mut self, request: &Request) -> Result<(), modbus_rs::Error> {
        let start_addr = request.data[0] as u16 | (request.data[1] as u16) << 8;
        let quantity = request.data[2] as u16 | (request.data[3] as u16) << 8;
        let byte_count = request.data[4] as usize;
        
        if start_addr + quantity > 32 {
            return server.send_exception(0x02).await;
        }

        for i in 0..quantity {
            let byte_index = 5 + (i as usize / 8);
            let bit_index = i % 8;
            let byte_val = request.data[byte_index];
            let state = (byte_val & (1 << bit_index)) != 0;
            
            let bank = ((start_addr + i) / 8) as u8;
            let relay = ((start_addr + i) % 8) as u8;
            
            if let Err(_) = self.relay_controller.set_relay(bank, relay, state) {
                return server.send_exception(0x04).await;
            }
        }

        // Send success response
        server.send_response(&request.data[0..4]).await
    }
}