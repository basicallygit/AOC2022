use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("No input file specified");
        std::process::exit(0);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    let mut add_sum = |chr: char| if chr.is_lowercase() {sum += chr as u32 - 96} else {sum += chr as u32 - 38};

    for line in reader.lines() {
        let line = line.unwrap();
        let splitter = (line.len()) / 2;

        let first_half = &line[..splitter];
        let second_half = &line[splitter..];
        
        for chr in first_half.chars() {
            if second_half.contains(chr) {
                add_sum(chr);
                break;
            }
        }
    }
    println!("{}", sum);
    
    Ok(())
}
