use crate::{Register, RegisterOrValue, Value};

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
