pub fn bytes(content: &str) -> i64 {
    content.len().try_into().unwrap()
}

#[test]
fn t_bytes () {
    let content = "my-content";
    let result = bytes(content);
    assert_eq!(result, 10);
}
