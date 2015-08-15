use std::collections::HashMap;
use super::lexer;

pub fn expand_aliases(aliases : &HashMap<String, String>, tokens : &mut Vec<String>) -> Vec<String> {
    let mut seen : Vec<String> = vec![];
    _expand_aliases(aliases, tokens, &mut seen)
}

fn _expand_aliases(
        aliases : &HashMap<String, String>, 
        tokens : &mut Vec<String>,
        aliases_seen : &mut Vec<String>) -> Vec<String> {
    let mut results : Vec<String> = vec![];
    for token in tokens.drain(..) {
        let seen = aliases_seen.contains(&token);
        match (aliases.get(&token), seen) {
            (Some(alias), false) => {
                aliases_seen.push(token);
                let mut lexer_results = lexer::lex(&alias[..]);
                let mut expanded_lexer_results = _expand_aliases(aliases, &mut lexer_results, aliases_seen);
                for lexed_token in expanded_lexer_results.drain(..) {
                    results.push(lexed_token);
                }
            },
            _ => results.push(token)
        }
    }
    results
}


#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn no_aliases() {
        let mut tokens = vec!["ls".to_string(), "-al".to_string(), "*.py".to_string()];
        let expected = vec!["ls".to_string(), "-al".to_string(), "*.py".to_string()];
        assert_eq!(expected, expand_aliases(&HashMap::new(), &mut tokens));
    }

    #[test]
    fn with_aliases() {
        let mut tokens = vec!["l".to_string(), "-al".to_string(), "*.py".to_string()];
        let mut aliases = HashMap::new();
        aliases.insert("l".to_string(), "ls".to_string());
        let expected = vec!["ls".to_string(), "-al".to_string(), "*.py".to_string()];
        assert_eq!(expected, expand_aliases(&aliases, &mut tokens));
    }

    #[test]
    fn alias_splits_tokens_after_expansion() {
        let mut tokens = vec!["lc".to_string(), "-al".to_string(), "*.py".to_string()];
        let mut aliases = HashMap::new();
        aliases.insert("lc".to_string(), "ls --color".to_string());
        let expected = vec!["ls".to_string(), "--color".to_string(), "-al".to_string(), "*.py".to_string()];
        assert_eq!(expected, expand_aliases(&aliases, &mut tokens));
    }

    #[test]
    fn nested_aliases() {
        let mut tokens = vec!["sl".to_string()];
        let mut aliases = HashMap::new();
        aliases.insert("sl".to_string(), "ls".to_string());
        aliases.insert("ls".to_string(), "cd".to_string());
        let expected = vec!["cd".to_string()];
        assert_eq!(expected, expand_aliases(&aliases, &mut tokens));
    }

    #[test]
    fn infinitely_recursive_alias() {
        let mut tokens = vec!["ls".to_string()];
        let mut aliases = HashMap::new();
        aliases.insert("ls".to_string(), "ls --color".to_string());
        let expected = vec!["ls".to_string(), "--color".to_string()];
        assert_eq!(expected, expand_aliases(&aliases, &mut tokens));
    }

}
