pub fn merge_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}
pub trait ByteOps {
    fn high_byte(&self) -> u8;
    fn low_byte(&self) -> u8;
}
impl ByteOps for u16 {
    fn high_byte(&self) -> u8 {
        (self >> 8) as u8
    }

    fn low_byte(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

pub fn check_c_carry_u8(lhs: u8, rhs: u8) -> bool {
    lhs.checked_add(rhs).is_none()
}

pub fn check_c_carry_u16(lhs: u16, rhs: u16) -> bool {
    lhs.checked_add(rhs).is_none()
}

pub fn check_c_borrow_u8(lhs: u8, rhs: u8) -> bool {
    lhs.checked_sub(rhs).is_none()
}

pub fn check_c_borrow_u16(lhs: u16, rhs: u16) -> bool {
    lhs.checked_sub(rhs).is_none()
}

pub fn check_h_carry_u8(lhs: u8, rhs: u8) -> bool {
    ((lhs & 0xF) + (rhs & 0xF)) & 0xF0 != 0
}
pub fn check_h_carry_u16(lhs: u16, rhs: u16) -> bool {
    ((lhs & 0xFFF) + (rhs & 0xFFF)) & 0xF000 != 0
}
pub fn check_h_borrow_u8(lhs: u8, rhs: u8) -> bool {
    (lhs & 0xF).checked_sub(rhs & 0xF).is_none()
}

pub fn check_h_borrow_u16(lhs: u16, rhs: u16) -> bool {
    (lhs & 0xFFF).checked_sub(rhs & 0xFFF).is_none()
}