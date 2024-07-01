pub enum CountType {
    Bytes,
    Lines,
}

pub fn bytes(content: &str) -> i64 {
    content.len().try_into().unwrap()
}

pub fn lines(content: &str) -> i64 {
    let split_c:Vec<&str> = content.split("\n").collect();
    split_c.len().try_into().unwrap()
}

#[cfg(test)]
mod counters_test {
    use crate::counters;

    #[test]
    fn t_bytes () {
        let content = "my-content";
        let result = counters::bytes(content);
        assert_eq!(result, 10);
    }

    #[test]
    fn t_lines() {
        let content = "my-content";
        let result = counters::lines(content);
        assert_eq!(result, 1);
    }

    #[test]
    fn t_lines_multi() {
        let content = "my-content
on two lines";
        let result = counters::lines(content);
        assert_eq!(result, 2);
    }
}