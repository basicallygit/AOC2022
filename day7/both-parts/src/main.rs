use std::collections::HashMap;
use std::fs::read_to_string;
use std::env;
fn main() -> std::io::Result<()> {
    
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("No input file specified");
        std::process::exit(1);
    }
    let contents = read_to_string(&args[1])?;


    let mut dirs: HashMap<Vec<&str>, u64> = HashMap::from([(vec![], 0)]);
    let mut current_dir: Vec<&str> = vec![];

    for line in contents.lines() {
        if let Some(path) = line.strip_prefix("$ cd ") {
            match path.trim() {
                x if x == ".." => drop(current_dir.pop()),
                x if x == "/" => current_dir = vec![],
                x => current_dir.push(x),
            }
        }
        else if let Some((size, _)) = line.split_once(' ') {
            let Ok(size) = size.parse::<u64>() else { continue; };
            (0..=current_dir.len()).for_each(|i| *dirs.entry(current_dir[..i].to_vec()).or_default() += size);
        }
    }
    let lessthan100k: u64 = dirs.values().filter(|&&x| x <= 100000).sum();

    //credit to kolsky i had no idea about saturating_sub()
    let minimum = dirs[[].as_slice()].saturating_sub(70000000 - 30000000);
    let part2 = dirs.values().filter(|&&x| x >= minimum).min().unwrap_or(&0);


    println!("Part 1: {}", lessthan100k);
    println!("Part 2: {}", part2);
    Ok(())
}