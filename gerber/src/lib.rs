#[macro_use]
extern crate decimal;
extern crate extprim;
#[macro_use]
extern crate extprim_literals;
extern crate num;
extern crate peg;

pub mod types;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::parser;

    #[test]
    fn uint() {}
}
