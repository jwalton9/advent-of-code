mod day1;
mod day2;
mod day3;

fn main() {
    let day = std::env::args().nth(1).expect("No day given");

    match day.as_str() {
        "day1" => day1::main(),
        "day2" => day2::main(),
        "day3" => day3::main(),
        _ => panic!("Day not found"),
    }
    .unwrap();
}
