use std::{collections::HashMap, fs};

pub fn main() -> () {
    let fish: Vec<u64> = fs::read_to_string("./src/data/day6.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(|str| u64::from_str_radix(str, 10).unwrap())
        .collect();

    let mut population: HashMap<u64, u64> = HashMap::new();

    for f in fish {
        let entry = population.entry(f).or_insert(0);
        *entry += 1;
    }

    for _day in 1..=256 {
        let mut next_day: HashMap<u64, u64> = HashMap::new();

        for (key, value) in population.clone() {
            let new_age = if key == 0 { 6 } else { key - 1 };

            let entry = next_day.entry(new_age).or_insert(0);

            *entry += value;

            if key == 0 {
                let new_fish = next_day.entry(8).or_insert(0);

                *new_fish += value;
            }
        }

        population = next_day;
    }

    let mut sum = 0;

    for value in population.values() {
        sum += value;
    }

    println!("Part two: {}", sum);
}
