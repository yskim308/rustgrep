use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {err}");
        process::exit(1);
    });
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("too few arguments. Need exactly 2!");
        } else if args.len() > 3 {
            return Err("too many arguments. Need exactly 2!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
