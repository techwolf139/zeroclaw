//! ZeroClaw ESP32 UI - Hardware Pin Configuration
//!
//! Default pin mapping for ESP32-S3 with ST7789 display and FT6X36 touch controller

use esp_idf_svc::hal::gpio::*;

// Display pins (SPI)
pub const PIN_SPI_SCK: i32 = 6;
pub const PIN_SPI_MOSI: i32 = 7;
pub const PIN_SPI_MISO: i32 = 8;
pub const PIN_SPI_CS: i32 = 10;
pub const PIN_DISPLAY_DC: i32 = 4;
pub const PIN_DISPLAY_RST: i32 = 3;
pub const PIN_DISPLAY_BL: i32 = 5;

// Touch controller pins (I2C)
pub const PIN_I2C_SDA: i32 = 1;
pub const PIN_I2C_SCL: i32 = 2;
pub const PIN_TOUCH_INT: i32 = 11;

pub struct Pins {
    pub spi_clk: i32,
    pub spi_mosi: i32,
    pub spi_miso: i32,
    pub spi_cs: i32,
    pub display_dc: i32,
    pub display_rst: i32,
    pub display_bl: i32,
    pub i2c_sda: i32,
    pub i2c_scl: i32,
    pub touch_int: i32,
}

impl Default for Pins {
    fn default() -> Self {
        Self {
            spi_clk: PIN_SPI_SCK,
            spi_mosi: PIN_SPI_MOSI,
            spi_miso: PIN_SPI_MISO,
            spi_cs: PIN_SPI_CS,
            display_dc: PIN_DISPLAY_DC,
            display_rst: PIN_DISPLAY_RST,
            display_bl: PIN_DISPLAY_BL,
            i2c_sda: PIN_I2C_SDA,
            i2c_scl: PIN_I2C_SCL,
            touch_int: PIN_TOUCH_INT,
        }
    }
}
