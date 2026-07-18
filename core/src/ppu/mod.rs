use crate::utils::{self, DISPLAY_BUFFER, GB_PALETTE, Point, SCREEN_HEIGHT, SCREEN_WIDTH, unpack_u8};
use crate::utils::BitOps;

pub const VRAM_START: u16   = 0x8000;
pub const VRAM_STOP: u16    = 0x9FFF;

const TILE_SET_START: u16   = 0x8000;
const TILE_SET_STOP: u16    = 0x97FF;
const TILE_MAP_START: u16   = 0x9800;
const TILE_MAP_STOP: u16    = 0x9FFF;

const BYTES_PER_TILE: u16 = 16;
const NUM_TILES: usize    = 384;

const NUM_TILE_COLS: usize = SCREEN_WIDTH / 8;
const NUM_TILE_ROWS: usize = SCREEN_HEIGHT / 8;
const LAYER_WIDTH: usize   = 32;

const TILE_MAP_SIZE: usize = (TILE_MAP_STOP - TILE_MAP_START + 1) as usize;
const TILE_MAP_TABLE_SIZE: usize = TILE_MAP_SIZE / 2;

pub const LCD_REG_START: u16    = 0xFF40;
pub const LCD_REG_STOP: u16     = 0xFF4B;
const LCD_REG_SIZE: usize = (LCD_REG_STOP - LCD_REG_START + 1) as usize;

const LCDC: u16                     = 0xFF40;
const STAT: u16                     = 0xFF41;
const SCY: u16                      = 0xFF42;
const SCX: u16                      = 0xFF43;
const LY: u16                       = 0xFF44;
const LYC: u16                      = 0xFF45;
const BGP: u16                      = 0xFF47;
const OBP0: u16                     = 0xFF48;
const OBP1: u16                     = 0xFF49;
const WY: u16                       = 0xFF4A;
const WX: u16                       = 0xFF4B;

const LCDC_LCD_ENABLED_BIT: u8      = 7;
const LCDC_WNDW_MAP_BIT: u8         = 6;
const LCDC_WNDW_ENABLED_BIT: u8     = 5;
const LCDC_BG_WNDW_TILE_BIT: u8     = 4;
const LCDC_BG_MAP_BIT: u8           = 3;
const LCDC_SPR_SIZE_BIT: u8         = 2;
const LCDC_SPR_ENABLED_BIT: u8      = 1;
const LCDC_BG_WNDW_ENABLED_BIT: u8  = 0;

const STAT_LY_LYC_IRQ_BIT: u8       = 6;
const STAT_OAM_IRQ_BIT: u8          = 5;
const STAT_VBLANK_IRQ_BIT: u8       = 4;
const STAT_HBLANK_IRQ_BIT: u8       = 3;
const STAT_LY_EQ_LYC_BIT: u8        = 2;



pub mod modes;
mod tile;

use tile::Tile;
use modes::{Lcd, LcdModeType, LcdResults};

pub struct PpuUpdateResult {
    pub lcd_result: LcdResults,
    pub irq: bool,
}
pub struct Ppu {
    mode: Lcd,
    tiles: [Tile; NUM_TILES],
    maps: [u8; TILE_MAP_SIZE],
    lcd_regs: [u8; LCD_REG_SIZE],
}
impl Ppu {
    pub fn new() -> Self {
        Self {
            mode: Lcd::new(),
            tiles: [Tile::new(); NUM_TILES],
            maps: [0; TILE_MAP_SIZE],
            lcd_regs: [0; LCD_REG_SIZE],
        }
    }

    pub fn update(&mut self, cycles: u8) -> PpuUpdateResult {
        let old_mode = self.mode.get_mode();
        let old_line = self.mode.get_line();
        let lcd_result = self.mode.step(cycles);
        let mut stat = self.read_lcd_reg(STAT);
        let mut irq = false;

        let scanline = self.mode.get_line();
        if old_line != scanline {
            let lyc = self.read_lcd_reg(LYC);
            stat.set_bit(STAT_LY_EQ_LYC_BIT, scanline == lyc);
            irq = (scanline == lyc) && stat.get_bit(STAT_LY_LYC_IRQ_BIT);
            self.write_lcd_reg(LY, scanline);
        }

        let mode = self.mode.get_mode();
        if old_mode != mode {
            match mode {
                LcdModeType::HBLANK => {
                    irq |= stat.get_bit(STAT_HBLANK_IRQ_BIT);
                },
                LcdModeType::VBLANK => {
                    irq |= stat.get_bit(STAT_VBLANK_IRQ_BIT);
                },
                LcdModeType::OAMReadMode => {
                    irq |= stat.get_bit(STAT_OAM_IRQ_BIT);
                }
                _ => {},
            }
        }
        stat &= 0b1111_1100;
        stat |= mode.get_idx();
        self.write_lcd_reg(STAT, stat);

        PpuUpdateResult{ lcd_result, irq }
    }
    pub fn render(&self) -> [u8; DISPLAY_BUFFER] {
        let mut result = [0xFF; DISPLAY_BUFFER];

        if self.is_bg_layer_displayed() {
            self.render_bg(&mut result);
        }

        return result;
    }
    fn render_bg(&self, buffer: &mut [u8]) {
        let map_offset = self.get_bg_tile_map_index() as usize * TILE_MAP_TABLE_SIZE;
        let palette = self.get_bg_palette();
        // Iterate over each screen row and column
        for ty in 0..NUM_TILE_ROWS {
            for tx in 0..NUM_TILE_COLS {
                // Get the appropriate pixel data for that spot
                let map_num = ty * LAYER_WIDTH + tx;
                let tile_index = self.maps[map_offset + map_num] as usize;
                // Calculate the correct tile index if needed
                let adjusted_tile_index = if self.get_bg_wndw_tile_set_index() == 1 {
                    tile_index as usize
                } else {
                    (256 + tile_index as i8 as isize) as usize
                };
                let tile = self.tiles[adjusted_tile_index];
                // Iterate over each pixel
                for y in 0..8 {
                    let row = tile.get_row(y);
                    let pixel_y = 8 * ty + y as usize;
                    for x in 0..8 {
                        // Use the palette table to get the right RGBA value
                        let pixel_x = 8 * tx + x;
                        let cell = row[x];
                        let color_idx = palette[cell as usize];
                        let color = GB_PALETTE[color_idx as usize];
                        // Copy the RGBA channels into the right spot in the buffer
                        let buffer_idx = 4 * (pixel_y * SCREEN_WIDTH + pixel_x);
                        for i in 0..4 {
                            buffer[buffer_idx + i] = color[i];
                        }
                    }
                }
            }
        }
    }
    pub fn read_vram(&self, addr: u16) -> u8 {
        match addr {
            TILE_SET_START..=TILE_SET_STOP => {
                let relative_addr = addr - TILE_SET_START;
                let tile_idx = relative_addr / BYTES_PER_TILE;
                let offset = relative_addr % BYTES_PER_TILE;
                self.tiles[tile_idx as usize].read_u8(offset)
            },
            TILE_MAP_START..=TILE_MAP_STOP => {
                let relative_addr = addr - TILE_MAP_START;
                self.maps[relative_addr as usize]
            },
            _ => { unreachable!() }
        }
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        match addr {
            TILE_SET_START..=TILE_SET_STOP => {
                let relative_addr = addr - TILE_SET_START;
                let tile_idx = relative_addr / BYTES_PER_TILE;
                let offset = relative_addr % BYTES_PER_TILE;
                self.tiles[tile_idx as usize].write_u8(offset, val);
            },
            TILE_MAP_START..=TILE_MAP_STOP => {
                let relative_addr = addr - TILE_MAP_START;
                self.maps[relative_addr as usize] = val;
            },
            _ => { unreachable!() }
        }
    }

    pub fn read_lcd_reg(&self, addr: u16) -> u8 {
        let relative_addr = addr - LCD_REG_START;
        self.lcd_regs[relative_addr as usize]
    }


    pub fn write_lcd_reg(&mut self, addr: u16, val: u8) {
        let relative_addr = addr - LCD_REG_START;
        self.lcd_regs[relative_addr as usize] = val;
    }

    fn is_lcd_enabled(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_LCD_ENABLED_BIT)
    }
    fn get_wndw_tile_map_index(&self) -> u8 {
        let lcdc = self.read_lcd_reg(LCDC);
        if lcdc.get_bit(LCDC_WNDW_MAP_BIT) { 1 } else { 0 }
    }
    fn is_window_layer_displayed(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_WNDW_ENABLED_BIT) && lcdc.get_bit(LCDC_WNDW_ENABLED_BIT)
    }
    fn get_bg_wndw_tile_set_index(&self) -> u8 {
        let lcdc = self.read_lcd_reg(LCDC);
        if lcdc.get_bit(LCDC_BG_WNDW_TILE_BIT) { 1 } else { 0 }
    }
    fn get_bg_tile_map_index(&self) -> u8 {
        let lcdc = self.read_lcd_reg(LCDC);
        if lcdc.get_bit(LCDC_BG_MAP_BIT) { 1 } else { 0 }
    }
    fn are_sprites_8x16(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_SPR_SIZE_BIT)
    }
    fn is_sprite_layer_displayed(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_SPR_ENABLED_BIT)
    }
    fn is_bg_layer_displayed(&self) -> bool {
        let lcdc = self.read_lcd_reg(LCDC);
        lcdc.get_bit(LCDC_BG_WNDW_ENABLED_BIT)
    }

    fn get_viewport_coords(&self) -> Point {
        let x = self.read_lcd_reg(SCX);
        let y = self.read_lcd_reg(SCY);
        Point::new(x, y)
    }
    fn get_window_coords(&self) -> Point {
        let x = self.read_lcd_reg(WX);
        let y = self.read_lcd_reg(WY);
        Point::new(x.saturating_sub(7), y)
    }
    fn get_bg_palette(&self) -> [u8; 4] {
        unpack_u8(self.read_lcd_reg(BGP))
    }
    fn get_sprite_palette(&self, index: u8) -> [u8; 4] {
        match index {
            0 => { unpack_u8(self.read_lcd_reg(OBP0)) },
            1 => { unpack_u8(self.read_lcd_reg(OBP1)) },
            _ => { unreachable!() }
        }
    }
}