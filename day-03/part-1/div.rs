use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn process_line(line: &str) -> isize {
    let bytes = line.as_bytes();
    let mut max: u8 = 0;
    let mut maybe_second_max: Option<u8> = None;

    for (i, b) in bytes.iter().enumerate() {
        let digit = b - b'0';
            if digit > max && i < bytes.len()-1 {
                max = digit;
                maybe_second_max = None;
                continue
            }
            if maybe_second_max.is_none_or(| curr| { digit > curr } )  {
                maybe_second_max = Some(digit);
            }
    }
    (max as isize) * 10 + maybe_second_max.unwrap() as isize
}
fn run(input: &str) -> isize {
    input.split('\n').map(process_line).sum::<isize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Test example"), 0)
    }

    #[test]
    fn process_line_test() {
        assert_eq!(process_line("987654321111111"), 98);
        assert_eq!(process_line("811111111111119"), 89);
        assert_eq!(process_line("234234234234278"), 78);
        assert_eq!(process_line("818181911112111"), 92);
    }
}
