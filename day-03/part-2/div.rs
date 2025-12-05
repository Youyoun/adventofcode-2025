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
    let len = bytes.len();
    let mut vals : Vec<isize>= vec![0;12];

    let mut left = 0;
    for k in 0..12 {
        // find the k-th digit
        // it's the leftmost max between the pos of the previous max and 12-k
        let (mut max_pos, mut max_val) = (0, 0);
        for i in left..(len-11+k) {
            let curr = bytes[i] - b'0';
            if curr > max_val {
                max_pos = i;
                max_val = curr;
            }
            if curr == 9 {
                break
            }
        }
        vals[k] = max_val as isize;
        left = max_pos+1;
    }
    
    // now build the number
    vals.into_iter().reduce(|acc, e| 10*acc + e).unwrap()
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
        assert_eq!(process_line("987654321111111"), 987654321111);
        assert_eq!(process_line("811111111111119"), 811111111119);
        assert_eq!(process_line("234234234234278"), 434234234278);
        assert_eq!(process_line("818181911112111"), 888911112111);
    }
}
