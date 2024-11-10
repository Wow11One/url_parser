use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "url_grammar.pest"]
pub struct URLPestParser;

fn main() {
    let successful_parse1 = URLPestParser::parse(Rule::scheme, "https");
    println!("{:?}", successful_parse1);

    let unsuccessful_parse = URLPestParser::parse(Rule::scheme, "ftp");
    println!("{:?}", unsuccessful_parse);

    let successful_parse3 = URLPestParser::parse(Rule::scheme, "HTTPs");
    println!("{:?}", successful_parse3);

    let successful_parse4 = URLPestParser::parse(Rule::domain_name, "jdbc.com");
    println!("{:?}", successful_parse4);
}
