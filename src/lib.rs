pub struct Lexer {
    input: String,
    cursor: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    INSERT,
    SELECT,
    EXIT,
}

#[derive(Debug)]
pub enum Token {
    SpecialChar(char),
    Command(Command),
    Number(u64),
    AlphaNumeric(String),
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { input, cursor: 0 }
    }
    pub fn trim_leading_spaces(&mut self, chars: &Vec<char>) {
        while self.cursor < chars.len() && chars[self.cursor].is_whitespace() {
            self.cursor += 1;
        }
    }
    fn reslice_input(&mut self, end: usize) {
        if end < self.input.len() {
            self.input = self.input[end..].to_string();
            self.cursor = 0;
        }
    }
    fn extract_token_and_reslice(
        &mut self,
        chars: &Vec<char>,
        matcher: fn(char) -> bool,
    ) -> String {
        let start = self.cursor;
        let mut end = self.cursor;
        while end < chars.len() && matcher(chars[end]) {
            end += 1;
        }
        let token = self.input[start..end].to_string();
        self.reslice_input(end);
        return token;
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let chars = self.input.chars().collect::<Vec<_>>();
        self.trim_leading_spaces(&chars);
        if self.cursor >= chars.len() {
            return None;
        } else if chars[self.cursor].is_alphabetic() {
            let token = self.extract_token_and_reslice(&chars, |c| c.is_alphanumeric());
            let result = match token.to_uppercase().as_str() {
                "INSERT" => Token::Command(Command::INSERT),
                "SELECT" => Token::Command(Command::SELECT),
                "EXIT" => Token::Command(Command::EXIT),
                _ => Token::AlphaNumeric(token),
            };
            return Some(result);
        } else if chars[self.cursor].is_numeric() {
            let num_str = self.extract_token_and_reslice(&chars, |c| c.is_numeric());
            let num = num_str.parse::<u64>().unwrap_or_else(|err| {
                eprintln!("error parsing number {err:?}");
                return 0;
            });
            return Some(Token::Number(num));
        } else {
            let token = Token::SpecialChar(chars[self.cursor]);
            self.cursor += 1;
            self.reslice_input(self.cursor);
            return Some(token);
        }
    }
}
