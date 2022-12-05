use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("No input file specified!");
        std::process::exit(0);
    }

    let mut lines = BufReader::new(File::open(&args[1])?)
        .lines().map(|l| l.unwrap());

    let mut line: String;
    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut crate_data = String::new();


    //-=-=-=-=-=-=- PARSE CRATES -=-=-=-=-=-=-=-
    let indexes: Vec<u8> = vec![1, 5, 9, 13, 17, 21, 25, 29, 33];
    for _ in 0..9 {
        crates.push(Vec::new());
    }

    loop {
        line = lines.next().unwrap();
        if line.starts_with(' ') {
            lines.next();
            break;
        }
        crate_data.push_str(&(line +"\n"));
    }
    
    crate_data = crate_data.lines().rev().collect::<Vec<&str>>().join("\n");

    let mut push_crates = |c: char, i: usize| {
        if c != ' ' {
            crates[i].push(c);
        }
    };
    
    for l in crate_data.lines() {
        for i in 0..9 {
            push_crates(l.chars().nth(indexes[i] as usize).unwrap(), i);
        }
    }
    //-=-=-=-=-=-=- END PARSE CRATES -=-=-=-=-=-=-=-

    //-=-=-=-=-=-=- PARSE INSTRUCTIONS -=-=-=-=-=-=-=-

    while let Some(l) = lines.next() {
        let split = l.split_whitespace().collect::<Vec<&str>>();
        let amount = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let to_append = crates[from].pop().unwrap();
            crates[to].push(to_append);
        }
    }
    //-=-=-=-=-=-=- END PARSE INSTRUCTIONS -=-=-=-=-=-=-=-

    println!("Final state:");
    for i in 0..9 {
        println!("{}: {:?}", i+1, crates[i]);
    }


    Ok(())
}