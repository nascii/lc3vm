pub struct Processor<E> {
    regs: Registers,
    env: E,
}

impl<E> Processor<H>
where
    E: Keyboard + Memory + Terminal,
{
    pub fn new(env: E) -> Processor {
        Processor {
            regs: Registers::default(),
            env,
        }
    }

    pub fn step(self, memory: &mut Memory) -> bool {
        let ip = self.load(Register::Pc);

        match memory.retrieve::<Instruction>() {
            Some(i) => self.execute(i, memory),
            None => unimplemented!(),
        }
    }

    fn execute(&mut self, instruction: Instruction, memory: &mut Memory) -> bool {
        use self::Instruction::*;

        match instruction {
            Add { target, lhs, rhs } => {
                let lhs = self.resolve(lhs);
                let rhs = self.resolve(rhs);
                self.save(target, lhs + rhs);
            } // ...
        }
    }

    fn resolve(arg: impl Into<RegisterOrValue>) -> Value {
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
