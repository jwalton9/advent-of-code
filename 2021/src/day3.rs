use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_most_common_bits(values: &Vec<String>) -> HashMap<usize, (u32, u32)> {
    let mut freq: HashMap<usize, (u32, u32)> = HashMap::new();

    for value in values {
        for (i, ch) in value.chars().enumerate() {
            let (zero_count, one_count) = freq.entry(i).or_insert((0, 0));

            if ch == '1' {
                *one_count += 1;
            } else if ch == '0' {
                *zero_count += 1;
            }
        }
    }

    return freq;
}

fn filter_bit_masks(values: &Vec<String>, bit: usize, filter_value: char) -> Vec<String> {
    let mut bm: Vec<String> = Vec::new();

    for value in values {
        match value.chars().nth(bit) {
            Some(c) if c == filter_value => bm.push(value.to_string()),
            _ => (),
        }
    }

    return bm;
}

fn calculate_bit_mask(
    values: &Vec<String>,
    bits: &HashMap<usize, (u32, u32)>,
    zero_value: char,
    one_value: char,
) -> String {
    let mut bit = 0;

    let mut bit_mask = values.clone();

    let mut most_common_bits = bits.clone();

    while bit_mask.len() > 1 {
        let (zero_count, one_count) = most_common_bits[&bit];

        let filter_value = if zero_count > one_count {
            zero_value
        } else {
            one_value
        };

        bit_mask = filter_bit_masks(&bit_mask, bit, filter_value);

        most_common_bits = get_most_common_bits(&bit_mask);

        bit += 1;
    }

    return bit_mask[0].to_string();
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("src/data/day3.csv").expect("Cannot open file"));

    let values: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let most_common_bits = get_most_common_bits(&values);

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for (i, (zero_count, one_count)) in most_common_bits.clone() {
        if one_count > zero_count {
            gamma_rate |= 1 << i;
        } else {
            epsilon_rate |= 1 << i;
        }
    }

    println!("Part one: {}", gamma_rate * epsilon_rate);

    let oxygen_generator = calculate_bit_mask(&values, &most_common_bits, '0', '1');
    let co2_generator = calculate_bit_mask(&values, &most_common_bits, '1', '0');

    let life_support = u32::from_str_radix(&oxygen_generator, 2).unwrap()
        * u32::from_str_radix(&co2_generator, 2).unwrap();

    println!("Part two: {}", life_support);

    return Ok(());
}
