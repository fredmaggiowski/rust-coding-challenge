use words_count::{self, WordsCount};

pub enum CountType {
    Bytes,
    Lines,
    Words,
    Chars,
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

    fn wordcount_wrapper(&self) -> WordsCount {
        words_count::count(self.content.to_string())
    }

    pub fn words(&self) -> i64 {
        self.wordcount_wrapper().words.try_into().unwrap()
    }

    pub fn chars(&self) -> i64 {
        self.wordcount_wrapper().characters.try_into().unwrap()
    }
}

#[cfg(test)]
mod counters_test {
    macro_rules! test_cases_words {
        ( $($label:ident : $content:expr, $exp:expr);* $(;)? ) => {
            $(
                #[test]
                fn $label() {
                    assert_eq!(Counter::new($content.to_string()).words(), $exp);
                }
            )*
        }
    }
    
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
    
    test_cases_words!  {    
        speachless: "", 0;
        single_word: "hello", 1;
        excerpt_1: "52. We cannot enter into alliance with neighbouring princes until we are
acquainted with their designs. We are not fit to lead an army on the
march unless we are.", 30;
        excerpt_2: "
[These three sentences are repeated from VII. §§ 12-14—in order to
emphasize their importance, the commentators seem to think. 
", 19
    }

    #[test]
    fn t_chars () {
        let content: &str = "w1 w2";
        let result = Counter::new(content.to_string()).chars();
        assert_eq!(result, 5);
    }
}
