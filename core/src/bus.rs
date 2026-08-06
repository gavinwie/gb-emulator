use crate::cart::{Cart, ROM_START, ROM_STOP};
use crate::ppu::{Ppu, VRAM_START, VRAM_STOP, LCD_REG_START, LCD_REG_STOP, OAM_START, OAM_STOP};
use crate::io::{IO, Buttons, IO_START, IO_STOP};
use crate::ppu::PpuUpdateResult;
use crate::utils::DISPLAY_BUFFER;
use crate::wram::{WRAM, ECHO_STOP, WRAM_START};

const HRAM_START: u16   = 0xFF80;
const HRAM_STOP: u16    = 0xFFFF;
const HRAM_SIZE: usize  = (HRAM_STOP - HRAM_START + 1) as usize;


const OAM_DMA: u16 = 0xFF46;

pub struct Bus {
    rom: Cart,
    ppu: Ppu,
    io: IO,
    wram: WRAM,
    hram: [u8; HRAM_SIZE],
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom: Cart::new(),
            ppu: Ppu::new(),
            io: IO::new(),
            wram: WRAM::new(),
            hram: [0; HRAM_SIZE],
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
            WRAM_START..=ECHO_STOP => {
                self.wram.read_u8(addr)
            },
            OAM_START..=OAM_STOP => {
                self.ppu.read_oam(addr)
            },
            IO_START..=IO_STOP => {
                self.io.read_u8(addr)
            },
            LCD_REG_START..=LCD_REG_STOP => {
                self.ppu.read_lcd_reg(addr)
            },
            HRAM_START..=HRAM_STOP => {
                let relative_addr = addr - HRAM_START;
                self.hram[relative_addr as usize]
            },
            _ => {
                0
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
            WRAM_START..=ECHO_STOP => {
                self.wram.write_u8(addr, val)
            },
            OAM_START..=OAM_STOP => {
                self.ppu.write_oam(addr, val);
            },
            IO_START..=IO_STOP => {
                self.io.write_u8(addr, val);
            },
            LCD_REG_START..=LCD_REG_STOP => {
                if addr == OAM_DMA {
                    self.dma_transfer(val);
                }
                self.ppu.write_lcd_reg(addr, val)
            },
            HRAM_START..=HRAM_STOP => {
                let relative_addr = addr - HRAM_START;
                self.hram[relative_addr as usize] = val;
            },
            _ => {
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

    fn dma_transfer(&mut self, val: u8) {
        let src = (val as u16) << 8;
        for i in 0..0xA0 {
            let val = self.read_ram(src + i);
            self.write_ram(OAM_START + i, val);
        }
    }
    pub fn update_timer(&mut self, cycles: u8) -> bool {
        self.io.update_timer(cycles)
    }
    pub fn render_scanline(&mut self) {
        self.ppu.render_scanline();
    }
    pub fn get_title(&self) -> &str {
        self.rom.get_title()
    }
}