use std::ops::{Index, IndexMut};

use crate::Value;

#[derive(Debug, Default)]
pub struct Registers {
    pub r0: Value,
    pub r1: Value,
    pub r2: Value,
    pub r3: Value,
    pub r4: Value,
    pub r5: Value,
    pub r6: Value,
    pub r7: Value,
    pub pc: Value,
    pub cond: Value,
    pub count: Value,
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
    COUNT,
}

impl Index<Register> for Registers {
    type Output = Value;

    fn index(&self, index: Register) -> &Value {
        match index {
            Register::R0 => &self.r0,
            Register::R1 => &self.r1,
            Register::R2 => &self.r2,
            Register::R3 => &self.r3,
            Register::R4 => &self.r4,
            Register::R5 => &self.r5,
            Register::R6 => &self.r6,
            Register::R7 => &self.r7,
            Register::PC => &self.pc,
            Register::COND => &self.cond,
            Register::COUNT => &self.count,
        }
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Value {
        match index {
            Register::R0 => &mut self.r0,
            Register::R1 => &mut self.r1,
            Register::R2 => &mut self.r2,
            Register::R3 => &mut self.r3,
            Register::R4 => &mut self.r4,
            Register::R5 => &mut self.r5,
            Register::R6 => &mut self.r6,
            Register::R7 => &mut self.r7,
            Register::PC => &mut self.pc,
            Register::COND => &mut self.cond,
            Register::COUNT => &mut self.count,
        }
    }
}
