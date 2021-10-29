use std::env; // 当被引入的目标存在两层嵌套时，应当引入其父级模块
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    /// 基本类型偏执的反模式
    fn new(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            // 更倾向于使用 panic 来暴露程序的内部问题，而非 非法使用问题
            // panic!("not enough arguments");
            return Err("not enough arguments")
        }
        let query = args[1].clone(); // TODO：这里clone的目的是为了借用所有权
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

/// 二进制项目的关注点分离原则：程序拆分为main.rs 和 lib.rs,并将实际的业务逻辑放入lib.rs 中
/// 一定程度上保证main.rs 文件肉眼可见的检查，以及lib.rs文件的单元测试覆盖率
fn main() {
    let args: Vec<String> = env::args().collect();

    //    let query = args[0]; // TODO: 这会导致所有权转移？
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}",err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {}",e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let content = fs::read_to_string(config.filename).expect("Something went wrong"); // TODO: 这里的expect 起到什么作用？

    println!("content is: {:?}", content);

    Ok(())
}
