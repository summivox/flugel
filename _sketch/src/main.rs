fn f(x: i32) -> bool {
    match x {
        1...10 => true,
        _ => false,
    }
}

fn main() {
    for i in 0..12 {
        println!("{i:3} {j:6}", i = i, j = f(i));
    }
}
