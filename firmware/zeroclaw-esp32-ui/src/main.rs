#![allow(unexpected_cfgs)]

mod protocol;
mod wifi;
mod client;
mod touch;

use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::spi::*;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::link_patches;
use log::{info, error};

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use mipidsi::{Builder, Orientation};

use protocol::ChatMessage;
use wifi::WifiManager;
use client::ZeroClawClient;
use touch::TouchController;

slint::include_modules!();

const DISPLAY_WIDTH: u16 = 320;
const DISPLAY_HEIGHT: u16 = 240;

struct AppState {
    wifi: WifiManager,
    client: Option<ZeroClawClient>,
}

fn main() -> anyhow::Result<()> {
    link_patches();
    EspLogger::initialize_default();
    
    info!("ZeroClaw ESP32 UI starting...");
    
    let peripherals = Peripherals::take()?;
    let pins = peripherals.pins;
    
    let spi = peripherals.spi2;
    let sclk = pins.gpio6;
    let sda = pins.gpio7;
    let sdi = pins.gpio8;
    let cs = pins.gpio10;
    let dc = PinDriver::output(pins.gpio4)?;
    let rst = PinDriver::output(pins.gpio3)?;
    let mut backlight = PinDriver::output(pins.gpio5)?;
    
    let config = SpiConfig::new()
        .baudrate(26.MHz().into())
        .data_mode(embedded_hal::spi::MODE_3);
    
    let device = SpiDeviceDriver::new_single(
        spi,
        sclk,
        sda,
        Some(sdi),
        Some(cs),
        &SpiDriverConfig::new(),
        &config,
    )?;
    
    let di = SPIInterfaceNoCS::new(device, dc);
    
    let mut display = Builder::st7789(di)
        .with_display_size(240, 320)
        .with_orientation(Orientation::Portrait)
        .with_invert(mipidsi::ColorInversion::Inverted)
        .init(&mut Ets, Some(rst))
        .map_err(|e| anyhow::anyhow!("Display init failed: {:?}", e))?;
    
    backlight.set_high()?;
    info!("Display initialized successfully");
    
    display.clear(Rgb565::BLACK).map_err(|e| anyhow::anyhow!("Clear failed: {:?}", e))?;
    
    let mut app_state = AppState {
        wifi: WifiManager::new(),
        client: None,
    };
    
    let ui = MainWindow::new()?;
    
    ui.on_send_message(move |text| {
        let text_str = text.to_string();
        info!("User message: {}", text_str);
        
        if let Some(ref mut client) = app_state.client {
            let msg = ChatMessage::new(
                protocol::MessageRole::User,
                &text_str,
            );
            info!("Message created: role={:?}", msg.role);
        }
    });
    
    let i2c = peripherals.i2c0;
    let i2c_config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c_driver = I2cDriver::new(i2c, pins.gpio1, pins.gpio2, &i2c_config)?;
    
    let touch_interrupt = PinDriver::input(pins.gpio11)?;
    
    let mut touch = match TouchController::new(
        i2c_driver,
        touch_interrupt,
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
    ) {
        Ok(t) => {
            info!("Touch controller ready");
            Some(t)
        }
        Err(e) => {
            error!("Touch init failed: {:?}", e);
            None
        }
    };
    
    ui.set_status_text(slint::SharedString::from("WiFi: Connecting..."));
    ui.set_is_connected(false);
    
    let zeroclient = ZeroClawClient::new("http://192.168.1.100:8080")?;
    app_state.client = Some(zeroclient);
    
    ui.set_status_text(slint::SharedString::from("Ready"));
    ui.set_is_connected(true);
    
    info!("ZeroClaw UI ready!");
    
    let _ = ui.run();
    
    Ok(())
}
