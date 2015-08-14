struct Lexer {
    state : LexerState,
    escaping : bool,
}

impl Lexer {
    fn process_char(&mut self, character : char) -> Option<char> {
        match self.state {
            LexerState::WeakQuoting => process_char_weak_quote(self, character),
            LexerState::StrongQuoting => process_char_strong_quote(self, character),
            LexerState::Comment => process_char_comment(self, character),
            _ => process_char_normal(self, character),
        }
    }
}

fn process_char_normal(lexer : &mut Lexer, character : char) -> Option<char> {
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
        lexer.state = LexerState::StrongQuoting;
        return None;
    }
    if character == '"' {
        lexer.state = LexerState::WeakQuoting;
        return None;
    }
    if character == '#' {
        lexer.state = LexerState::Comment;
        return None;
    }
    Some(character)
}

fn process_char_comment(lexer : &mut Lexer,  character : char) -> Option<char> {
    None
}

fn process_char_strong_quote(lexer : &mut Lexer, character : char) -> Option<char> {
    if character == '\'' {
        lexer.state = LexerState::Normal;
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
        lexer.state = LexerState::Normal;
        return None;
    }
    Some(character)
}

enum LexerState {
    Normal,
    WeakQuoting,
    StrongQuoting,
    HereDoc,
    Comment,
}

/// lex expects to receive a newline terminated string.
/// lex should only be used on a single line at a time.
pub fn lex(input : &str) -> Vec<String> {
    let mut lexer = Lexer { escaping: false, state: LexerState::Normal };
    let mut result : Vec<String> = vec![];
    let whitespace = vec![' ', '\n', '\t'];
    let mut current_lexeme = String::new();
    let special = vec!['`', '$', '\\'];  // Part of terrible escape hack
    for character in input.chars() {

        // TODO(Skyler) Fix terrible hack to make escaping work
        match lexer.state {
            LexerState::WeakQuoting => {
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
                LexerState::WeakQuoting => {
                }
                LexerState::StrongQuoting => (),
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

    #[test]
    fn lex_comment() {
        let expected = vec!["okay"];
        let actual = lex("okay#${}()\";'a\\\n");
        assert_eq!(expected, actual);
    }

}
