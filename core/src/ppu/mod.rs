pub const VRAM_START: u16   = 0x8000;
pub const VRAM_STOP: u16    = 0x9FFF;

const TILE_SET_START: u16   = 0x8000;
const TILE_SET_STOP: u16    = 0x97FF;
const TILE_MAP_START: u16   = 0x9800;
const TILE_MAP_STOP: u16    = 0x9FFF;

const BYTES_PER_TILE: u16 = 16;
const NUM_TILES: usize    = 384;

const TILE_MAP_SIZE: usize = (TILE_MAP_STOP - TILE_MAP_START + 1) as usize;

mod tile;

use tile::Tile;
pub struct Ppu {
    tiles: [Tile; NUM_TILES],
    maps: [u8; TILE_MAP_SIZE],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            tiles: [Tile::new(); NUM_TILES],
            maps: [0; TILE_MAP_SIZE]
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
}