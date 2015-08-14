lazy_static! {
    static ref OPERATORS : Vec<&'static str> = vec![
        "&&",
        "||",
        ";;",
        "<<",
        ">>",
        "<<-",
    ];

    static ref OPERATOR_STARTERS : Vec<char> = vec![
        '&',
        '|',
        ';',
        '<',
        '>',
    ];
}

struct Lexer {
    state : LexerQuotingState,
    escaping : bool,
    current_token : String,
    tokens : Vec<String>,
    current_token_type : TokenType,
}

enum LexerQuotingState {
    Normal,
    WeakQuoting,
    StrongQuoting,
    HereDoc,
    Comment,
}

enum TokenType {
    Word,
    Operator,
    Undetermined,
}

impl Lexer {
    fn process_char(&mut self, character : char) -> Option<char> {
        match self.state {
            LexerQuotingState::WeakQuoting => process_char_weak_quote(self, character),
            LexerQuotingState::StrongQuoting => process_char_strong_quote(self, character),
            LexerQuotingState::Comment => process_char_comment(self, character),
            _ => process_char_normal(self, character),
        }
    }

    fn delimit_token(mut self) -> Lexer {
        if !self.current_token.is_empty() {
            let result = self.current_token;
            self.tokens.push(result);
            self.current_token = String::new();
            self.current_token_type = TokenType::Undetermined;
        }
        self
    }

    fn should_delimit(&self, next : char) -> bool {

        match self.state {  // We should not delimit if we are quoting
            LexerQuotingState::Normal => (),
            _ => return false,
        }

        let whitespace = vec![' ', '\n', '\t'];
        if whitespace.contains(&next) {
            return true;
        }

        match self.current_token_type {
            TokenType::Word => {
                if OPERATOR_STARTERS.contains(&next) && !self.escaping {
                    return true;
                }
            },
            TokenType::Operator => {
                let position = self.current_token.len();
                for operator in OPERATORS.iter() {
                    let nth_character = operator.chars().nth(position);
                    if nth_character.is_some() {
                        if next == nth_character.unwrap() {
                            return false;
                        }
                    }
                }
                return true;
            },
            _ => (),
        }

        false
    }

    fn push_char(&mut self, character : char) {
        if self.current_token.is_empty() {
            if OPERATOR_STARTERS.contains(&character) {
                self.current_token_type = TokenType::Operator;
            }
            else {
                self.current_token_type = TokenType::Word;
            }
        }
        self.current_token.push(character);
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
        lexer.state = LexerQuotingState::StrongQuoting;
        return None;
    }
    if character == '"' {
        lexer.state = LexerQuotingState::WeakQuoting;
        return None;
    }
    if character == '#' {
        lexer.state = LexerQuotingState::Comment;
        return None;
    }
    Some(character)
}

fn process_char_comment(lexer : &mut Lexer,  character : char) -> Option<char> {
    None
}

fn process_char_strong_quote(lexer : &mut Lexer, character : char) -> Option<char> {
    if character == '\'' {
        lexer.state = LexerQuotingState::Normal;
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
        lexer.state = LexerQuotingState::Normal;
        return None;
    }
    Some(character)
}

/// lex expects to receive a newline terminated string.
/// lex should only be used on a single line at a time.
pub fn lex(input : &str) -> Vec<String> {
    let mut lexer = Lexer { 
        escaping: false, 
        state: LexerQuotingState::Normal,
        current_token: String::new(),
        tokens : vec![],
        current_token_type : TokenType::Undetermined,
    };
    let whitespace = vec![' ', '\n', '\t'];
    let special = vec!['`', '$', '\\'];  // Part of terrible escape hack
    for character in input.chars() {

        // TODO(Skyler) Fix terrible hack to make escaping work
        match lexer.state {
            LexerQuotingState::WeakQuoting => {
                if lexer.escaping && !special.contains(&character) {
                    lexer.push_char('\\');
                }
            },
            _ => (),
        }

        if lexer.should_delimit(character) {
            lexer = lexer.delimit_token();
        }

        match lexer.process_char(character) {
            Some(chr) => lexer.push_char(chr),
            None => (),
        }

    }

    let has_remaining_token = !lexer.current_token.is_empty();
    if has_remaining_token {
        lexer = lexer.delimit_token();
    }
    lexer.tokens
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

    #[test]
    fn lex_operator() {
        let expected = vec!["cd", "&&", "ls"];
        let actual = lex("cd&&ls\n");
        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_many_operators() {
        let expected = vec!["cd", "&&", "||", "<<-", "ls"];
        let actual = lex("cd&&||<<-ls");
        assert_eq!(expected, actual);
    }
}
