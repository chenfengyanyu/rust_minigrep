use std::env;
use std::fs;
use std::process;
use std::error::Error;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // 执行一些自定义的且不会产生 panic! 的错误处理策略
    // 当 Result 的值是 Ok 时，这个方法的行为与 unwarp 相同，会返回 Ok 中的值
    // 当值是 Err 时，这个方法则会调用闭包（closure）中编写的匿名函数
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        // 立刻终止程序运行并将我们指定的错误码返回给调用者
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数缺失！");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}

// Box<dyn Error> 意味着函数会返回一个实现了 Error trait 的类型，但我们并不需要指定具体类型是什么
// dyn -> dynamic 指动态类型
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; // ?运算符可以将错误值返回给函数的调用者来进行处理
    println!("text: \n{}", contents);
    Ok(())
}

// cargo run 命令：["target/debug/minigrep"]
// cargo run hello jartto 命令：["target/debug/minigrep", "hello", "jartto"]
