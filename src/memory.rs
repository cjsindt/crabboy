pub struct Memory {
    memory: [u8; 0xFFFF]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0xFFFF]
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        u16::from_le_bytes([
            self.memory[address as usize],
            self.memory[(address + 1) as usize]
        ])
    }

    pub fn write(&mut self, address: usize, data: &[u8]) {
        // Ensure the address is within bounds
        assert!(address + data.len() <= self.memory.len(), "Address out of bounds");

        // Write data starting at the specified address
        self.memory[address..(address + data.len())].copy_from_slice(data);
    }
}