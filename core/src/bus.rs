use crate::cart::{Cart, ROM_START, ROM_STOP};
use crate::ppu::{Ppu, VRAM_START, VRAM_STOP, LCD_REG_START, LCD_REG_STOP};
use crate::io::{IO, Buttons, IO_START, IO_STOP};
use crate::ppu::PpuUpdateResult;
use crate::utils::DISPLAY_BUFFER;

pub struct Bus {
    rom: Cart,
    ppu: Ppu,
    io: IO,
    ram: [u8; 0x8000], 
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom: Cart::new(),
            ppu: Ppu::new(),
            io: IO::new(),
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
            // OAM_START..=OAM_STOP => {
            //     self.ppu.read_oam(addr)
            // },
            IO_START..=IO_STOP => {
                self.io.read_u8(addr)
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
            // OAM_START..=OAM_STOP => {
            //     self.ppu.write_oam(addr, val);
            // },
            IO_START..=IO_STOP => {
                self.io.write_u8(addr, val);
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

    pub fn render(&self) -> [u8; DISPLAY_BUFFER] {
        self.ppu.render()
    }
    pub fn press_button(&mut self, button: Buttons, pressed: bool) {
        self.io.set_button(button, pressed);
    }
}