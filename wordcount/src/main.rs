use counters::CountType;
use file::read_file;

use cliparser;

mod counters;
mod file;

fn print(count: i64, path: &str) {
    println!("{} {}", count, path)
}

fn read_and_count(path: &str, count_type: counters::CountType) -> i64 {
    let content = read_file(&path);
    match count_type {
        counters::CountType::Bytes => counters::bytes(content.as_str()),
        counters::CountType::Lines => counters::lines(content.as_str()),
    }
}

fn map_count_flag_to_counttype(arg: &str) -> CountType {
    match arg {
        "-c" => counters::CountType::Bytes,
        "-l" => counters::CountType::Lines,
        _ => panic!("invalid flag, expected -c or -l"),
    }
}

fn main() {
    let args = cliparser::parse();
    
    let count_type_arg = &args[1];
    let path = &args[2];

    let count_type = map_count_flag_to_counttype(count_type_arg);
    let count_result = read_and_count(path, count_type);

    print(count_result, &path)
}

#[cfg(test)]
mod main_test {
    use crate::counters::CountType;

    #[test]
    fn t_read_and_count() {
        let result = crate::read_and_count("./data/readfile-test.txt", CountType::Bytes);
        assert_eq!(result, 6);
    }


    #[test]
    fn t_read_and_count_original_dataset_bytes() {
        let result = crate::read_and_count("./data/test.txt", CountType::Bytes);
        // https://codingchallenges.fyi/challenges/challenge-wc#step-one
        // ➜  rust-coding-challenge git:(main) ✗ wc -c ./wordcount/data/test.txt 
        //    335042 ./wordcount/data/test.txt
        assert_eq!(result, 335042);
    }

    #[test]
    fn t_read_and_count_original_dataset_lines() {
        let result = crate::read_and_count("./data/test.txt", CountType::Lines);
        // https://codingchallenges.fyi/challenges/challenge-wc#step-two
        // ➜  rust-coding-challenge git:(main) wc -l ./wordcount/data/test.txt
        //    7146 ./wordcount/data/test.txt
        assert_eq!(result, 7146);
    }
}