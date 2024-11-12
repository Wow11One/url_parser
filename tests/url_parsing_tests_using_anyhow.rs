#[cfg(test)]
mod url_parsing_tests_using_anyhow {
    use anyhow::{Context, Result};
    use std::collections::HashMap;
    use ukma_url_parser::URLError;

    use ukma_url_parser::parse_url;
    use ukma_url_parser::URLScheme;

    #[test]
    fn test_parse_url_successful() -> Result<()> {
        let url = "http://example.com:80/path?key=value#fragment";
        let parsed = parse_url(url).context("Failed to parse a valid URL")?;

        assert_eq!(parsed.url_scheme, URLScheme::Http);
        assert_eq!(
            parsed.domain_parts,
            vec!["example".to_string(), "com".to_string()]
        );
        assert_eq!(parsed.port, Some(80));
        assert_eq!(parsed.paths, vec!["path".to_string()]);

        let mut expected_params = HashMap::new();
        expected_params.insert("key".to_string(), "value".to_string());
        assert_eq!(parsed.parameters, expected_params);

        assert_eq!(parsed.fragment, Some("fragment".to_string()));
        Ok(())
    }

    #[test]
    fn test_parse_url_invalid_scheme() -> Result<()> {
        let url = "unknown://example.com";
        match parse_url(url) {
            Err(URLError::ParsingError) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Unexpected error: {}", e)),
            _ => Err(anyhow::anyhow!(
                "Expected InvalidPort error, but got success"
            )),
        }
    }

    #[test]
    fn test_parse_url_no_parameters() -> Result<()> {
        let url = "http://example.com/path#fragment";
        let parsed = parse_url(url).context("Failed to parse URL without parameters")?;

        assert_eq!(parsed.parameters.len(), 0);
        Ok(())
    }
}
