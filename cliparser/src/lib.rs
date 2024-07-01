use std::env;

pub fn parse() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    return args
}
