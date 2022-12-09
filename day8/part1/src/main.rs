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

    let mut visible: u32 = 0;
    for i in 0..map.len() {
        for j in 0..map.len() {
            let height = map[i][j];
            let directions: Vec<Vec<(usize, usize)>> = vec![
                (0..i).zip(once(j).cycle()).collect(),
                ((i + 1)..map.len()).zip(once(j).cycle()).collect(),
                (once(i).cycle()).zip(0..j).collect(),
                (once(i).cycle()).zip((j + 1)..map.len()).collect(),
            ];

            for direction in directions.iter() {
                let mut vis = true;
                for (d1, d2) in direction.iter() {
                    if map[*d1][*d2] >= height {
                        vis = false;
                        break;
                    }
                }
                if vis {
                    visible += 1;
                    break;
                }
            }
        }
    }

    println!("{}", visible);
    Ok(())
}
