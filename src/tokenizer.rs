use std::iter::Peekable;
use std::str::Chars;

/// Represents different types of tokens in the C language.
#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    Operator(String),
    // Other token types can be added here as needed.
}

/// A tokenizer for the C programming language.
pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    /// Creates a new tokenizer for the given input string.
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input: input.chars().peekable(),
        }
    }

    /// Consumes and ignores any whitespace characters.
    fn consume_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    /// Returns the next token from the input, if available.
    pub fn next_token(&mut self) -> Option<Token> {
        self.consume_whitespace(); // First consume any leading whitespace

        let current_char = self.input.peek()?;

        match current_char {
            'a'..='z' | 'A'..='Z' | '_' => {
                let identifier = self.consume_while(|c| c.is_alphanumeric() || c == '_');
                if identifier == "if" || identifier == "int" {
                    Some(Token::Keyword(identifier))
                } else {
                    Some(Token::Identifier(identifier))
                }
            }
            '0'..='9' => {
                let number = self.consume_while(|c| c.is_digit(10));
                Some(Token::Number(number))
            }
            '=' => {
                self.input.next();
                Some(Token::Operator("=".to_string()))
            }
            ';' => {
                self.input.next();
                Some(Token::Operator(";".to_string()))
            }
            _ => None, // Unrecognized characters are ignored
        }
    }

    /// Consumes characters as long as they satisfy the given condition.
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(&ch) = self.input.peek() {
            if test(ch) {
                result.push(ch);
                self.input.next();
            } else {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the basic functionality of the Tokenizer.
    #[test]
    fn test_tokenizer() {
        let input = "int x = 42;";
        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next_token() {
            tokens.push(token);
        }

        assert_eq!(
            tokens,
            vec![
                Token::Keyword("int".to_string()),
                Token::Identifier("x".to_string()),
                Token::Operator("=".to_string()),
                Token::Number("42".to_string()),
                Token::Operator(";".to_string()),
            ]
        );
    }
}
