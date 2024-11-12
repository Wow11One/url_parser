use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

use thiserror::Error;

#[derive(Parser)]
#[grammar = "url_grammar.pest"]
pub struct URLPestParser;

#[derive(Debug, Error)]
pub enum URLError {
    #[error("Parsing error")]
    ParsingError,

    #[error("No domain name found in URL")]
    MissingDomainName,

    #[error("Failed to parse port as a valid u16")]
    InvalidPort,
}

#[derive(Debug)]
pub struct ParsedURL {
    pub url_scheme: URLScheme,
    pub domain_parts: Vec<String>,
    pub port: Option<u16>,
    pub paths: Vec<String>,
    pub parameters: HashMap<String, String>,
    pub fragment: Option<String>,
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
    Unknown,
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
            URLScheme::Unknown => "Unknown",
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

    // could do it with match, like in parse_url method, but decided to show different variants
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
        if label_rule.as_rule() == Rule::label {
            domain_name_parts.push(label_rule.as_str());
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
    let port_value_as_string = port_pair.as_str();
    let port_value = port_value_as_string.parse::<u16>().unwrap();

    Ok(port_value)
}

pub fn parse_url_paths(string: &'static str) -> Result<Vec<&'static str>, String> {
    let mut paths: Vec<&'static str> = Vec::new();
    let domain_pairs = URLPestParser::parse(Rule::paths, string).map_err(|e| format!("{}", e))?;

    for label_rule in domain_pairs
        .into_iter()
        .next()
        .expect("No domain name found")
        .into_inner()
    {
        if label_rule.as_rule() == Rule::word_with_numbers {
            paths.push(label_rule.as_str());
        }
    }

    Ok(paths)
}

pub fn parse_url_parameters(parameters_str: &str) -> Result<HashMap<&str, &str>, String> {
    let mut map = HashMap::new();

    let parsed = URLPestParser::parse(Rule::parameters, parameters_str)
        .map_err(|e| format!("Parsing error: {}", e))?;

    for pair in parsed {
        for inner_pair in pair.into_inner() {
            if let Rule::key_value_pair = inner_pair.as_rule() {
                let mut inner_rules = inner_pair.into_inner();

                let key = inner_rules.next().unwrap().as_str();
                let value = inner_rules.next().unwrap().as_str();

                map.insert(key, value);
            }
        }
    }

    Ok(map)
}

pub fn parse_url(url: &str) -> Result<ParsedURL, URLError> {
    let mut scheme = URLScheme::Unknown;
    let mut domain_parts = Vec::new();
    let mut port = None;
    let mut paths = Vec::new();
    let mut parameters = HashMap::new();
    let mut fragment = None;

    let parsed = URLPestParser::parse(Rule::url, url.trim()).map_err(|_| URLError::ParsingError)?;

    for pair in parsed
        .into_iter()
        .next()
        .ok_or(URLError::MissingDomainName)?
        .into_inner()
    {
        match pair.as_rule() {
            Rule::scheme => {
                scheme = match pair.as_str() {
                    "file" => URLScheme::File,
                    "ftp" => URLScheme::Ftp,
                    "http" => URLScheme::Http,
                    "imap" => URLScheme::Imap,
                    "irc" => URLScheme::Irc,
                    "nntp" => URLScheme::Nntp,
                    "acap" => URLScheme::Acap,
                    "icap" => URLScheme::Icap,
                    "mtqp" => URLScheme::Mtqp,
                    "wss" => URLScheme::Wss,
                    _other => URLScheme::Unknown,
                };
            }
            Rule::domain_name => {
                for label in pair.into_inner() {
                    domain_parts.push(label.as_str().to_string());
                }
            }
            Rule::port => {
                port = Some(
                    pair.as_str()
                        .parse::<u16>()
                        .map_err(|_| URLError::InvalidPort)?,
                );
            }
            Rule::paths => {
                for path in pair.into_inner() {
                    paths.push(path.as_str().to_string());
                }
            }
            Rule::parameters => {
                for param_pair in pair.into_inner() {
                    let mut inner_rules = param_pair.into_inner();
                    let key = inner_rules.next().unwrap().as_str().to_string();
                    let value = inner_rules.next().unwrap().as_str().to_string();
                    parameters.insert(key, value);
                }
            }
            Rule::fragment => {
                fragment = Some(pair.as_str().to_string());
            }
            _ => {}
        }
    }

    Ok(ParsedURL {
        url_scheme: scheme,
        domain_parts,
        port,
        paths,
        parameters,
        fragment,
    })
}
