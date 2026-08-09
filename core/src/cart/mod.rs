use std::str::from_utf8;

pub const ROM_START: u16    = 0x0000;
pub const ROM_STOP: u16     = 0x7FFF;

const TITLE_START: usize = 0x0134;
const TITLE_STOP: usize  = 0x0142;

const CART_TYPE_ADDR: usize = 0x0147;

const RAM_SIZES: [usize; 6] = [
    0,
    2,
    8,
    32,
    128,
    64
];

const RAM_SIZE_ADDR: usize = 0x0149;
pub const EXT_RAM_START: u16    = 0xA000;
pub const EXT_RAM_STOP: u16     = 0xBFFF;


#[derive(Clone, Copy, PartialEq)]
pub enum MBC {
    NONE,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
    INV,
}

pub struct Cart {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mbc: MBC,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            rom: Vec::new(),
            ram: Vec::new(),
            mbc: MBC::NONE,
        }
    }

    pub fn get_title(&self) -> &str {
        let data = &self.rom[TITLE_START..TITLE_STOP];
        from_utf8(data).unwrap().trim_end_matches(char::from(0))
    }

    pub fn load_cart(&mut self, rom: &[u8]) {
        self.rom = rom.to_vec();
        self.mbc = self.get_mbc();
        self.init_ext_ram();
    }

    pub fn read_cart(&self, addr: u16) -> u8 {
        // TODO: Handle bank switching
        self.rom[addr as usize]
    }

    pub fn write_cart(&mut self, addr: u16, val: u8) {
        // TODO: Handle bank switching
    }

    fn get_mbc(&self) -> MBC {
        let cart_type = self.rom[CART_TYPE_ADDR];
        match cart_type {
            0x00 =>         { MBC::NONE },
            0x01..=0x03 =>  { MBC::MBC1 },
            0x05..=0x06 =>  { MBC::MBC2 },
            0x0F..=0x13 =>  { MBC::MBC3 },
            0x19..=0x1E =>  { MBC::MBC5 },
            _ =>            { MBC::INV },
        }
    }

    pub fn has_battery(&self) -> bool {
        let has_battery = [
            0x03, 0x06, 0x09,
            0x0D, 0x0F, 0x10,
            0x13, 0x1B, 0x1E,
        ];

        let cart_type = self.rom[CART_TYPE_ADDR];
        has_battery.contains(&cart_type)
    }

    fn has_external_ram(&self) -> bool {
        let has_ext_ram = [
            0x02, 0x03, 0x08,
            0x09, 0x0C, 0x0D,
            0x10, 0x12, 0x13,
            0x16, 0x17, 0x1A,
            0x1B, 0x1D, 0x1E,
        ];

        let cart_type = self.rom[CART_TYPE_ADDR];
        has_ext_ram.contains(&cart_type)
    }

    fn init_ext_ram(&mut self) {
        let mut ram_size_idx = self.rom[RAM_SIZE_ADDR] as usize;

        // Some headers don't report their external RAM capacity correctly
        if self.has_external_ram() && ram_size_idx == 0 {
            ram_size_idx = 1;
        }

        let ram_size = RAM_SIZES[ram_size_idx] * 1024;
        self.ram = vec![0; ram_size];
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        let rel_addr = addr - EXT_RAM_START;
        self.ram[rel_addr as usize]
    }

    pub fn write_ram(&mut self, addr: u16, val: u8) {
        let rel_addr = addr - EXT_RAM_START;
        self.ram[rel_addr as usize] = val;
    }

}

