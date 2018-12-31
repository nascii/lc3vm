use crate::{Environment, Instruction, Register, RegisterOrValue, Registers, Value};

pub struct Processor<E> {
    regs: Registers,
    env: E,
}

impl<E: Environment> Processor<E> {
    pub fn new(env: E) -> Processor<E> {
        Processor {
            regs: Registers::default(),
            env,
        }
    }

    pub fn registers(&self) -> &Registers {
        &self.regs
    }

    pub fn set_register(&mut self, reg: Register, value: Value) {
        self.save(reg, value);
    }

    pub fn step(&mut self) -> bool {
        let ip = self.load(Register::PC);

        match self.env.read_mem::<Instruction>(ip) {
            Some(i) => self.execute(i),
            None => unimplemented!(),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> bool {
        use self::Instruction::*;

        match instruction {
            ADD { dest, lhs, rhs } => {
                let lhs = self.resolve(lhs);
                let rhs = self.resolve(rhs);
                self.save(dest, lhs + rhs);
                true
            } // ...
            _ => unimplemented!(),
        }
    }

    fn resolve(&mut self, arg: impl Into<RegisterOrValue>) -> Value {
        match arg.into() {
            RegisterOrValue::Value(val) => val,
            RegisterOrValue::Register(reg) => self.load(reg),
        }
    }

    fn save(&mut self, reg: Register, val: Value) {
        self.regs[reg] = val;
    }

    fn load(&mut self, reg: Register) -> Value {
        self.regs[reg]
    }
}
