#[cfg(test)]
mod tests {
    use ukma_url_parser::url_parser;
    use ukma_url_parser::URLSchemes;

    #[test]
    fn url_parser_parses_http_scheme_correctly() {
        assert_eq!(url_parser::url_scheme("http"), Ok(URLSchemes::Http));
    }

    #[test]
    fn url_parser_parses_ftp_scheme_correctly() {
        assert_eq!(url_parser::url_scheme("ftp"), Ok(URLSchemes::Ftp));
    }

    #[test]
    fn url_parser_throws_error_if_scheme_is_not_correct() {
        assert!(url_parser::url_scheme("invalid_scheme").is_err());
    }
}
