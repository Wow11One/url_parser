#[cfg(test)]
mod url_grammar_unit_tests {
    use pest::Parser;
    use ukma_url_parser::*;

    // Створюємо функцію для спрощення тестів
    fn parse_rule(rule: Rule, input: &str) -> Result<(), String> {
        URLPestParser::parse(rule, input)
            .map(|_| ())
            .map_err(|e| format!("Parsing error: {}", e))
    }

    #[test]
    fn test_number() {
        assert!(parse_rule(Rule::number, "123").is_ok());
        assert!(parse_rule(Rule::number, "456789").is_ok());
    }

    #[test]
    fn test_letter() {
        assert!(parse_rule(Rule::letter, "a").is_ok());
        assert!(parse_rule(Rule::letter, "Z").is_ok());
    }

    #[test]
    fn test_scheme() {
        assert!(parse_rule(Rule::scheme, "http").is_ok());
        assert!(parse_rule(Rule::scheme, "ftp").is_ok());
    }

    #[test]
    fn test_word() {
        assert!(parse_rule(Rule::word, "hello").is_ok());
        assert!(parse_rule(Rule::word, "World").is_ok());
    }

    #[test]
    fn test_word_with_numbers() {
        assert!(parse_rule(Rule::word_with_numbers, "hello123").is_ok());
        assert!(parse_rule(Rule::word_with_numbers, "test456test").is_ok());
    }

    #[test]
    fn test_label() {
        assert!(parse_rule(Rule::label, "example").is_ok());
        assert!(parse_rule(Rule::label, "sub-domain123").is_ok());
    }

    #[test]
    fn test_domain_name() {
        assert!(parse_rule(Rule::domain_name, "example.com").is_ok());
        assert!(parse_rule(Rule::domain_name, "sub.example.co.uk").is_ok());
    }

    #[test]
    fn test_port() {
        assert!(parse_rule(Rule::port, "8080").is_ok());
        assert!(parse_rule(Rule::port, "443").is_ok());
    }

    #[test]
    fn test_paths() {
        assert!(parse_rule(Rule::paths, "/path/to/resource").is_ok());
        assert!(parse_rule(Rule::paths, "/another/path").is_ok());
    }

    #[test]
    fn test_key_value_pair() {
        assert!(parse_rule(Rule::key_value_pair, "key=value").is_ok());
        assert!(parse_rule(Rule::key_value_pair, "param123=val456").is_ok());
    }

    #[test]
    fn test_parameters() {
        assert!(parse_rule(Rule::parameters, "key=value&param=val").is_ok());
        assert!(parse_rule(Rule::parameters, "foo=bar&test123=example456").is_ok());
    }

    #[test]
    fn test_fragment() {
        assert!(parse_rule(Rule::fragment, "section1").is_ok());
        assert!(parse_rule(Rule::fragment, "part2").is_ok());
    }

    #[test]
    fn test_url() {
        assert!(parse_rule(Rule::url, "http://example.com").is_ok());
        assert!(parse_rule(
            Rule::url,
            "http://example.com:8080/path/to/page?param=value#section"
        )
        .is_ok());
    }
}
