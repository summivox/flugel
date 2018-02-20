extern crate pest;
use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("lex.pest");

#[derive(Parser)]
#[grammar = "lex.pest"]
pub struct Lex;

pub fn lex(input: &str) -> pest::iterators::Pairs<Rule> {
    Lex::parse(Rule::integer, input).unwrap()
}

#[cfg(test)]
mod tests {}
