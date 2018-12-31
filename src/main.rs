use lc3vm::{Processor, TrapHandlers};

struct TermEmulator {
    memory: Memory,
    processor: Processor,
}

impl TrapHandlers for TermEmulator {
    fn getc(&mut self) -> Value {
        unimplemented!();
    }

    fn out(&mut self, value: Value) {
        let ch = char::from(value);
        println!("{}", ch);
    }

    fn puts(&mut self, loc: Value) {
        unimplemented!();
    }

    fn in(&mut self, loc: Value) -> Value {
        unimplemented!();
    }

    fn putsp(&mut self, loc: Value) {
        unimplemented!();
    }

    fn halt(&mut self) {
        println!("LOLKEK");
    }
}

fn main() {
    let config = get_config();

    let contents = fs::read_to_string(config.file_path);
    println!("With text:\n{:?}", contents);
}

struct Config {
    file_path: String,
}

fn get_config() -> Config {
    let mut args = env::args().collect::<Vec<_>>();
    let file_path = args.swap_remove(1);

    Config { file_path }
}

