#[cfg(test)]
mod url_paths_parsing_tests {
    use ukma_url_parser::parse_url_paths;

    #[test]
    fn url_parser_parses_path_with_one_part_correctly() {
        assert_eq!(parse_url_paths("/users"), Ok(vec!["users"]));
    }

    #[test]
    fn url_parser_parses_path_with_couple_of_parts_correctly() {
        assert_eq!(
            parse_url_paths("/users/ukraine/kyiv/2"),
            Ok(vec!["users", "ukraine", "kyiv", "2"])
        );
    }

    #[test]
    fn url_parser_throws_error_if_path_format_is_not_correct() {
        assert!(parse_url_paths("incorrect_format").is_err());
    }

    #[test]
    fn url_parser_throws_error_if_paths_have_whitespaces() {
        assert!(parse_url_paths("/  Ukraine").is_err());
    }
}
