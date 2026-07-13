pub mod opcodes;
use crate::utils::*;
pub struct Cpu {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0x0000,
            sp: 0x0000,
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
        }
    }

    pub fn get_r8(&self, r: Regs8) -> u8 {
        match r {
            Regs8::A => { self.a },
            Regs8::B => { self.b },
            Regs8::C => { self.c },
            Regs8::D => { self.d },
            Regs8::E => { self.e },
            Regs8::F => { self.f },
            Regs8::H => { self.h },
            Regs8::L => { self.l },
            Regs8::HL => {
                let addr = self.get_r16(Regs16::HL);
                self.read_ram(addr)
            }
        }
    }
    pub fn set_r8(&mut self, r: Regs8, val: u8) {
        match r {
            Regs8::A => { self.a = val },
            Regs8::B => { self.b = val },
            Regs8::C => { self.c = val },
            Regs8::D => { self.d = val },
            Regs8::E => { self.e = val },
            // Note: The bottom four bits of F will always be 0
            Regs8::F => { self.f = val & 0xF0 },
            Regs8::H => { self.h = val },
            Regs8::L => { self.l = val },
            Regs8::HL => {
                let addr = self.get_r16(Regs16::HL);
                self.write_ram(addr, val);
            }
        }
    }
    pub fn dec_r8(&mut self, r: Regs8) {
        let val = self.get_r8(r);
        let dec = val.wrapping_sub(1);
        let set_h = check_h_borrow_u8(val, 1);

        self.set_r8(r, dec);
        self.set_flag(Flags::N, true);
        self.set_flag(Flags::Z, dec == 0);
        self.set_flag(Flags::H, set_h);
    }
    pub fn inc_r8(&mut self, r: Regs8) {
        let val = self.get_r8(r);
        let inc = val.wrapping_add(1);
        let set_h = check_h_carry_u8(val, 1);

        self.set_r8(r, inc);
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::Z, inc == 0);
        self.set_flag(Flags::H, set_h);
    }

    pub fn get_r16(&self, r: Regs16) -> u16 {
        match r {
            Regs16::AF => { merge_bytes(self.a, self.f) },
            Regs16::BC => { merge_bytes(self.b, self.c) },
            Regs16::DE => { merge_bytes(self.d, self.e) },
            Regs16::HL => { merge_bytes(self.h, self.l) },
            Regs16::SP => { self.sp },
        }
    }
    pub fn set_r16(&mut self, r: Regs16, val: u16) {
        let high = val.high_byte();
        let low = val.low_byte();
        match r {
            Regs16::AF => {
                self.set_r8(Regs8::A, high);
                self.set_r8(Regs8::F, low);
            },
            Regs16::BC => {
                self.set_r8(Regs8::B, high);
                self.set_r8(Regs8::C, low);
            },
            Regs16::DE => {
                self.set_r8(Regs8::D, high);
                self.set_r8(Regs8::E, low);
            },
            Regs16::HL => {
                self.set_r8(Regs8::H, high);
                self.set_r8(Regs8::L, low);
            },
            Regs16::SP => { self.sp = val; },
        }
    }
    pub fn dec_r16(&mut self, r: Regs16) {
        let val = self.get_r16(r);
        let dec = val.wrapping_sub(1);
        self.set_r16(r, dec);
    }
    pub fn inc_r16(&mut self, r: Regs16) {
        let val = self.get_r16(r);
        let inc = val.wrapping_add(1);
        self.set_r16(r, inc);
    }

    pub fn get_flag(&self, f: Flags) -> bool {
        match f {
            Flags::Z => { (self.f & 0b1000_0000) != 0 },
            Flags::N => { (self.f & 0b0100_0000) != 0 },
            Flags::H => { (self.f & 0b0010_0000) != 0 },
            Flags::C => { (self.f & 0b0001_0000) != 0 },
        }
    }
    pub fn set_flag(&mut self, f: Flags, val: bool) {
        if val {
            match f {
                Flags::Z => { self.f |= 0b1000_0000 },
                Flags::N => { self.f |= 0b0100_0000 },
                Flags::H => { self.f |= 0b0010_0000 },
                Flags::C => { self.f |= 0b0001_0000 },
            }
        } else {
            match f {
                Flags::Z => { self.f &= 0b0111_0000 },
                Flags::N => { self.f &= 0b1011_0000 },
                Flags::H => { self.f &= 0b1101_0000 },
                Flags::C => { self.f &= 0b1110_0000 },
            }
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let val = self.read_ram(self.pc);
        self.pc += 1;
        val
    }
    pub fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch();
        let high = self.fetch();
        let val = merge_bytes(high, low);
        val
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        todo!();
    }
    pub fn write_ram(&mut self, addr: u16, val: u8) {
        todo!();
    }
}

#[derive(Copy, Clone)]
pub enum Regs8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    HL,
}

#[derive(Copy, Clone)]
pub enum Regs16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

pub enum Flags {
    Z,
    N,
    H,
    C,
}