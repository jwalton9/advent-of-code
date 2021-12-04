use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_bit_at(x: u32, n: u8) -> Result<bool, &'static str> {
    if n < 32 {
        Ok(x & 1 << n != 0)
    } else {
        Err("Bit out of range")
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("src/data/day3.csv").expect("Cannot open file"));

    let mut freq: HashMap<u8, (u32, u32)> = HashMap::new();

    for line in reader.lines() {
        let bin = u32::from_str_radix(line.unwrap().as_str(), 2).unwrap();

        for i in 0..31 {
            let (zero_count, one_count) = freq.entry(i).or_insert((0, 0));
            match get_bit_at(bin, i).unwrap() {
                true => *one_count += 1,
                false => *zero_count += 1,
            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..31 {
        let (zero_count, one_count) = freq.get(&i).expect("No count found for bit");
        println!("{}, {}", zero_count, one_count);

        if one_count == &0 {
            break;
        } else if one_count > zero_count {
            gamma_rate |= 1 << i;
        } else {
            epsilon_rate |= 1 << i;
        }
    }

    println!("Gamma Rate: {}, Epsilon Rate: {}", gamma_rate, epsilon_rate);
    println!("Part one: {}", gamma_rate * epsilon_rate);

    return Ok(());
}
