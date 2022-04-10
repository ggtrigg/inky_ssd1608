use i2cdev::linux::LinuxI2CDevice;
use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::{Pin, Spidev};
use embedded_hal::blocking::spi::Write;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*
};
use std::io::{Error, ErrorKind};
use std::thread::sleep;
use std::time::Duration;

mod eeprom;
use eeprom::EEPType;

const EEP_ADDRESS: u16 = 0x50;
const SPI_CHUNK_SIZE: usize = 4096;
const SPI_COMMAND: u8 = 0;
const SPI_DATA: u8 = 1;
const DRIVER_CONTROL: u8 = 0x01;
const _GATE_VOLTAGE: u8 = 0x03;
const _SOURCE_VOLTAGE: u8 = 0x04;
const _DISPLAY_CONTROL: u8 = 0x07;
const _NON_OVERLAP: u8 = 0x0B;
const _BOOSTER_SOFT_START: u8 = 0x0C;
const _GATE_SCAN_START: u8 = 0x0F;
const _DEEP_SLEEP: u8 = 0x10;
const DATA_MODE: u8 = 0x11;
const _SW_RESET: u8 = 0x12;
const _TEMP_WRITE: u8 = 0x1A;
const _TEMP_READ: u8 = 0x1B;
const _TEMP_CONTROL: u8 = 0x1C;
const _TEMP_LOAD: u8 = 0x1D;
const MASTER_ACTIVATE: u8 = 0x20;
const _DISP_CTRL1: u8 = 0x21;
const _DISP_CTRL2: u8 = 0x22;
const WRITE_RAM: u8 = 0x24;
const WRITE_ALTRAM: u8 = 0x26;
const _READ_RAM: u8 = 0x25;
const _VCOM_SENSE: u8 = 0x28;
const _VCOM_DURATION: u8 = 0x29;
const WRITE_VCOM: u8 = 0x2C;
const _READ_OTP: u8 = 0x2D;
const WRITE_LUT: u8 = 0x32;
const WRITE_DUMMY: u8 = 0x3A;
const WRITE_GATELINE: u8 = 0x3B;
const WRITE_BORDER: u8 = 0x3C;
const SET_RAMXPOS: u8 = 0x44;
const SET_RAMYPOS: u8 = 0x45;
const SET_RAMXCOUNT: u8 = 0x4E;
const SET_RAMYCOUNT: u8 = 0x4F;
const _NOP: u8 = 0xFF;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Colour {
    White,
    Black,
    Red,
    Yellow,
    RedHt,
}

#[allow(dead_code)]
pub struct Inky1608 {
    pub width: u16,
    pub height: u16,
    cols: u16,
    rows: u16,
    r_cols: u16,
    r_rows: u16,
    rotation: i16,
    offset_x: u16,
    offset_y: u16,
    colour: Colour,
    border_colour: Colour,
    lut: [u8; 30],
    cs_channel: u16,
    dc_pin: Pin,
    reset_pin: Pin,
    busy_pin: Pin,
    h_flip: bool,
    v_flip: bool,
    eeprom: eeprom::EEPType<LinuxI2CDevice>,
    spidev: Spidev,
    framebuffer: Vec<bool>
}

impl Inky1608 {
    #[allow(dead_code)]
    pub fn new(
        resolution: Option<(u16, u16)>,
        colour: Option<&str>,
        cs_channel: u16,
        dc_pin: u64,
        reset_pin: u64,
        busy_pin: u64,
        h_flip: bool,
        v_flip: bool,
        spidev: Option<Spidev>,
        i2c_bus: Option<LinuxI2CDevice>,
    ) -> Result<Inky1608, Error> {
        // Get eeprom info first so resolution and colour-type can be auto detected.
        // (Actually it seems that the eeprom reported resolution isn't correct, so it
        //  really always needs to be specified.)
        let dev = match i2c_bus {
            Some(d) => d,
            None => LinuxI2CDevice::new("/dev/i2c-1", EEP_ADDRESS)?,
        };
        let eep_type = EEPType::new(dev)?;
        
        match eep_type.display_variant {
            10 | 11 | 12 => (),
            _ => panic!("This driver is not compatible with your board.")
        };
        
        let res = match resolution {
            Some(r) => (r.0, r.1),
            None => (eep_type.width, eep_type.height)
        };
        
        let (cols, rows, rotation, offset_x, offset_y) = match res {
            (250, 122) => Ok((136, 250, -90, 0, 6)),
            _ => Err(Error::new(ErrorKind::Other, "invalid resolution")),
        }?;
        
        let (r_cols, r_rows) = match rotation {
            90 | -90 => (rows, cols),
            _ => (cols, rows)
        };
        
        let col = match colour {
            Some(c) => c,
            None => eep_type.colour_name()
        };

        let colour = match col {
            "red" => Ok(Colour::Red),
            "black" => Ok(Colour::Black),
            "yellow" => Ok(Colour::Yellow),
            _ => Err(Error::new(ErrorKind::Other, "invalid colour")),
        }?;

        let spibus = match spidev {
            Some(b) => b,
            None => {
                let mut spi = Spidev::open(format!("/dev/spidev0.{}", cs_channel))?;
                let options = SpidevOptions::new()
                    .bits_per_word(8)
                    .max_speed_hz(488_000)
                    .mode(SpiModeFlags::SPI_MODE_0)
                    .build();
                spi.configure(&options)?;
                spi
            }
        };

        let inky = Inky1608 {
            width: res.0,
            height: res.1,
            cols,
            rows,
            r_cols,
            r_rows,
            rotation,
            offset_x,
            offset_y,
            colour,
            border_colour: Colour::White,
            lut: get_lut(&colour),
            cs_channel,
            dc_pin: Pin::new(dc_pin),
            reset_pin: Pin::new(reset_pin),
            busy_pin: Pin::new(busy_pin),
            h_flip,
            v_flip,
            eeprom: eep_type,
            spidev: spibus,
            framebuffer: vec![false; (cols * rows).into()]
        };
        Ok(inky)
    }

    fn setup(&mut self) -> Result<(), linux_embedded_hal::sysfs_gpio::Error> {
        self.busy_pin.export()?;
        while !self.busy_pin.is_exported() {}
        self.busy_pin.set_direction(Direction::In)?;

        self.dc_pin.export()?;
        while !self.dc_pin.is_exported() {}
        self.dc_pin.set_direction(Direction::Out)?;
        self.dc_pin.set_value(0)?;

        self.reset_pin.export()?;
        while !self.reset_pin.is_exported() {}
        self.reset_pin.set_direction(Direction::Out)?;
        self.reset_pin.set_value(1)?;

        let mut delay = Duration::new(0, 500_000);
        self.reset_pin.set_value(0)?;
        sleep(delay);
        self.reset_pin.set_value(1)?;
        sleep(delay);
        self.send_command(0x12, None)?; // Soft reset
        delay = Duration::new(1, 0);
        sleep(delay);
        self.busy_wait()
    }
    
    #[allow(dead_code)]
    fn update(&mut self, buf_a: Vec<u8>, buf_b: Vec<u8>, busy_wait: bool) -> Result<(), linux_embedded_hal::sysfs_gpio::Error> {
        self.setup()?;
        
        let mut packed_height = vec![((self.rows - 1) & 0xff) as u8, ((self.rows - 1) >> 8) as u8];

        let mut temp = packed_height.clone();
        temp.push(0x00 as u8);
        self.send_command(DRIVER_CONTROL, Some(&temp as &[u8]))?;  // Gate setting

        self.send_command(WRITE_DUMMY, Some(&[0x1B]))?;    // Set dummy line period
        self.send_command(WRITE_GATELINE, Some(&[0x0B]))?;    // Set Line Width

        self.send_command(DATA_MODE, Some(&[0x03]))?;    // Data entry squence (scan direction leftward and downward)
        self.send_command(SET_RAMXPOS, Some(&[0x00, ((self.cols / 8) - 1) as u8]))?;    // Set ram X start and end position
        let mut temp = vec![0x00 as u8, 0x00 as u8];
        temp.append(&mut packed_height);
        self.send_command(SET_RAMYPOS, Some(&temp))?;    // Set ram Y start and end position

        self.send_command(WRITE_VCOM, Some(&[0x70]))?;    // VCOM Voltage

        let lut = self.lut;
        self.send_command(WRITE_LUT, Some(&lut))?;   // Write LUT DATA

        match self.border_colour {
          Colour::Black => self.send_command(WRITE_BORDER, Some(&[0x00]))?,     // GS Transition Define A + VSS + LUT0
          Colour::Red => if self.colour == Colour::Red { self.send_command(WRITE_BORDER, Some(&[0b00000110]))? },   // Fix Level Define A + VSH2 + LUT3
          Colour::Yellow => if self.colour == Colour::Yellow { self.send_command(WRITE_BORDER, Some(&[0b00001111]))? },   // GS Transition Define A + VSH2 + LUT3
          Colour::White => self.send_command(WRITE_BORDER, Some(&[0b00000001]))?,   // GS Transition Define A + VSH2 + LUT1
          _ => ()
        };
        
        // Set RAM address to 0, 0
        self.send_command(SET_RAMXCOUNT, Some(&[0x00]))?;
        self.send_command(SET_RAMYCOUNT, Some(&[0x00, 0x00]))?;

        // Do RAM B/W
        self.send_command(WRITE_RAM, Some(&buf_a))?;
        // & Yellow/Red
        self.send_command(WRITE_ALTRAM, Some(&buf_b))?;
        
        if busy_wait {
            self.busy_wait()?;
        }
        self.send_command(MASTER_ACTIVATE, None)?;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn flush(&mut self) -> Result<(), linux_embedded_hal::sysfs_gpio::Error> {
        // Need to convert the framebuffer which is Vec<bool> into Vec<u8> where
        // each bit is a pixel.
        let mut destvec : Vec<u8> = vec![];
        for bytes in self.framebuffer.chunks(8) {
            let mut dest : u8 = 0x0;
            for byte in bytes {
                if *byte {
                    dest = (dest << 1) | 0x1;
                } else {
                    dest = dest << 1;
                }
            }
            destvec.push(dest ^ 0xff);
        }
        self.update(destvec, vec![0x0; (self.cols * self.rows).into()], true)?;
        Ok(())
    }
    
    fn busy_wait(&mut self) -> Result<(), linux_embedded_hal::sysfs_gpio::Error> {
        let delay = Duration::new(0, 10_000);
        while self.busy_pin.get_value()? != 0 {
            sleep(delay);
        }
        Ok(())
    }
    
    fn send_command(&mut self, command: u8, data: Option<&[u8]>) -> Result<(), linux_embedded_hal::sysfs_gpio::Error> {
        self.spi_write(SPI_COMMAND, &[command])?;
        match data {
            Some(d) => self.spi_write(SPI_DATA, d),
            None => Ok(())
        }
    }
    
    #[allow(dead_code)]
    fn send_data(&mut self, data: &[u8]) -> Result<(), linux_embedded_hal::sysfs_gpio::Error> {
        self.spi_write(SPI_DATA, data)
    }
    
    #[allow(dead_code)]
    pub fn set_border(&mut self, colour: Colour) {
        self.border_colour = match colour {
            Colour::Black => Colour::Black,
            Colour::White => Colour::White,
            Colour::Red => Colour::Red,
            Colour::Yellow => Colour::Yellow,
            _ => self.border_colour
        }
    }
    
    #[allow(dead_code)]
    pub fn ident(&self) {
        println!("Inky is {} rows x {} cols, rotation {} and colour {}", self.rows, self.cols, self.rotation, match self.colour {
            Colour::Black => "black",
            Colour::Red => "red",
            Colour::Yellow => "yellow",
            _ => "unknown!!"
        });
    }
    
    fn spi_write(&mut self, dc: u8, data: &[u8]) -> Result<(), linux_embedded_hal::sysfs_gpio::Error> {
        self.dc_pin.set_value(dc)?;
        if cfg!(target_os = "linux") {
            for data_chunk in data.chunks(SPI_CHUNK_SIZE) {
                self.spidev.write(data_chunk)?;
            }
        } else {
            self.spidev.write(data)?;
        }
        Ok(())
    }
}

fn get_lut(colour: &Colour) -> [u8; 30] {
    match colour {
        Colour::Black => [
            0x02, 0x02, 0x01, 0x11, 0x12, 0x12, 0x22, 0x22, 0x66, 0x69,
            0x69, 0x59, 0x58, 0x99, 0x99, 0x88, 0x00, 0x00, 0x00, 0x00,
            0xF8, 0xB4, 0x13, 0x51, 0x35, 0x51, 0x51, 0x19, 0x01, 0x00
        ],
        Colour::Red => [
            0x02, 0x02, 0x01, 0x11, 0x12, 0x12, 0x22, 0x22, 0x66, 0x69,
            0x69, 0x59, 0x58, 0x99, 0x99, 0x88, 0x00, 0x00, 0x00, 0x00,
            0xF8, 0xB4, 0x13, 0x51, 0x35, 0x51, 0x51, 0x19, 0x01, 0x00
        ],
        Colour::Yellow => [
            0x02, 0x02, 0x01, 0x11, 0x12, 0x12, 0x22, 0x22, 0x66, 0x69,
            0x69, 0x59, 0x58, 0x99, 0x99, 0x88, 0x00, 0x00, 0x00, 0x00,
            0xF8, 0xB4, 0x13, 0x51, 0x35, 0x51, 0x51, 0x19, 0x01, 0x00
        ],
        _ => [ // default to black
            0x02, 0x02, 0x01, 0x11, 0x12, 0x12, 0x22, 0x22, 0x66, 0x69,
            0x69, 0x59, 0x58, 0x99, 0x99, 0x88, 0x00, 0x00, 0x00, 0x00,
            0xF8, 0xB4, 0x13, 0x51, 0x35, 0x51, 0x51, 0x19, 0x01, 0x00
        ],
    }
}

impl DrawTarget for Inky1608 {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;
    
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, colour) in pixels.into_iter() {
            if coord.x >= 0 && coord.x < self.r_cols.into() && coord.y >= 0 && coord.y < self.r_rows.into() {
                let offset : u16 = match self.rotation {
                    90 | -90 => coord.x as u16 * self.cols + (self.cols - coord.y as u16),
                    _ => coord.y as u16 * self.r_cols + coord.x as u16
                };
                self.framebuffer[offset as usize] = colour.is_on();
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Inky1608 {
    fn size(&self) -> Size {
        Size::new(self.r_cols.into(), self.r_rows.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn bad_resolution() {
        Inky1608::new(Some((27, 10)), Some("black"), 8, 22, 27, 17, false, false, None, None).expect("bad resolution");
    }

    #[test]
    #[should_panic]
    fn bad_colour() {
        Inky1608::new(Some((212, 104)), Some("purple"), 8, 22, 27, 17, false, false, None, None).expect("bad colour");
    }

    #[test]
    fn new() {
        let inky = Inky1608::new(Some((212, 104)), Some("black"), 0, 22, 27, 17, false, false, None, None).expect("inky new");
        assert_eq!(inky.cols, 104);
        assert_eq!(inky.rows, 212);
        assert_eq!(inky.rotation, -90);
        assert_eq!(inky.colour, Colour::Black);
    }
}
