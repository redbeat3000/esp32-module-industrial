// src/main.rs
#![no_std]
#![no_main]

use esp32_hal::prelude::*;
use esp32_hal::gpio::GpioPin;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::BlockingWifi;
use esp_idf_svc::http::server::EspHttpServer;
use smol::Timer;

mod relays;
mod modbus;
mod ethernet;

use relays::RelayController;

#[entry]
fn main() -> ! {
    // Initialize ESP32 peripherals
    let peripherals = esp32_hal::Peripherals::take().unwrap();
    
    // Setup serial logging
    let serial = peripherals.UART0;
    esp_println::logger::init_logger_from_env();
    
    log::info!("ESP32 Relay Controller Starting...");
    
    // Initialize relay controller with GPIO pins
    let mut relay_controller = RelayController::new(peripherals.GPIO);
    
    // Start WiFi (for Wokwi simulation)
    setup_wifi();
    
    // Start async runtime
    smol::block_on(async_main(relay_controller));
    
    loop {
        // Main loop handled by async tasks
        smol::Timer::after(std::time::Duration::from_secs(1)).await;
    }
}

async fn async_main(mut relay_controller: RelayController) {
    log::info!("Starting async tasks...");
    
    // Spawn Modbus RTU task
    smol::spawn(modbus_task(relay_controller.clone())).detach();
    
    // Spawn Ethernet/HTTP task  
    smol::spawn(ethernet_task(relay_controller.clone())).detach();
    
    // Spawn system monitor
    smol::spawn(system_monitor_task()).detach();
}

async fn modbus_task(relay_controller: RelayController) {
    log::info!("Modbus RTU Task Started");
    
    // Simulate Modbus communication in Wokwi
    loop {
        // In real hardware, this would use UART2 for RS485
        simulate_modbus_communication(&relay_controller).await;
        Timer::after(std::time::Duration::from_millis(100)).await;
    }
}

async fn ethernet_task(relay_controller: RelayController) {
    log::info!("Ethernet Task Started");
    
    // For Wokwi simulation, we'll create a simple HTTP server
    // In real hardware, this could be Modbus TCP or MQTT
    
    let server = EspHttpServer::new(&Default::default()).unwrap();
    
    server
        .fn_handler("/relays", esp_idf_svc::http::Method::Get, move |request| {
            handle_relay_status(request, &relay_controller)
        })
        .fn_handler("/relays", esp_idf_svc::http::Method::Post, move |mut request| {
            handle_relay_control(&mut request, &relay_controller)
        })
        .unwrap();
    
    log::info!("HTTP Server started on http://localhost:8080");
    
    loop {
        Timer::after(std::time::Duration::from_secs(10)).await;
    }
}

fn handle_relay_status(
    _request: &mut esp_idf_svc::http::server::EspHttpRequest,
    relay_controller: &RelayController,
) -> anyhow::Result<esp_idf_svc::http::server::EspHttpResponse> {
    let status = relay_controller.get_all_states();
    let response = format!(r#"{{"relay_states": "{:032b}"}}"#, status);
    
    Ok(esp_idf_svc::http::server::EspHttpResponse::new(
        200,
        "OK",
        &[("Content-Type", "application/json")],
        response.as_bytes(),
    )?)
}

fn handle_relay_control(
    request: &mut esp_idf_svc::http::server::EspHttpRequest,
    relay_controller: &RelayController,
) -> anyhow::Result<esp_idf_svc::http::server::EspHttpResponse> {
    let mut body = [0u8; 256];
    let len = request.read(&mut body)?;
    let body_str = std::str::from_utf8(&body[..len])?;
    
    // Simple JSON parsing for simulation
    if let Some(bank_str) = body_str.strip_prefix(r#"{"bank":"#) {
        if let Some(rest) = bank_str.strip_suffix('}') {
            if let Some((bank, state)) = rest.split_once("\"state\":").and_then(|(b, s)| {
                Some((b.trim_matches('"').parse::<u8>().ok()?, s.trim_matches('"').parse::<bool>().ok()?))
            }) {
                let _ = relay_controller.set_bank(bank, if state { 0xFF } else { 0x00 });
            }
        }
    }
    
    Ok(esp_idf_svc::http::server::EspHttpResponse::new(200, "OK", &[], b"OK"))
}

async fn system_monitor_task() {
    let mut counter = 0;
    loop {
        counter += 1;
        log::info!("System monitor heartbeat: {}", counter);
        
        // Simulate watchdog and health checks
        Timer::after(std::time::Duration::from_secs(5)).await;
    }
}

fn setup_wifi() {
    // For Wokwi simulation, WiFi is simulated
    log::info!("WiFi setup complete (simulated in Wokwi)");
}

async fn simulate_modbus_communication(relay_controller: &RelayController) {
    // Simulate receiving Modbus commands
    static mut SIMULATION_COUNTER: u32 = 0;
    
    unsafe {
        if SIMULATION_COUNTER % 50 == 0 {
            // Simulate a Modbus "write single coil" command every 5 seconds
            let bank = (SIMULATION_COUNTER / 50) as u8 % 4;
            let state = SIMULATION_COUNTER % 2 == 0;
            
            log::info!("[SIM] Modbus command: Set bank {} to {}", bank, state);
            let _ = relay_controller.set_bank(bank, if state { 0xFF } else { 0x00 });
        }
        SIMULATION_COUNTER += 1;
    }
}