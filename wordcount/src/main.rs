use file::read_file;

use cliparser;

mod counters;
mod file;

enum CountType {
    Bytes,
}


fn print(count: i64, path: &str) {
    println!("{} {}", count, path)
}

fn read_and_count(path: &str, count_type: CountType) -> i64 {
    let content = read_file(&path);
    match count_type {
        CountType::Bytes => counters::bytes(content.as_str()),
    }
}

fn main() {
    let args = cliparser::parse();

    let path = &args[2];
    dbg!(path);
    
    let count_result = read_and_count(path, CountType::Bytes);

    print(count_result, &path)
}

#[cfg(test)]
mod main_test {
    use crate::CountType;

    #[test]
    fn t_read_and_count() {
        let result = crate::read_and_count("./data/readfile-test.txt", CountType::Bytes);
        assert_eq!(result, 6);
    }


    #[test]
    fn t_read_and_count_original_dataset() {
        let result = crate::read_and_count("./data/test.txt", CountType::Bytes);
        // https://codingchallenges.fyi/challenges/challenge-wc#step-one
        // ➜  rust-coding-challenge git:(main) ✗ wc -c ./wordcount/data/test.txt 
        //    335043 ./wordcount/data/test.txt
        assert_eq!(result, 335043);
    }
}