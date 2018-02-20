extern crate flugel_gerber;
use flugel_gerber::lex::lex;

fn main() {
    for pair in lex("+012345") {
        println!("{:?}", pair);
    }
}
