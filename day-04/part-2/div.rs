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

fn run_once(grid: &mut Vec<Vec<i32>>) -> isize {
    let (width, height) = (grid[0].len(), grid.len());

    let mut removable: Vec<(usize, usize)> = vec![];

    for x in 1..width - 1 {
        for y in 1..height - 1 {
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
                removable.push((y, x));
            }
        }
    }

    let count = removable.len();
    for (y, x) in removable.into_iter() {
        grid[y][x] = 0;
    }
    count.try_into().unwrap()
}

fn build_grid(input: &str) -> Vec<Vec<i32>> {
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
    grid
}

fn run(input: &str) -> isize {
    let mut grid = build_grid(input);

    let mut count = 0;
    loop {
        let count_once = run_once(&mut grid);
        if count_once == 0 {
            return count;
        }
        count += count_once;
    }
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
