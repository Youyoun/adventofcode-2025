use aoc::enizor::grid::*;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

use Direction::*;

const LOOPAROUND: [Direction; 8] = [Up, Right, Down, Down, Left, Left, Up, Up];

fn run(input: &str) -> isize {
    let grid = StrGrid::from_input(input);
    let mut cur = 0;
    let mut res = 0;
    while cur < grid.data.len() {
        if grid.data[cur] == b'@' {
            let mut pos = grid.from_cur(cur);
            let mut count = 0;
            for dir in LOOPAROUND {
                if grid.step_mut(&mut pos, dir) && grid.data[grid.cur(pos)] == b'@' {
                    count += 1;
                    if count >= 4 {
                        break;
                    }
                }
            }
            if count < 4 {
                res += 1;
            }
        }
        cur += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."),
            13
        )
    }
}
