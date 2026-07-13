use crate::cpu::*;
use crate::utils::*;

const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
//  0x00,   0x01, 0x02, 0x03,   0x04,   0x05,   0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,     0x0C,   0x0D, 0x0E, 0x0F
  nop_00,   todo, todo, inc_03, inc_04, dec_05, todo, todo, todo, todo, todo, dec_0b,   inc_0c, dec_0d, todo, todo, // 0x00
  todo,     todo, todo, inc_13, inc_14, dec_15, todo, todo, todo, todo, todo, dec_1b,   inc_1c, dec_1d, todo, todo, // 0x10
  todo,     todo, todo, inc_23, inc_24, dec_25, todo, todo, todo, todo, todo, dec_2b,   inc_2c, dec_2d, todo, todo, // 0x20
  todo,     todo, todo, inc_33, inc_34, dec_35, todo, todo, todo, todo, todo, dec_3b,   inc_3c, dec_3d, todo, todo, // 0x30
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0x40
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0x50
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0x60
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0x70
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0x80
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0x90
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0xA0
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0xB0
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0xC0
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0xD0
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0xE0
  todo,     todo, todo, todo,   todo,   todo,   todo, todo, todo, todo, todo, todo,     todo,   todo, todo, todo, // 0xF0
];

fn todo(cpu: &mut Cpu) -> u8 {
  todo!();
}

pub fn execute(cpu: &mut Cpu) -> u8 {
  let op_index = cpu.fetch();
  OPCODES[op_index as usize](cpu)
}
fn nop_00(_cpu: &mut Cpu) -> u8 {
    // Do nothing
    1
}
// INC BC
// ----
fn inc_03(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::BC);
    2
}
// INC DE
// ----
fn inc_13(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::DE);
    2
}
// INC HL
// ----
fn inc_23(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::HL);
    2
}
// INC SP
// ----
fn inc_33(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::SP);
    2
}
// INC B
// Z0H-
fn inc_04(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::B);
    1
}
// INC D
// Z0H-
fn inc_14(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::D);
    1
}
// INC H
// Z0H-
fn inc_24(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::H);
    1
}
// INC (HL)
// Z0H-
fn inc_34(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::HL);
    3
}
// INC C
// Z0H-
fn inc_0c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::C);
    1
}
// INC E
// Z0H-
fn inc_1c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::E);
    1
}
// INC L
// Z0H-
fn inc_2c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::L);
    1
}
// INC A
// Z0H-
fn inc_3c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::A);
    1
}
// DEC B
// Z1H-
fn dec_05(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::B);
    1
}
// DEC D
// Z1H-
fn dec_15(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::D);
    1
}
// DEC H
// Z1H-
fn dec_25(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::H);
    1
}
// DEC (HL)
// Z1H-
fn dec_35(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::HL);
    3
}
// DEC BC
// ----
fn dec_0b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::BC);
    2
}
// DEC DE
// ----
fn dec_1b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::DE);
    2
}
// DEC HL
// ----
fn dec_2b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::HL);
    2
}
// DEC SP
// ----
fn dec_3b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::SP);
    2
}
// DEC C
// Z1H-
fn dec_0d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::C);
    1
}
// DEC E
// Z1H-
fn dec_1d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::E);
    1
}
// DEC L
// Z1H-
fn dec_2d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::L);
    1
}
// DEC A
// Z1H-
fn dec_3d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::A);
    1
}