use std::error::Error;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("src/data/day2.csv").expect("Cannot open file"));

    let (mut part_one_x, mut part_one_y) = (0, 0);
    let (mut part_two_x, mut part_two_y, mut part_two_aim) = (0, 0, 0);

    for line in reader.lines() {
        let text = line?;
        let mut split = text.split_whitespace();

        let command = split.next().expect("No command found");
        let value = split.next().expect("No value found");
        let inc = value.parse::<i32>().unwrap();

        match command {
            "forward" => {
                part_one_x += inc;
                part_two_x += inc;
                part_two_y += part_two_aim * inc;
            }
            "down" => {
                part_one_y -= inc;
                part_two_aim += inc;
            }
            "up" => {
                part_one_y += inc;
                part_two_aim -= inc
            }
            _ => println!("Command {} not found", command),
        }
    }

    println!("Part one: {}", part_one_x.abs() * part_one_y.abs());
    println!("Part two: {}", part_two_x.abs() * part_two_y.abs());

    Ok(())
}
