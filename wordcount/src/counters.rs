use regex::Regex;

pub enum CountType {
    Bytes,
    Lines,
    Words,
}

pub struct Counter {
    content: String
}

impl Counter {
    pub fn new(content: String) -> Counter {
        Counter{ content: content }
    }

    pub fn bytes(&self) -> i64 {
        self.content.len().try_into().unwrap()
    }
    
    pub fn lines(&self) -> i64 {
        let split_c:Vec<&str> = self.content.lines().collect();
        split_c.len().try_into().unwrap()
    }
    pub fn words(&self) -> i64 {
        let separator = Regex::new(r"\s").expect("invalid regex");
        let splitted: Vec<&str> = separator.split(self.content.as_str()).collect();
        splitted.len().try_into().unwrap()
    }    
}

#[cfg(test)]
mod counters_test {
    use crate::counters::Counter;

    #[test]
    fn t_bytes () {
        let content = "my-content";
        let result = Counter::new(content.to_string()).bytes();
        assert_eq!(result, 10);
    }

    #[test]
    fn t_lines() {
        let content = "my-content";
        let result = Counter::new(content.to_string()).lines();
        assert_eq!(result, 1);
    }

    #[test]
    fn t_lines_multi() {
        let content = "my-content
on two lines";
        let result = Counter::new(content.to_string()).lines();
        assert_eq!(result, 2);
    }

    #[test]
    fn t_words() {
        let content: &str = "w1 w2 w-3";
        let result = Counter::new(content.to_string()).words();
        assert_eq!(result, 3);
    }

    #[test]
    fn t_words_multiline() {
        let content: &str = "w1 w2
w-3";
        let result = Counter::new(content.to_string()).words();
        assert_eq!(result, 3);
    }
}