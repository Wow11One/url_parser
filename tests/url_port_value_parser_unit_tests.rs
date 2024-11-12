#[cfg(test)]
mod url_port_parsing_tests {
    use ukma_url_parser::parse_port_value;

    #[test]
    fn url_parser_parses_port_value_correctly_first_variant() {
        assert_eq!(parse_port_value("8080"), Ok(8080));
    }

    #[test]
    fn url_parser_parses_port_value_correctly_second_variant() {
        assert_eq!(parse_port_value("3000"), Ok(3000));
    }

    #[test]
    fn url_parser_throws_error_if_port_value_is_not_number() {
        assert!(parse_port_value("fdfdf").is_err());
    }
}
