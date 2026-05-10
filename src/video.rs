//! RAM-backed monochrome video view.

use core::fmt;

use crate::memory::Memory;

pub const VIDEO_BASE: u16 = 0x2000;
pub const VIDEO_WIDTH: usize = 64;
pub const VIDEO_HEIGHT: usize = 32;
pub const VIDEO_BYTES_PER_ROW: usize = VIDEO_WIDTH / 8;
pub const VIDEO_SIZE_BYTES: usize = VIDEO_BYTES_PER_ROW * VIDEO_HEIGHT;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct VideoView {
    base: u16,
    width: usize,
    height: usize,
}

impl VideoView {
    pub const fn new(base: u16) -> Self {
        Self {
            base,
            width: VIDEO_WIDTH,
            height: VIDEO_HEIGHT,
        }
    }

    pub const fn elf_64x32() -> Self {
        Self::new(VIDEO_BASE)
    }

    pub const fn base(self) -> u16 {
        self.base
    }

    pub const fn width(self) -> usize {
        self.width
    }

    pub const fn height(self) -> usize {
        self.height
    }

    pub const fn size_bytes(self) -> usize {
        (self.width / 8) * self.height
    }

    pub fn pixel(self, mem: &Memory, x: usize, y: usize) -> Option<bool> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let bytes_per_row = self.width / 8;
        let byte_offset = y * bytes_per_row + x / 8;
        let bit = 7 - (x % 8);
        let byte = mem.read_byte(self.base.wrapping_add(byte_offset as u16));
        Some(byte & (1 << bit) != 0)
    }

    pub fn render_text(self, mem: &Memory) -> String {
        let mut out = String::with_capacity((self.width + 1) * self.height);
        self.write_text(mem, &mut out).expect("write to String");
        out
    }

    pub fn write_text(self, mem: &Memory, out: &mut dyn fmt::Write) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.pixel(mem, x, y).unwrap_or(false) {
                    '#'
                } else {
                    '.'
                };
                out.write_char(c)?;
            }
            if y + 1 < self.height {
                out.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl Default for VideoView {
    fn default() -> Self {
        Self::elf_64x32()
    }
}
