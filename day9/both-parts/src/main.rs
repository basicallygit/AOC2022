use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 1 {
        eprintln!("No input file specified.");
        std::process::exit(0);
    }

    let mut second_visited: HashSet<(isize, isize)> = HashSet::new();
    let mut tail_visited: HashSet<(isize, isize)> = HashSet::new();

    let mut positions = [(0_isize, 0_isize); 10];
    for line in read_to_string(&args[1]).unwrap().lines() {
        let mut iter = line.split_whitespace();

        let next_step = match iter.next().unwrap() {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => (0,0),
        };

        for _ in 0..iter.next().unwrap().parse::<usize>().unwrap() {
            let head = &mut positions[0];
            head.0 += next_step.0;
            head.1 += next_step.1;

            for i in 0..9 {
                let x = (positions[i].0 - positions[i + 1].0).signum();
                let y = (positions[i].1 - positions[i + 1].1).signum();
                if x * (positions[i].0 - positions[i + 1].0) > 1
                    || y * (positions[i].1 - positions[i + 1].1) > 1
                {
                    let knot = &mut positions[i + 1];
                    knot.0 += x;
                    knot.1 += y;
                }
            }

            second_visited.insert(positions[1]);
            tail_visited.insert(positions[9]);
        }
    }

    println!("P1: {}", second_visited.len());
    println!("P2: {}", tail_visited.len());

    Ok(())
}