//! this file contains VGA related things

#![no_std]

/// C-like enum for representing VGA colors
/// all of these colors are represented using 
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// this struct is used to represent ColorCodes
/// which includes:
///     fg (foreground): Color
///     bg (backgroung): Color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]

struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]

/// this struct is used to represent a screen char
/// it includes:
///     ascii_char (ASCII char): u8
///     color_code (a color code): ColorCode
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

/// consts that tell us the buffer width and size
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// struct to represent our Buffer
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// this struct is used to write to the screen
pub struct Writer {
    column_pos: usize,
    color_code: ColorCode,
    buff: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.newline(), // will impl this later

            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.newline();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;

                let color_code = self.color_code;
                self.buff.chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    color_code
                };
                self.column_pos += 1;
            }
        }
    }
    pub fn newline(&mut self) {
        /* TODO  */
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            // check if byte is ascii or not
            0x20..=0x7e | b'\n' => self.write_byte(byte),
            
            // if not, print ■ char
            _ => self.write_byte(0xfe);
        }
    }
}

// temp fn for test, remove this!!
pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
