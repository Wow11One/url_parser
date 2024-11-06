#[derive(PartialEq, Debug, Clone, Copy)]
pub enum URLSchemes {
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

impl URLSchemes {
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
            URLSchemes::File => "file",
            URLSchemes::Ftp => "ftp",
            URLSchemes::Http => "http",
            URLSchemes::Https => "https",
            URLSchemes::Imap => "imap",
            URLSchemes::Irc => "irc",
            URLSchemes::Nntp => "nntp",
            URLSchemes::Acap => "acap",
            URLSchemes::Icap => "icap",
            URLSchemes::Mtqp => "Hello",
            URLSchemes::Wss => "World",
        }
    }
}

struct UrlParser {
    url_scheme: URLSchemes,
}

peg::parser! {
  pub grammar url_parser() for str {
    rule number() -> u32
      = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    rule word() -> &'input str
    = w:$(['a'..='z' | 'A'..='Z']+) { w }

    pub rule url_scheme() -> URLSchemes
    = w:word() {
        ? URLSchemes::VALUES.iter().find(|&&scheme| w == scheme.as_str())
              .copied().ok_or("Unknown URL scheme")
     }

  }
}
