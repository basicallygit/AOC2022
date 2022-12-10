use std::fs::read_to_string;
use std::env;
use std::iter::once;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("No input file specified");
        std::process::exit(1);
    }
    let input = read_to_string(&args[1])?;

    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut max_score = 0;
    for x in 0..map.len() {
        for y in 0..map.len() {
            let height = map[x][y];
            let mut score = 1;

            let directions: Vec<Vec<(usize, usize)>> = vec![
                (0..x).rev().zip(once(y).cycle()).collect(),
                ((x + 1)..map.len()).zip(once(y).cycle()).collect(),
                (once(x).cycle()).zip((0..y).rev()).collect(),
                (once(x).cycle()).zip((y + 1)..map.len()).collect(),
            ];
            for direction in directions.iter() {
                let mut trees = 0;
                for (dx, dy) in direction.iter() {
                    trees += 1;
                    if map[*dx][*dy] >= height {
                        break;
                    }
                }
                score *= trees;
            }

            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("{}", max_score);
    Ok(())
}
