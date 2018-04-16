extern crate flugel_gerber;
use flugel_gerber::parser;

#[macro_use]
extern crate decimal;
use decimal::d128;

fn main() {
    {
        let input = "-.0142857000999000";
        let ret = parser::dec(input).unwrap();
        println!("{}", ret);
    }
    {
        let input = "123xyzabc jklMNOpqer#@!:";
        let ret = parser::string(input).unwrap();
        println!("{}", ret);
    }
    {
        let input = "5D6A7F";
        let ret = parser::hex_u32(input).unwrap();
        println!("{:x}", ret);
    }
    {
        let input = "%FSLAX25Y14*%";
        println!("{:?}", parser::command(input));
    }
    {
        println!("{:?}", parser::command("%MOIN*%"));
        println!("{:?}", parser::command("%MOMM*%"));
    }
    {
        println!("{:?}", parser::command("G01*"));
        println!("{:?}", parser::command("X+12Y-34J0D0003*"));
    }
    {
        println!("{:?}", parser::command("%SR*%"));
    }
    {
        println!("{:?}", parser::command("%SRX3Y2I5.0J4.0*%"));
    }
    {
        let input = "G04.#mycomment:() test*";
        println!("{:?}", parser::command(input));
    }
    {
        println!("{:?}", parser::command("%ADD12C*%"));
        println!("{:?}", parser::command("%ADD10C,.025*%"));
        println!("{:?}", parser::command("%ADD10C,0.5X0.25*%"));
        println!("{:?}", parser::command("\
            %ADD146Rect,\
            0.0807087X0.1023622X0.0118110X0.5000000X0.3000000*%"));
    }
    {
        println!("{:?}", parser::am_expr("-$1x-(-$2-3.12/+$3)"));
    }
    {
        let input = "\
            %AM.DONUTCAL*\
            1,1,$1,$2,$3*\
            $4=$1x0.75*\
            1,0,$4,$2,$3*%\
        ";
        println!("{:?}", input);
        println!("{:?}", parser::command(input));
    }
}
