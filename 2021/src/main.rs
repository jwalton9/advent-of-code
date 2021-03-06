mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let day = std::env::args().nth(1).expect("No day given");

    match day.as_str() {
        "day1" => day1::main(),
        "day2" => day2::main(),
        "day3" => day3::main(),
        "day4" => day4::main(),
        "day5" => day5::main(),
        "day6" => day6::main(),
        _ => panic!("Day not found"),
    }
}
