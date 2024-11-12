#[cfg(test)]
mod url_parameters_parsing_tests {
    use std::collections::HashMap;

    use ukma_url_parser::parse_url_parameters;

    #[test]
    fn url_parser_parses_parameters_correctly() {
        assert_eq!(
            parse_url_parameters("vova=havryliuk&ukraine=love&ukma=home"),
            Ok(HashMap::from([
                ("vova", "havryliuk"),
                ("ukraine", "love"),
                ("ukma", "home"),
            ]))
        );
    }

    #[test]
    fn url_parser_throws_error_if_parameters_do_not_have_value() {
        assert!(parse_url_parameters("vova=").is_err());
    }

    #[test]
    fn url_parser_throws_error_if_parameters_contain_invalid_symbol() {
        assert!(parse_url_parameters("vov.a=havryliuk").is_err());
    }
}
