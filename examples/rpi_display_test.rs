// On newer pis, need this dtoverlay=spi0-1cs,cs0_pin=25 added to /boot/config.txt
// to free up pin 8 for inky's CS pin.

extern crate inky_ssd1608;
use inky_ssd1608::{Inky1608, Colour};
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};

// Graphics
// #[macro_use]
extern crate embedded_graphics;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::Text;

// Font
extern crate profont;
use profont::{PROFONT_14_POINT, PROFONT_10_POINT};

fn main() {
    let raw_image = ImageRaw::<BinaryColor>::new(RUSTLOGO, 64);
    let mut inky = Inky1608::new(Some((250, 122)), None, 0, 22, 27, 17, false, false, None, None).expect("inky");
    let inky_info = format!("{}", inky);
    inky.set_border(Colour::Black);
    let mut style = MonoTextStyle::new(&PROFONT_10_POINT, BinaryColor::On);
    Text::new(&inky_info, Point::new(10, 20), style).draw(&mut inky).expect("text");
    style = MonoTextStyle::new(&PROFONT_14_POINT, BinaryColor::On);
    Text::new("Inky pHat\ndisplay driver", Point::new(10, 100), style).draw(&mut inky).expect("text");
    Image::new(&raw_image, Point::new(170, 60)).draw(&mut inky).expect("image");
    inky.flush().unwrap();
}

#[rustfmt::skip]
const RUSTLOGO: &[u8] = &[
    0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b01100001,
    0b10000110, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b01110011,
    0b11001110, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00011000, 0b01111111,
    0b11111110, 0b00011000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00011111, 0b11111111,
    0b11111111, 0b11111000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00011111, 0b11111111,
    0b11111111, 0b11111000, 0b00000000, 0b00000000,
    0b00000000, 0b00000011, 0b10011111, 0b11111100,
    0b00111111, 0b11111001, 0b11000000, 0b00000000,
    0b00000000, 0b00000011, 0b11111111, 0b11111100,
    0b00111111, 0b11111111, 0b11000000, 0b00000000,
    0b00000000, 0b00000011, 0b11111111, 0b11111100,
    0b00111111, 0b11111111, 0b11000000, 0b00000000,
    0b00000000, 0b00000011, 0b11111111, 0b00000110,
    0b01100000, 0b11111111, 0b11000000, 0b00000000,
    0b00000000, 0b00111111, 0b11111100, 0b00000011,
    0b11000000, 0b00111111, 0b11111100, 0b00000000,
    0b00000000, 0b00111111, 0b11110000, 0b00000001,
    0b10000000, 0b00001111, 0b11111100, 0b00000000,
    0b00000000, 0b00111111, 0b11000000, 0b00000000,
    0b00000000, 0b00000011, 0b11111100, 0b00000000,
    0b00000000, 0b00111111, 0b10000000, 0b00000000,
    0b00000000, 0b00000001, 0b11111100, 0b00000000,
    0b00000011, 0b11111111, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b11111111, 0b11000000,
    0b00000011, 0b11111110, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b01111111, 0b11000000,
    0b00000011, 0b11111111, 0b11111111, 0b11111111,
    0b11111111, 0b00000000, 0b00111111, 0b11000000,
    0b00000001, 0b11111111, 0b11111111, 0b11111111,
    0b11111111, 0b11100000, 0b00011111, 0b10000000,
    0b00000001, 0b11111111, 0b11111111, 0b11111111,
    0b11111111, 0b11111000, 0b00001111, 0b10000000,
    0b00011111, 0b11111111, 0b11111111, 0b11111111,
    0b11111111, 0b11111100, 0b00001111, 0b11111000,
    0b00011111, 0b11111111, 0b11111111, 0b11111111,
    0b11111111, 0b11111110, 0b00000111, 0b11111000,
    0b00001111, 0b11111111, 0b11111111, 0b11111111,
    0b11111111, 0b11111110, 0b00001111, 0b11110000,
    0b00001111, 0b11111111, 0b11111111, 0b11111111,
    0b11111111, 0b11111111, 0b00001111, 0b11110000,
    0b00001110, 0b00011000, 0b11111111, 0b11000000,
    0b00001111, 0b11111111, 0b00011000, 0b11110000,
    0b00001110, 0b00011000, 0b11111111, 0b11000000,
    0b00000111, 0b11111111, 0b00010000, 0b01110000,
    0b01111110, 0b00011000, 0b11111111, 0b11000000,
    0b00000011, 0b11111111, 0b00111000, 0b01111110,
    0b01111111, 0b00111000, 0b11111111, 0b11000000,
    0b00000011, 0b11111110, 0b00111000, 0b11111110,
    0b00111111, 0b11110000, 0b11111111, 0b11000000,
    0b00000111, 0b11111110, 0b00001111, 0b11111100,
    0b00011111, 0b11000000, 0b11111111, 0b11111111,
    0b11111111, 0b11111100, 0b00000011, 0b11111000,
    0b00011111, 0b00000000, 0b11111111, 0b11111111,
    0b11111111, 0b11110000, 0b00000000, 0b11111000,
    0b00111111, 0b00000000, 0b11111111, 0b11111111,
    0b11111111, 0b11100000, 0b00000000, 0b11111100,
    0b01111111, 0b00000000, 0b11111111, 0b11111111,
    0b11111111, 0b11110000, 0b00000000, 0b11111110,
    0b01111111, 0b00000000, 0b11111111, 0b11111111,
    0b11111111, 0b11111000, 0b00000000, 0b11111110,
    0b00111111, 0b00000000, 0b11111111, 0b11111111,
    0b11111111, 0b11111100, 0b00000000, 0b11111100,
    0b00011111, 0b00000000, 0b11111111, 0b11000000,
    0b00011111, 0b11111100, 0b00000111, 0b11111000,
    0b00011111, 0b10000000, 0b11111111, 0b11000000,
    0b00001111, 0b11111100, 0b00000111, 0b11111000,
    0b00111111, 0b10000000, 0b11111111, 0b11000000,
    0b00001111, 0b11111110, 0b00000111, 0b11111100,
    0b01111111, 0b10000000, 0b11111111, 0b11000000,
    0b00000111, 0b11111110, 0b00001111, 0b11111110,
    0b01111111, 0b11000000, 0b11111111, 0b11100000,
    0b00000111, 0b11111111, 0b10011111, 0b11111110,
    0b00001111, 0b11111111, 0b11111111, 0b11111111,
    0b00000111, 0b11111111, 0b11111111, 0b11110000,
    0b00001111, 0b11111111, 0b11111111, 0b11111111,
    0b00000111, 0b11111111, 0b11111111, 0b11110000,
    0b00001111, 0b11111111, 0b11111111, 0b11111111,
    0b00000011, 0b11111111, 0b11111111, 0b11110000,
    0b00001111, 0b11111111, 0b11111111, 0b11111111,
    0b00000011, 0b11111111, 0b11111111, 0b11110000,
    0b00011111, 0b11111111, 0b11111111, 0b11111111,
    0b00000001, 0b11111111, 0b11111111, 0b11111000,
    0b00011111, 0b11111111, 0b11111111, 0b11111111,
    0b00000001, 0b11111111, 0b11111111, 0b11111000,
    0b00000001, 0b11111000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00011111, 0b10000000,
    0b00000001, 0b11111000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00011111, 0b10000000,
    0b00000011, 0b11111100, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00111111, 0b11000000,
    0b00000011, 0b11111111, 0b11111000, 0b00000000,
    0b00000000, 0b00011111, 0b11111111, 0b11000000,
    0b00000011, 0b11111111, 0b10111000, 0b00000000,
    0b00000000, 0b00011111, 0b11111111, 0b11000000,
    0b00000000, 0b00111111, 0b00001000, 0b00000000,
    0b00000000, 0b00010000, 0b11111100, 0b00000000,
    0b00000000, 0b00111111, 0b00001000, 0b00000000,
    0b00000000, 0b00110000, 0b11111100, 0b00000000,
    0b00000000, 0b00111111, 0b00001100, 0b00000000,
    0b00000000, 0b00110000, 0b11111100, 0b00000000,
    0b00000000, 0b00111111, 0b10011100, 0b00000000,
    0b00000000, 0b00111001, 0b11111100, 0b00000000,
    0b00000000, 0b00000011, 0b11111111, 0b10000000,
    0b00000001, 0b11111111, 0b11000000, 0b00000000,
    0b00000000, 0b00000011, 0b11111111, 0b11111110,
    0b01111111, 0b11111111, 0b11000000, 0b00000000,
    0b00000000, 0b00000011, 0b11111111, 0b11111111,
    0b11111111, 0b11111111, 0b11000000, 0b00000000,
    0b00000000, 0b00000011, 0b10011111, 0b11111111,
    0b11111111, 0b11111001, 0b11000000, 0b00000000,
    0b00000000, 0b00000000, 0b00011111, 0b11111111,
    0b11111111, 0b11111000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00011111, 0b11111111,
    0b11111111, 0b11111000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00011000, 0b01111111,
    0b11111110, 0b00011000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b01110011,
    0b11001110, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b01100001,
    0b10000110, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000,
    0b00000000, 0b00000000, 0b00000000, 0b00000000,
];