struct Lexer {
    state : QuoteStatus,
    escaping : bool,
}

trait LexerState {
    fn process_char(&mut self, character : char) -> Option<char>;
}

impl LexerState for Lexer {
    fn process_char(&mut self, character : char) -> Option<char> {
        match self.state {
            QuoteStatus::None => process_char_not_quoting(self, character),
            QuoteStatus::Weak => process_char_weak_quote(self, character),
            QuoteStatus::Strong => process_char_strong_quote(self, character),
            _ => process_char_strong_quote(self, character),
        }
    }
}

fn process_char_not_quoting(lexer : &mut Lexer, character : char) -> Option<char> {
    if lexer.escaping {
        lexer.escaping = false;
        return Some(character);
    }
    let whitespace = vec![' ', '\n', '\t'];
    if whitespace.contains(&character) {
        return None;
    }
    if character == '\\' {
        lexer.escaping = true;
        return None;
    }
    if character == '\'' {
        lexer.state = QuoteStatus::Strong;
        return None;
    }
    if character == '"' {
        lexer.state = QuoteStatus::Weak;
        return None;
    }
    Some(character)
}

fn process_char_strong_quote(lexer : &mut Lexer, character : char) -> Option<char> {
    if character == '\'' {
        lexer.state = QuoteStatus::None;
        None
    }
    else {
        Some(character)
    }
}

fn process_char_weak_quote(lexer : &mut Lexer, character : char) -> Option<char> {
    if lexer.escaping {
        lexer.escaping = false;
        return Some(character);
    }
    if character == '\\' {
        lexer.escaping = true;
        return None;
    }
    if character == '"' {
        lexer.state = QuoteStatus::None;
        return None;
    }
    Some(character)
}

enum QuoteStatus {
    None,
    Weak,
    Strong,
    HereDoc,
}

/// lex expects to receive a newline terminated string.
pub fn lex(input : &str) -> Vec<String> {
    let mut lexer = Lexer { escaping: false, state: QuoteStatus::None };
    let mut result : Vec<String> = vec![];
    let whitespace = vec![' ', '\n', '\t'];
    let mut current_lexeme = String::new();
    let special = vec!['`', '$', '\\'];  // Part of terrible escape hack
    for character in input.chars() {

        // TODO(Skyler) Fix terrible hack to make escaping work
        match lexer.state {
            QuoteStatus::Weak => {
                if lexer.escaping && !special.contains(&character) {
                    current_lexeme.push('\\');
                }
            },
            _ => (),
        }

        match lexer.process_char(character) {
            Some(chr) => current_lexeme.push(chr),
            None => (),
        }

        if whitespace.contains(&character) {
            match lexer.state {
                QuoteStatus::Weak => {
                }
                QuoteStatus::Strong => (),
                _ => {
                    if !current_lexeme.is_empty() {
                        result.push(current_lexeme);
                        current_lexeme = String::new();
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn lex_empty_string() {
        let expected : Vec<&str> = vec![];
        assert_eq!(expected, lex(""));
    }

    #[test]
    fn lex_newline_only() {
        let expected : Vec<&str> = vec![];
        assert_eq!(expected, lex("\n"));
    }

    #[test]
    fn lex_single_word() {
        let expected = vec!["cd"];
        assert_eq!(expected, lex("cd\n"));
    }

    #[test]
    fn lex_two_words() {
        let expected = vec!["ls", "-al"];
        assert_eq!(expected, lex("ls -al\n"));
    }

    #[test]
    fn lex_weak_quoting() {
        let expected = vec!["cd", "dir with spaces"];
        let actual = lex("cd \"dir with spaces\"\n");
        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_strong_quoting() {
        let expected = vec!["cd", " ${PWD}\\\\`echo`"];
        let actual = lex("cd ' ${PWD}\\\\`echo`'\n");
        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_unquoted_escape() {
        let expected = vec!["\\${PWD}()`\"'<<rn"];
        let actual = lex("\\\\\\${PWD}\\(\\)\\`\\\"\\'\\<\\<\\r\\n\n");
        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_weak_quote_escape() {
        let expected = vec!["\\t$`\\o"];
        let actual = lex("\"\\\\t\\$\\`\\o\"\n");
        assert_eq!(expected, actual);
    }

}
