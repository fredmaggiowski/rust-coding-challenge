use file::read_file;

use cliparser;

mod counters;
mod file;

fn print(count: i64, path: &str) {
    println!("{} {}", count, path)
}

fn main() {
    let args = cliparser::parse();

    let path = &args[2];
    dbg!(path);
    
    let content = read_file(&path);
    let bytes = counters::bytes(content.as_str());
    print(bytes, &path)
}

#[cfg(test)]
mod main_test;