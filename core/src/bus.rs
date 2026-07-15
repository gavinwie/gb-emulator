use crate::cart::{Cart, ROM_START, ROM_STOP};
pub struct Bus {
    rom: Cart,
    ram: [u8; 0x8000], 
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom: Cart::new(),
            ram: [0; 0x8000],
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.rom.load_cart(data);
    }
    
    pub fn read_ram(&self, addr: u16) -> u8 {
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.read_cart(addr)
            },
            _ => {
                let offset = addr - ROM_STOP - 1;
                self.ram[offset as usize]
            }
        }
    }
    pub fn write_ram(&mut self, addr: u16, val: u8) {
        match addr {
            ROM_START..=ROM_STOP => {
                self.rom.write_cart(addr, val);
            },
            _ => {
                let offset = addr - ROM_STOP - 1;
                self.ram[offset as usize] = val;
            }
        }
    }
}