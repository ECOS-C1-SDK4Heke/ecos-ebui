use core::fmt::Debug;
use core::prelude::rust_2024::derive;
use core::{result::Result, result::Result::Ok};

use ecos_ssc1::GpioPin;
use embedded_hal::digital::Error;
use embedded_hal::digital::ErrorKind;
use embedded_hal::digital::ErrorType;
use embedded_hal::digital::OutputPin;

#[derive(Debug)]
pub struct EbdHalGpio {
    pin: u32, // 使用排针号：1–16
}

impl EbdHalGpio {
    pub fn new(pin: u32) -> Self {
        Self { pin }
    }
}

impl Error for EbdHalGpio {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl ErrorType for EbdHalGpio {
    type Error = ErrorKind;
}

impl OutputPin for EbdHalGpio {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        GpioPin::set_level(self.pin, true);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        GpioPin::set_level(self.pin, false);
        Ok(())
    }
}
