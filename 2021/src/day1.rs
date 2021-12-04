use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("src/data/day1.csv").expect("Cannot open file"));

    let mut increases = 0;
    let mut last_seen = 0;

    let mut windows: HashMap<usize, i32> = HashMap::new();
    let mut window_increases = 0;

    for (i, line) in reader.lines().enumerate() {
        let value = line?.parse()?;

        if (last_seen > 0) & (value > last_seen) {
            increases += 1
        }

        windows
            .entry(i)
            .and_modify(|x| *x += value)
            .or_insert(value);

        if i > 0 {
            windows.entry(i - 1).and_modify(|x| *x += value);
        }

        if i > 1 {
            windows.entry(i - 2).and_modify(|x| *x += value);

            if i > 2 && windows[&(i - 2)] > windows[&(i - 3)] {
                window_increases += 1;
            }
        }

        last_seen = value
    }

    println!("Number of increases: {}", increases);
    println!("Number of window increases: {}", window_increases);

    Ok(())
}
