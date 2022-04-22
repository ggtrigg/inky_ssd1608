// Read the eeprom on a pimoroni inky e-paper display to determine it's type/characteristics.
extern crate i2cdev;

use std::fmt;
use i2cdev::core::*;
use i2cdev::linux::LinuxI2CError;

#[allow(dead_code)]
pub struct EEPType<T: I2CDevice> {
    pub width: u16,
    pub height: u16,
    pub colour: u8,
    pcb_variant: u8,
    pub display_variant: u8,
    i2cdev: T
}

impl<T: I2CDevice> fmt::Display for EEPType<T>
where
    T: I2CDevice, LinuxI2CError: std::convert::From<<T as I2CDevice>::Error>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nDisplay: {}x{}\nColour: {}", self.display_name(), self.width, self.height, self.colour_name())
    }
}

impl<T> EEPType<T>
where
    T: I2CDevice, LinuxI2CError: std::convert::From<<T as I2CDevice>::Error>
{
    #[allow(dead_code)]
    pub fn new(mut i2c_dev: T) -> Result<EEPType<T>, LinuxI2CError> {
        i2c_dev.smbus_write_i2c_block_data(0x00, &[0x00])?;
        let data = i2c_dev.smbus_read_i2c_block_data(0, 29)?;
        let eep_type = EEPType {
            width: (data[0] + (data[1] << 1)).into(),
            height: (data[2] + (data[3] << 1)).into(),
            colour: data[4],
            pcb_variant: data[5],
            display_variant: data[6],
            i2cdev: i2c_dev
        };
        Ok(eep_type)
    }

    pub fn colour_name(&self) -> &str {
        VALID_COLOURS[usize::from(self.colour)]
    }
    
    fn display_name(&self) -> &str {
        match DISPLAY_VARIANT[usize::from(self.display_variant)] {
            Some(n) => n,
            None => "unknown"
        }
    }
}

// const EEP_ADDRESS: u16 = 0x50;
const VALID_COLOURS: [&str; 6] = [
    "unknown",
    "black",
    "red",
    "yellow",
    "unknown",
    "7colour"
];

const DISPLAY_VARIANT: [Option<&str>; 17] = [
    None,
    Some("Red pHAT (High-Temp)"),
    Some("Yellow wHAT"),
    Some("Black wHAT"),
    Some("Black pHAT"),
    Some("Yellow pHAT"),
    Some("Red wHAT"),
    Some("Red wHAT (High-Temp)"),
    Some("Red wHAT"),
    None,
    Some("Black pHAT (SSD1608)"),
    Some("Red pHAT (SSD1608)"),
    Some("Yellow pHAT (SSD1608)"),
    None,
    Some("7-Colour (UC8159)"),
    None,
    Some("7-Colour 640x400 (UC8159)")
];

#[cfg(test)]
mod tests {
    use super::*;
    use i2cdev::mock::MockI2CDevice;
    
    #[test]
    fn eeptype_new() {
        let i2cdev = MockI2CDevice::new();
        let eep_type = EEPType::new(i2cdev);
        match eep_type {
            Ok(e) => assert_eq!(e.width, 0),
            Err(e) => panic!("EEPType errored! ({})", e)
        }
    }
}