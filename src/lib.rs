use derive_more::From;

pub use crate::{
    instructions::Instruction,
    memory::{FromMemory, Memory},
    processor::Processor,
    registers::{Register, Registers},
};

mod instructions;
mod memory;
mod parser;
mod processor;
mod registers;

pub trait Environment {
    fn get_char(&mut self) -> Value;
    fn put_char(&mut self, value: Value);
    fn read_mem<R: FromMemory>(&mut self, loc: Value) -> Option<R>;
    fn write_mem(&mut self, loc: Value, value: Value);
}

pub type Value = i32;

#[derive(From, Debug)]
pub enum RegisterOrValue {
    Register(Register),
    Value(Value),
}

#[derive(Debug)]
pub enum Flag {
    POS = 1,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}
