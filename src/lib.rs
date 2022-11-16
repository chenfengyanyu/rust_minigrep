use std::fs;
use std::error::Error;
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; // ?运算符可以将错误值返回给函数的调用者来进行处理
    println!("text: \n{}", contents);
    Ok(())
}