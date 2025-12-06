use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parses_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (part_one, part_two) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(usize, usize)> = part_one
        .split('\n')
        .into_iter()
        .map(|line| {
            let mut range_parts = line.split('-').into_iter();
            let left = range_parts.next().unwrap().parse::<usize>().unwrap();
            let right = range_parts.next().unwrap().parse::<usize>().unwrap();
            (left, right)
        })
        .collect();

    let ingredients = part_two
        .split('\n')
        .into_iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    (ranges, ingredients)
}

fn run(input: &str) -> isize {
    let (ranges, ingredients) = parses_input(input);

    ingredients
        .into_iter()
        .filter(|x| ranges.iter().any(|(start, end)| start <= x && x <= end))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Test example"), 0)
    }
}
