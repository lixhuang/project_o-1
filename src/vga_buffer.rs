#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | foreground as u8)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[ScreenChar;BUFFER_WIDTH];BUFFER_HEIGHT],
}

use core::ptr::Unique;

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl ::core::fmt::Write for Writer {
    // add code here
////
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer().chars[row][col] = ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                };
                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        for row in 0..(BUFFER_HEIGHT - 1) {
            let buffer = self.buffer();
            buffer.chars[row] = buffer.chars[row + 1];
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        self.buffer().chars[row] = [blank; BUFFER_WIDTH];
    }
}

use spin::Mutex;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

macro_rules! print {
    ($($arg:tt)*) => ( {
        use core::fmt::Write;
        let mut writer = $crate::vga_buffer::WRITER.lock();
        writer.write_fmt(format_args!($($arg)*)).unwrap();
    } );
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}


pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}

