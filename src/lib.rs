use derive_more::From;
use std::ops::Index;

pub trait FromMemory: Sized {
    fn parse(cell: u16) -> Option<Self>;
}

impl FromMemory for Value {
    #[inline]
    fn parse(cell: u16) -> Option<Value> {
        Some(i32::from(cell))
    }
}

impl FromMemory for Instruction {
    fn parse(cell: u16) -> Option<Instruction> {
        unimplemented!();
    }
}

pub struct Memory {}

impl Memory {
    pub fn retrieve<R: FromMemory>(&self, addr: usize) -> Option<R> {
        unimplemented!();
    }
}

pub struct Emulator<'m> {
    memory: &'m Memory,
}

impl Emulator<'_> {
    pub fn new(memory: &Memory) -> Emulator {
        Emulator { memory }
    }

    pub fn step(&mut self) -> bool {
        unimplemented!();
    }

    pub fn registers(&self) -> &Registers {
        unimplemented!();
    }

    pub fn set_register(&mut self, reg: Register, value: Value) {}
}

pub type Value = i32;

#[derive(Debug, Default)]
pub struct Registers {
    pub r0: Value,
    pub r1: Value,
    pub r2: Value,
    pub r3: Value,
    pub r4: Value,
    pub pc: Value,
}

impl Index<Register> for Registers {
    type Output = Value;

    fn index(&self, index: Register) -> &Value {
        match index {
            Register::R0 => &self.r0,
            Register::R1 => &self.r1,
            Register::R2 => &self.r2,
            Register::R3 => &self.r3,
            _ => unimplemented!(),
        }
    }
}

#[derive(From, Debug)]
pub enum RegisterOrValue {
    Register(Register),
    Value(Value),
}

#[derive(Debug)]
pub enum Instruction {
    ADD {
        dest: Register,
        lhs: Register,
        rhs: RegisterOrValue,
    },
    AND {
        dest: Register,
        lhs: Register,
        rhs: RegisterOrValue,
    },
    BR {
        offset: Value,
        n: bool,
        z: bool,
        p: bool,
    },
    JMP(Register),
    JSR(RegisterOrValue),
    LD {
        dest: Register,
        offset: Value,
    },
    LDI {
        dest: Register,
        offset: Value,
    },
    LDR {
        dest: Register,
        base: Register,
        offset: Value,
    },
    LEA {
        dest: Register,
        offset: Value,
    },
    NOT {
        dest: Register,
        source: Register,
    },
    RTI,
    ST {
        source: Register,
        offset: Value,
    },
    STI {
        source: Register,
        offset: Value,
    },
    STR {
        source: Register,
        base: Register,
        offset: Value,
    },
    TRAP(Value),
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

#[derive(Debug)]
pub enum Flag {
    POS = 1,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}
