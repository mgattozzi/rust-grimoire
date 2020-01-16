use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Type a number: ");
    let num = {
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input)?;
        input.trim().parse::<i64>()?
    };
    println!("The Answer to Life, The Universe, and Everything: {}", num);
    Ok(())
}
