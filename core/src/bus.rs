use crate::cart::{Cart, ROM_START, ROM_STOP};
use crate::ppu::{Ppu, VRAM_START, VRAM_STOP, LCD_REG_START, LCD_REG_STOP};
use crate::ppu::PpuUpdateResult;

pub struct Bus {
    rom: Cart,
    ppu: Ppu,
    ram: [u8; 0x8000], 
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom: Cart::new(),
            ppu: Ppu::new(),
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
            VRAM_START..=VRAM_STOP => {
                self.ppu.read_vram(addr)
            },
            LCD_REG_START..=LCD_REG_STOP => {
                self.ppu.read_lcd_reg(addr)
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
            VRAM_START..=VRAM_STOP => {
                self.ppu.write_vram(addr, val);
            },
            LCD_REG_START..=LCD_REG_STOP => {
                self.ppu.write_lcd_reg(addr, val)
            },
            _ => {
                let offset = addr - ROM_STOP - 1;
                self.ram[offset as usize] = val;
            }
        }
    }

    pub fn update_ppu(&mut self, cycles: u8) -> PpuUpdateResult {
        return self.ppu.update(cycles)
    }
}