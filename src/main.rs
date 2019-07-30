use std::io;
use std::io::Write;

macro_rules! scanline {
    ($x:expr) => {
       io::stdin().read_line(&mut $x).unwrap();
    };
}

fn main() {
    let zahl = 15;
    let mut name = String::new();
    // io::stdin().read_to_string(&mut name);
    print!("Name: ");
    io::stdout().flush().expect("should flush here");
    scanline!(name);
    name = name.trim().to_string();
    println!("Hello, {}! {}", name, zahl * zahl);
}
