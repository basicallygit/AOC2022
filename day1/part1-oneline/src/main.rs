use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!()
    }
    let reader = BufReader::new(File::open(&args[1])?);
    let mut elfs: Vec<u32> = vec![];
    let mut cur: u32 = 0;

    //START
    reader.lines().flatten().for_each(|line| {if line.is_empty() {elfs.push(cur); cur = 0;} else {cur += line.parse::<u32>().unwrap();}});
    //END 

    println!("Max: {}", elfs.iter().max().unwrap());
    Ok(())
}