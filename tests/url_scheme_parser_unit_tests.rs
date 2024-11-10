#[cfg(test)]
mod url_scheme_parsing_tests {
    use ukma_url_parser::parse_url_scheme;
    use ukma_url_parser::URLScheme;

    #[test]
    fn url_parser_parses_http_scheme_correctly() {
        assert_eq!(parse_url_scheme("http"), Ok(URLScheme::Http));
    }

    #[test]
    fn url_parser_parses_ftp_scheme_correctly() {
        assert_eq!(parse_url_scheme("ftp"), Ok(URLScheme::Ftp));
    }

    #[test]
    fn url_parser_parses_http_scheme_correctly_ignoring_case() {
        assert_eq!(parse_url_scheme("HTTP"), Ok(URLScheme::Http));
    }

    #[test]
    fn url_parser_parses_ftp_scheme_correctly_ignoring_case() {
        assert_eq!(parse_url_scheme("FtP"), Ok(URLScheme::Ftp));
    }

    #[test]
    fn url_parser_throws_error_if_scheme_is_not_correct() {
        assert!(parse_url_scheme("invalid_scheme").is_err());
    }

    #[test]
    fn url_parser_throws_error_if_scheme_is_empty() {
        assert!(parse_url_scheme("").is_err());
    }
}
