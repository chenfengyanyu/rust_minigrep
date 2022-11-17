use std::env;
use std::process;

// 导入自定义 lib 内容
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // 执行一些自定义的且不会产生 panic! 的错误处理策略
    // 当 Result 的值是 Ok 时，这个方法的行为与 unwarp 相同，会返回 Ok 中的值
    // 当值是 Err 时，这个方法则会调用闭包（closure）中编写的匿名函数
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        // 立刻终止程序运行并将我们指定的错误码返回给调用者
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}



// cargo run 命令：["target/debug/minigrep"]
// cargo run hello jartto 命令：["target/debug/minigrep", "hello", "jartto"]
// cargo run to poem.txt > output.txt
