use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No file specified");
        std::process::exit(1);
    }
    let mut elves: Vec<u32> = Vec::new();
    let mut cur_calories: u32 = 0;
    let file = File::open(&args[1]).expect("Input file does not exist");
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if !(line == "") {
            cur_calories += line.parse::<u32>().unwrap_or(0);
        }
        else {
            elves.push(cur_calories);
            cur_calories = 0;
        }
    }
    elves.push(cur_calories);
    println!("{}", find_largest(elves));

    Ok(())
}

fn find_largest(elves: Vec<u32>) -> u32 {
    let mut largest: u32 = 0;
    for elve in elves {
        if elve > largest {
            largest = elve;
        }
    }
    largest
}
