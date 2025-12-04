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
const REMOVED: u8 = 17;

fn run(input: &str) -> isize {
    let grid = StrGrid::from_input(input);
    let mut neighbors_count = vec![0u8; grid.data.len()];
    let mut res = 0;
    // let mut changed = true;
    let mut cur = 0;
    let mut to_check: Vec<usize> = vec![];
    let mut local_neighbors = Vec::with_capacity(8);
    while cur < grid.data.len() {
        if grid.data[cur] == b'@' {
            local_neighbors.clear();
            let mut pos = grid.from_cur(cur);
            for dir in LOOPAROUND {
                if grid.step_mut(&mut pos, dir) {
                    let cur2 = grid.cur(pos);
                    if grid.data[cur2] == b'@' {
                        local_neighbors.push(cur2);
                    }
                }
            }
            if local_neighbors.len() < 4 {
                res += 1;
                for &cur2 in &local_neighbors {
                    neighbors_count[cur2] = neighbors_count[cur2].wrapping_sub(1);
                    if neighbors_count[cur2] <= 3 {
                        to_check.push(cur2);
                    }
                }
                neighbors_count[cur] = REMOVED;
            } else {
                neighbors_count[cur] =
                    neighbors_count[cur].wrapping_add(local_neighbors.len() as u8);
                if neighbors_count[cur] <= 3 {
                    to_check.push(cur);
                }
            }
        }
        cur += 1;
    }
    while let Some(cur) = to_check.pop() {
        if let 0..=3 = neighbors_count[cur] {
            local_neighbors.clear();
            let mut pos = grid.from_cur(cur);
            for dir in LOOPAROUND {
                if grid.step_mut(&mut pos, dir) {
                    let cur2 = grid.cur(pos);
                    if neighbors_count[cur2] <= 8 {
                        local_neighbors.push(cur2);
                    }
                }
            }
            res += 1;
            neighbors_count[cur] = REMOVED;
            for &cur2 in &local_neighbors {
                neighbors_count[cur2] = neighbors_count[cur2].wrapping_sub(1);
                if neighbors_count[cur2] <= 3 {
                    to_check.push(cur2);
                }
            }
        }
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
            43
        )
    }
}
