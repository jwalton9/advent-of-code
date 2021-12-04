mod day1;
mod day2;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let day = std::env::args().nth(1).expect("No day given");

    match day.as_str() {
        "day1" => day1::main()?,
        "day2" => day2::main()?,
        _ => println!("Day not found"),
    };

    Ok(())
}
