use std::io::{BufRead, BufReader};
use std::env;
use std::fs::File;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("No input file sepcified");
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut overlap_count: u16 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let pairs: Vec<&str> = line.split(',').collect();

        let first_elf = pairs[0];
        let second_elf = pairs[1];
    
        if contains_range(first_elf, second_elf) {
            overlap_count += 1;
        }
    }

    println!("{}", overlap_count);
    
    Ok(())
}

fn contains_range(first: &str, second: &str) -> bool {
    let first_split: Vec<&str> = first.split('-').collect();
    let a: u8 = first_split[0].parse().unwrap();
    let b: u8 = first_split[1].parse().unwrap();

    let second_split: Vec<&str> = second.split('-').collect();
    let c: u8 = second_split[0].parse().unwrap();
    let d: u8 = second_split[1].parse().unwrap();


    let r1 = a..=b;
    let r2 = c..=d;
    r1.contains(&c) || r1.contains(&d) || r2.contains(&a) || r2.contains(&b)
}