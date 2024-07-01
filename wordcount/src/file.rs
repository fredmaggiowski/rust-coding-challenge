use std::fs;

pub fn read_file(path: &str) -> String {
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    return contents
}

#[test]
fn t_read_file () {
    let expected = "hello!";
    let result = read_file("./data/readfile-test.txt");
    assert_eq!(result, expected);
}
