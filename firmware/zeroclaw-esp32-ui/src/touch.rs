use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::i2c::{I2c, I2cConfig, I2cDriver};
use ft6x36::{Ft6x36, TouchEvent};
use log::info;

pub struct TouchController {
    controller: Option<Ft6x36<I2c<'static>>>,
    width: u16,
    height: u16,
}

impl TouchController {
    pub fn new(
        i2c: I2c<'static>,
        interrupt_pin: PinDriver<'static, Input>,
        width: u16,
        height: u16,
    ) -> Result<Self, TouchError> {
        let controller =
            Ft6x36::new(i2c, interrupt_pin).map_err(|e| TouchError::InitFailed(e.to_string()))?;

        info!("Touch controller initialized ({}x{})", width, height);

        Ok(Self {
            controller: Some(controller),
            width,
            height,
        })
    }

    pub fn get_touch_event(&mut self) -> Option<TouchPoint> {
        let controller = self.controller.as_mut()?;

        if let Ok(event) = controller.get_touch_event() {
            match event {
                TouchEvent::Touch { x, y } => {
                    let x = x.min(self.width as u8) as u16;
                    let y = y.min(self.height as u8) as u16;
                    return Some(TouchPoint {
                        x,
                        y,
                        pressed: true,
                    });
                }
                TouchEvent::Release => {
                    return Some(TouchPoint {
                        x: 0,
                        y: 0,
                        pressed: false,
                    });
                }
                TouchEvent::None => return None,
            }
        }
        None
    }

    pub fn is_pressed(&mut self) -> bool {
        if let Some(controller) = self.controller.as_mut() {
            controller.is_pressed().unwrap_or(false)
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TouchPoint {
    pub x: u16,
    pub y: u16,
    pub pressed: bool,
}

#[derive(Debug, Clone)]
pub enum TouchError {
    InitFailed(String),
    ReadFailed(String),
    NotInitialized,
}

impl core::fmt::Display for TouchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TouchError::InitFailed(e) => write!(f, "Touch init failed: {}", e),
            TouchError::ReadFailed(e) => write!(f, "Touch read failed: {}", e),
            TouchError::NotInitialized => write!(f, "Touch not initialized"),
        }
    }
}
