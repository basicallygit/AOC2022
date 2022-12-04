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
                    'X' => {score += 3},
                    'Y' => {score += 4},
                    'Z' => {score += 8},
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
                    'X' => {score += 2},
                    'Y' => {score += 6},
                    'Z' => {score += 7},
                    _ => {}
                }
            }
            _ => {}
        }
    }
    println!("{}", score);

    Ok(())
}
