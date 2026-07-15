pub const ROM_START: u16    = 0x0000;
pub const ROM_STOP: u16     = 0x7FFF;
pub struct Cart {
    rom: Vec<u8>,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            rom: Vec::new(),
        }
    }

    pub fn load_cart(&mut self, rom: &[u8]) {
        self.rom = rom.to_vec();
    }

    pub fn read_cart(&self, addr: u16) -> u8 {
        // TODO: Handle bank switching
        self.rom[addr as usize]
    }

    pub fn write_cart(&mut self, addr: u16, val: u8) {
        // TODO: Handle bank switching
    }
}