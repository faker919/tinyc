use std::io::{self, BufRead, Write};

fn main() {
    print!("cal> ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        println!("{}", line.unwrap());
        print!("cal> ");
        io::stdout().flush().unwrap();
    }
}
