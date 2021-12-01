use std::fs::File;
use std::io::{prelude::*, BufReader};

#[allow(non_snake_case)]

fn main() {
    day_one();
}

// DAY ONE

fn day_one() {
    let mut lines: Vec<usize> = Vec::new();

    // Get input
    let input_file_name = "day_one_input.txt";

    let file = File::open(input_file_name).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        lines.push(line.parse::<usize>().unwrap());
    }

    // Part 1
    {
        let mut times_it_increased: usize = 0;
        let mut previous_value: usize = 0;
        for val in &lines {
            if previous_value != 0 {
                if val > &previous_value {
                    times_it_increased += 1;
                }
                previous_value = *val;
            } else {
                previous_value = *val;
            }
        }

        println!("{}", times_it_increased);
    }
    // Part 2
    {
        let mut previous_sum: usize = 0;
        let mut times_it_increased: usize = 0;
        let mut sti: usize = 0;
        let mut eni: usize = 2;

        while eni < lines.len() {
            let current_sum = lines[sti] + lines[sti + 1] + lines[eni];
            if previous_sum != 0 {
                if current_sum > previous_sum {
                    times_it_increased += 1;
                }
            }
            sti += 1;
            eni += 1;
            previous_sum = current_sum;
        }
        println!("{}", times_it_increased);
    }
}


// DAY TWO