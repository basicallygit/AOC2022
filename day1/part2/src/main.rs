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

    for line in reader.lines() {
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

    let mut total_top3: u32 = 0;
    
    for _ in 0..3 {
        let (largest_index, largest) = find_largest(&elves);
        total_top3 += largest;
        elves.remove(largest_index as usize);
    }
    println!("{}", total_top3);
    
    Ok(())
}

fn find_largest(elves: &Vec<u32>) -> (u8, u32) {
    let mut largest: u32 = 0;
    let mut largest_index: u8 = 0;
    let mut i: u8 = 0;
    for elve in elves {
        if elve > &mut largest {
            largest = *elve;
            largest_index = i;
        }
        i += 1;
    }
    (largest_index, largest)
}
