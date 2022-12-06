use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No file specified");
        std::process::exit(1);
    }

    let reader = BufReader::new(File::open(&args[1])?);

    let input = reader.lines().flatten()
        .next().unwrap();
    
    part1(&input);
    part2(input);

    Ok(())
}

fn has_n_unique(n: usize, chars: &[u8]) -> bool {
    for (i, &ch) in chars.iter().take(n.min(chars.len())).enumerate() {
        if chars[..i].contains(&ch) {
            return false;
        }
    }
    true
}

fn part1(inp: &String) {
    let inp = inp.as_bytes();
    for i in 0..inp.len() {
        if has_n_unique(4, &inp[i..]) {
            println!("Part 1: {}", i + 4);
            break;
        }
    }
}

fn part2(inp: String) {
    let inp = inp.as_bytes();
    for i in 0..inp.len() {
        if has_n_unique(14, &inp[i..]) {
            println!("Part 2: {}", i + 14);
            break;
        }
    }
}