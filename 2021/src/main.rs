mod day1;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = std::env::args().nth(1).expect("No day given");

    match day.as_str() {
        "day1" => day1::main()?,
        _ => println!("Day not found"),
    };

    Ok(())
}
