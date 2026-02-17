use esp_idf_svc::wifi::{EspWifi, WifiDriver};
use esp_idf_svc::eventloop::EspEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::system::OsTimer;
use log::{info, error, warn};
use heapless::String;
use embassy_time::{Duration, Timer};

use crate::protocol::UiStatus;

pub struct WifiManager {
    ssid: String<32>,
    password: String<64>,
    connected: bool,
}

impl WifiManager {
    pub fn new() -> Self {
        Self {
            ssid: String::new(),
            password: String::new(),
            connected: false,
        }
    }

    pub fn connect(&mut self, ssid: &str, password: &str) -> Result<(), WifiError> {
        self.ssid.clear();
        self.password.clear();
        
        self.ssid
            .push_str(ssid)
            .map_err(|_| WifiError::SsidTooLong)?;
        self.password
            .push_str(password)
            .map_err(|_| WifiError::PasswordTooLong)?;
        
        info!("WiFi credentials set: SSID={}", self.ssid);
        Ok(())
    }

    pub async fn connect_async(
        &mut self,
        wifi: &mut EspWifi,
        ssid: &str,
        password: &str,
    ) -> Result<(), WifiError> {
        self.connect(ssid, password)?;
        
        wifi.start()?;
        info!("WiFi starting...");
        
        wifi.scan()?;
        info!("WiFi scan complete");
        
        wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(
            esp_idf_svc::wifi::ClientConfiguration {
                ssid: heapless::String::from(ssid),
                password: heapless::String::from(password),
                ..Default::default()
            },
        ))?;
        
        wifi.connect()?;
        info!("WiFi connecting...");
        
        let max_attempts = 30;
        for i in 0..max_attempts {
            Timer::after(Duration::from_secs(1)).await;
            
            if wifi.is_connected() {
                self.connected = true;
                info!("WiFi connected successfully!");
                return Ok(());
            }
            
            if i % 5 == 0 {
                info!("WiFi connection attempt {}/{}", i + 1, max_attempts);
            }
        }
        
        error!("WiFi connection timeout");
        Err(WifiError::ConnectionTimeout)
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn disconnect(&mut self) -> Result<(), WifiError> {
        self.connected = false;
        Ok(())
    }

    pub fn get_status(&self) -> UiStatus {
        UiStatus {
            connected: self.connected,
            wifi_ssid: if self.connected {
                Some(self.ssid.clone())
            } else {
                None
            },
            signal_strength: None,
            battery_level: None,
            zeroclaw_connected: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum WifiError {
    SsidTooLong,
    PasswordTooLong,
    ConnectionTimeout,
    NotConnected,
    AlreadyConnected,
}

impl core::fmt::Display for WifiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WifiError::SsidTooLong => write!(f, "SSID too long (max 32 chars)"),
            WifiError::PasswordTooLong => write!(f, "Password too long (max 64 chars)"),
            WifiError::ConnectionTimeout => write!(f, "WiFi connection timeout"),
            WifiError::NotConnected => write!(f, "WiFi not connected"),
            WifiError::AlreadyConnected => write!(f, "WiFi already connected"),
        }
    }
}
