use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::env;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("No input file specified");
        std::process::exit(0);
    }

    let line_count = get_line_count(&args[1])? / 3;
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;
    let mut add_sum = |chr: char| if chr.is_lowercase() {sum += chr as u32 - 96} else {sum += chr as u32 - 38};

    let mut lines = reader.lines();

    for _ in 0..line_count {
        let first_elf = lines.next().unwrap()?;
        let second_elf = lines.next().unwrap()?;
        let third_elf = lines.next().unwrap()?;

        for chr in first_elf.chars() {
            if second_elf.contains(chr) && third_elf.contains(chr) {
                add_sum(chr);
                break;
            }
        }
    }
    println!("{}", sum);
    
    Ok(())
}

fn get_line_count(fname: &str) -> Result<usize, Error> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().count())
}