use std::{
    cmp,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type Coordinate = (usize, usize);

pub fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("./src/data/day5.txt").expect("Failed to open file"));

    let mut line_defs: Vec<(Coordinate, Coordinate)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for line in reader.lines() {
        let mut line_def: Vec<Coordinate> = Vec::new();

        for coords in line?.split(" -> ") {
            let string_coords: Vec<&str> = coords.split(",").collect();

            let x = usize::from_str_radix(string_coords[0], 10).unwrap();
            let y = usize::from_str_radix(string_coords[1], 10).unwrap();

            line_def.push((x, y));
            max_x = cmp::max(max_x, x);
            max_y = cmp::max(max_y, y);
        }

        line_defs.push((line_def[0], line_def[1]));
    }

    let mut grid = vec![vec![0; max_x + 1]; max_y + 1];

    for ((x1, y1), (x2, y2)) in line_defs {
        let x_min = cmp::min(x1, x2);
        let x_max = cmp::max(x1, x2);

        let y_min = cmp::min(y1, y2);
        let y_max = cmp::max(y1, y2);

        if y2 == y1 {
            for x in x_min..=x_max {
                grid[y1][x] += 1;
            }
        } else if x2 == x1 {
            for y in y_min..=y_max {
                grid[y][x1] += 1;
            }
        } else {
            let m = (y2 as i32 - y1 as i32) / (x2 as i32 - x1 as i32);

            for x in x_min..=x_max {
                let delta = x - x_min;

                let y = if m == 1 { y_min + delta } else { y_max - delta };

                grid[y][x] += 1;
            }
        }
    }

    let mut sum: u32 = 0;

    for row in grid {
        for value in row {
            if value > 1 {
                sum += 1;
            }
        }
    }

    println!("Part one: {}", sum);

    Ok(())
}
