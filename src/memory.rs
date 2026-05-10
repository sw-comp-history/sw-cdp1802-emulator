//! Byte-addressed memory.

use sw_isa_core::DecodeError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Memory {
    bytes: Vec<u8>,
}

impl Memory {
    pub fn new(size_bytes: usize) -> Self {
        Self {
            bytes: vec![0; size_bytes],
        }
    }

    pub fn size_bytes(&self) -> usize {
        self.bytes.len()
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.bytes.get(addr as usize).copied().unwrap_or(0)
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        if let Some(slot) = self.bytes.get_mut(addr as usize) {
            *slot = value;
        }
    }

    pub fn load_bytes(&mut self, start: u16, bytes: &[u8]) {
        let start = start as usize;
        if start >= self.bytes.len() {
            return;
        }
        let available = self.bytes.len() - start;
        let n = available.min(bytes.len());
        self.bytes[start..start + n].copy_from_slice(&bytes[..n]);
    }

    pub fn read_range(&self, start: u16, len: usize) -> Vec<u8> {
        (0..len)
            .map(|i| self.read_byte(start.wrapping_add(i as u16)))
            .collect()
    }

    pub fn fetch_bytes(&self, addr: u16) -> [u8; 2] {
        [self.read_byte(addr), self.read_byte(addr.wrapping_add(1))]
    }

    pub fn decode_at(&self, addr: u16) -> Result<(sw_cdp1802_isa::Instruction, u16), DecodeError> {
        let bytes = self.fetch_bytes(addr);
        let (insn, n) = sw_cdp1802_isa::decode::decode(&bytes)?;
        Ok((insn, n as u16))
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(65_536)
    }
}
