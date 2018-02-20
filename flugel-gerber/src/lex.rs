use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("lex.pest");

#[derive(Parser)]
#[grammar = "lex.pest"]
pub struct Lex;

#[cfg(test)]
mod tests {
    use super::Lex;

    #[test]
    fn sanity() {
        let pairs = Lex::parse(Rule::integer, "+012345").unwrap();
        for pair in pairs {
            // A pair is a combination of the rule which matched and a span of input
            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", pair.clone().into_span());
            println!("Text:    {}", pair.clone().into_span().as_str());
            println!("");
        }
    }
}
