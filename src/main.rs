use std::io;
use std::io::Write;

macro_rules! scanline {
    ($x:expr) => {
       io::stdin().read_line(&mut $x).unwrap();
    };
}

// io::stdin().read_to_string(&mut name);

// macro_rules! scan {
//     ($x:expr) => {
//         io::stdin().read_to_string(&mut $x).unwrap();
//     }
// }

// More on Macros: https://doc.rust-lang.org/1.7.0/book/macros.html
//
// :expr is no type. It's like a Metatype. Others are ident, path, ty, pat, stmt, block, item, tt
// These are types of arguments. 
macro_rules! flush {
    ($x:expr) => {
        $x().flush().expect("should flush here");
    }
}

fn main() {
    let zahl = 15;
    let mut name = String::new();

    print!("Name: ");

    // Try to replace flush with a macro
    // io::stdout().flush().expect("should flush here");
    flush!(io::stdout);

    scanline!(name);
    name = name.trim().to_string();
    println!("Hello, {}! {}", name, zahl * zahl);
}
