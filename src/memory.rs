use crate::Value;

pub struct Memory(Vec<u16>);

pub trait FromMemory: Sized {
    fn parse(cell: u16) -> Option<Self>;
}

impl Memory {
    pub fn save(&mut self, loc: Value, value: Value) {
        let cell = value as u16;
        // TODO: avoid panic?
        self.0[loc as usize] = cell;
    }

    pub fn load<R: FromMemory>(&mut self, loc: Value) -> Option<R> {
        let cell = self.0.get(loc as usize)?;
        R::parse(*cell)
    }
}
