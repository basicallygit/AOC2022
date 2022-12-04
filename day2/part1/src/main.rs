use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("No input file specified");
        std::process::exit(1);
    }
    
    let file = File::open(&args[1]).expect("Input file does not exist");
    let reader = BufReader::new(file);

    let mut score: u32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let opponent = line.chars().nth(0).unwrap();
        let mychoice = line.chars().nth(2).unwrap();
        match opponent {
            'A' => {
                match mychoice {
                    'X' => {score += 4},
                    'Y' => {score += 8},
                    'Z' => {score += 3},
                    _ => {}
                }
            },
            'B' => {
                match mychoice {
                    'X' => {score += 1},
                    'Y' => {score += 5},
                    'Z' => {score += 9},
                    _ => {}
                }
            },
            'C' => {
                match mychoice {
                    'X' => {score += 7},
                    'Y' => {score += 2},
                    'Z' => {score += 6},
                    _ => {}
                }
            }
            _ => {}
        }
    }
    println!("{}", score);

    Ok(())
}
