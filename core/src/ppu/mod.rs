pub const VRAM_START: u16   = 0x8000;
pub const VRAM_STOP: u16    = 0x9FFF;

const TILE_SET_START: u16   = 0x8000;
const TILE_SET_STOP: u16    = 0x97FF;
const TILE_MAP_START: u16   = 0x9800;
const TILE_MAP_STOP: u16    = 0x9FFF;

const BYTES_PER_TILE: u16 = 16;
const NUM_TILES: usize    = 384;

mod tile;

use tile::Tile;
pub struct Ppu {
    tiles: [Tile; NUM_TILES],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            tiles: [Tile::new(); NUM_TILES],
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
                todo!();
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
                todo!();
            },
            _ => { unreachable!() }
        }
    }
}