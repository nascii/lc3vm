use crate::{FromMemory, Value, Instruction};

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
