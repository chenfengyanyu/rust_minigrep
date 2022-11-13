use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // let query = &args[1];
    // let filename = &args[2];
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("文件读取异常");
    println!("text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

// cargo run 命令：["target/debug/minigrep"]
// cargo run hello jartto 命令：["target/debug/minigrep", "hello", "jartto"]
