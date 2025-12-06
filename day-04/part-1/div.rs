use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn compute_size(input: &str) -> (usize, usize) {
    let width = input.find('\n').unwrap();
    let height = input
        .strip_suffix('\n')
        .unwrap_or(input)
        .chars()
        .filter(|x| *x == '\n')
        .count();

    (width, height + 1)
}

fn run(input: &str) -> isize {
    let (width, height) = compute_size(input);
    let mut grid = vec![vec![0; height + 2]; width + 2];

    let (mut y, mut x): (usize, usize) = (1, 1);

    for c in input.chars() {
        match c {
            '.' => {
                grid[y][x] = 0;
                x += 1;
            }
            '@' => {
                grid[y][x] = 1;
                x += 1;
            }
            '\n' => {
                y += 1;
                x = 1;
            }
            _ => unreachable!("unknown character"),
        }
    }

    let mut count = 0;
    for x in 1..(width + 1) as usize {
        for y in 1..(height + 1) as usize {
            if grid[y][x] == 0 {
                continue;
            }
            let s = grid[y - 1][x - 1]
                + grid[y - 1][x]
                + grid[y - 1][x + 1]
                + grid[y][x - 1]
                + grid[y][x + 1]
                + grid[y + 1][x - 1]
                + grid[y + 1][x]
                + grid[y + 1][x + 1];

            if s < 4 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Test example"), 0)
    }

    #[test]
    fn compute_size_test() {
        {
            let (width, height) = compute_size("....\n....\n....");
            assert_eq!(width, 4);
            assert_eq!(height, 3);
        }
        {
            let (width, height) = compute_size("....\n....\n....\n");
            assert_eq!(width, 4);
            assert_eq!(height, 3);
        }
    }
}
