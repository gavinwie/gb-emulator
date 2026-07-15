pub struct Bus {
    ram: [u8; 0x10000],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: [0; 0x10000],
        }
    }
    
    pub fn read_ram(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }
}