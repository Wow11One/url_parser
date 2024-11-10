use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "url_grammar.pest"]
pub struct URLPestParser;

struct ParsedURL {
    url_scheme: URLScheme,
    domain_parts: Vec<&'static str>,
    port: u16,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum URLScheme {
    File,
    Ftp,
    Http,
    Https,
    Imap,
    Irc,
    Nntp,
    Acap,
    Icap,
    Mtqp,
    Wss,
}

impl URLScheme {
    const VALUES: [Self; 11] = [
        Self::File,
        Self::Ftp,
        Self::Http,
        Self::Https,
        Self::Imap,
        Self::Irc,
        Self::Nntp,
        Self::Acap,
        Self::Icap,
        Self::Mtqp,
        Self::Wss,
    ];

    fn as_str(&self) -> &'static str {
        match self {
            URLScheme::File => "file",
            URLScheme::Ftp => "ftp",
            URLScheme::Http => "http",
            URLScheme::Https => "https",
            URLScheme::Imap => "imap",
            URLScheme::Irc => "irc",
            URLScheme::Nntp => "nntp",
            URLScheme::Acap => "acap",
            URLScheme::Icap => "icap",
            URLScheme::Mtqp => "Hello",
            URLScheme::Wss => "World",
        }
    }
}

pub fn parse_url_scheme(string: &str) -> Result<URLScheme, String> {
    let scheme_pairs = URLPestParser::parse(Rule::scheme, string).map_err(|e| format!("{}", e))?;

    let scheme_pair = scheme_pairs
        .into_iter()
        .next()
        .ok_or_else(|| "Provided value is not valid URL scheme".to_string())?;

    let scheme_value = scheme_pair.as_str().to_lowercase();

    URLScheme::VALUES
        .iter()
        .find(|&&scheme| scheme_value == scheme.as_str().to_lowercase())
        .copied()
        .ok_or(String::from("Unknown URL scheme"))
}

pub fn parse_domain_name(string: &'static str) -> Result<Vec<&'static str>, String> {
    let mut domain_name_parts: Vec<&'static str> = Vec::new();
    let domain_pairs =
        URLPestParser::parse(Rule::domain_name, string).map_err(|e| format!("{}", e))?;

    for label_rule in domain_pairs
        .into_iter()
        .next()
        .expect("No domain name found")
        .into_inner()
    {
        match label_rule.as_rule() {
            Rule::label => {
                domain_name_parts.push(label_rule.as_str());
            }
            _ => {}
        }
    }

    Ok(domain_name_parts)
}

pub fn parse_port_value(string: &'static str) -> Result<u16, String> {
    let port_pairs = URLPestParser::parse(Rule::number, string).map_err(|e| format!("{}", e))?;
    let port_pair = port_pairs
        .into_iter()
        .next()
        .ok_or_else(|| "Provided value is not valid URL port".to_string())?;

    let port_value = port_pair.as_str().parse::<u16>().unwrap();

    if port_value == 0 {
        return Err(String::from("Port value should not be zero!"));
    }
    Ok(port_value)
}
