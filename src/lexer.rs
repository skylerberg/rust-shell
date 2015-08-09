pub fn lex(input : &str) -> Vec<String> {
    let mut result : Vec<String> = vec![];
    let whitespace = vec![' ', '\n'];
    let mut quoted = false;
    let mut current_lexeme = String::new();
    for character in input.chars() {
        if character == '"' {
            quoted = !quoted;
            continue;
        }
        if quoted || !whitespace.contains(&character) {
            current_lexeme.push(character);
        }
        else {
            if !current_lexeme.is_empty() {
                result.push(current_lexeme);
                current_lexeme = String::new();
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
    fn lex_double_quoted() {
        let expected = vec!["cd", "dir with spaces"];
        let actual = lex("cd \"dir with spaces\"\n");
        assert_eq!(expected, actual);
    }

}
