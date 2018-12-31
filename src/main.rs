use lc3vm::{Environment, FromMemory, Memory, Processor};

struct Env {
    memory: Memory,
    processor: Processor,
}

impl Environment for TermEmulator {
    fn get_char(&mut self) -> Value {
        unimplemented!();
    }

    fn put_char(&mut self, value: Value) {
        let ch = char::from(value);
        println!("{}", ch);
    }

    fn read_mem<R: FromMemory>(&mut self, loc: Value) -> R {
        self.memory.load(loc)
    }

    fn write_mem(&mut self, loc: Value, value: Value) {
        self.memory.save(loc, value)
    }
}

struct Config {
    file_path: String,
}

fn get_config() -> Config {
    let mut args = env::args().collect::<Vec<_>>();
    let file_path = args.swap_remove(1);

    Config { file_path }
}

fn main() {
    let config = get_config();

    let contents = fs::read_to_string(config.file_path);
    println!("With text:\n{:?}", contents);
}
