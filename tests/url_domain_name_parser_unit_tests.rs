#[cfg(test)]
mod url_domain_parsing_tests {
    use ukma_url_parser::parse_domain_name;
    use ukma_url_parser::URLScheme;

    #[test]
    fn url_parser_parses_http_domain_correctly() {
        assert_eq!(
            parse_domain_name("ukma-university.com"),
            Ok(vec!["ukma-university", "com"])
        );
    }

    #[test]
    fn url_parser_parses_http_domain_with_couple_of_subdomains_correctly() {
        assert_eq!(
            parse_domain_name("www.dist.ukma-university.com"),
            Ok(vec!["www", "dist", "ukma-university", "com"])
        );
    }

    #[test]
    fn url_parser_throws_error_if_domain_is_not_correct() {
        assert!(parse_domain_name("ukma-university.").is_err());
    }

    #[test]
    fn url_parser_throws_error_if_domain_has_whitespaces() {
        assert!(parse_domain_name("ukma    .com").is_err());
    }

    #[test]
    fn url_parser_throws_error_if_domain_label_starts_with_number() {
        assert!(parse_domain_name("4ukma.com").is_err());
    }
}
