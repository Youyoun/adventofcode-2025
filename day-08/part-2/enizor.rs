use std::env::args;
use std::time::Instant;

use aoc::enizor::parser::Parser;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const ROOT_BIT: u16 = 1 << 15;
const SIZE_MASK: u16 = !ROOT_BIT;
// A root of size 1
const DEFAULT_NODE: Node = Node(1 | ROOT_BIT);

// Either a root with ROOT_BIT set and a size on the other bits
// Or pointer to its parent
#[derive(Debug, Clone, Copy)]
struct Node(u16);

impl Node {
    fn is_root(&self) -> bool {
        self.0 & ROOT_BIT != 0
    }
    fn parent(&self) -> u16 {
        debug_assert!(!self.is_root());
        self.0
    }
    fn size(&self) -> u16 {
        debug_assert!(self.is_root());
        self.0 & SIZE_MASK
    }
}

struct DisjointSetForest {
    nodes: Vec<Node>,
}

impl DisjointSetForest {
    fn new(n: usize) -> Self {
        assert!((n as u16) < ROOT_BIT);
        Self {
            nodes: vec![DEFAULT_NODE; n],
        }
    }
    fn find_root(&mut self, mut n: u16) -> u16 {
        loop {
            let node = self.nodes[n as usize];
            if node.is_root() {
                return n;
            } else {
                let parent_node = self.nodes[node.parent() as usize];
                if parent_node.is_root() {
                    return node.parent();
                } else {
                    self.nodes[n as usize] = parent_node;
                    n = parent_node.parent();
                }
            }
        }
    }
    fn merge(&mut self, lhs: u16, rhs: u16) -> u16 {
        let l_root = self.find_root(lhs);
        let r_root = self.find_root(rhs);
        if l_root != r_root {
            let size = self.nodes[r_root as usize].size();
            self.nodes[r_root as usize].0 = l_root;
            self.nodes[l_root as usize].0 += size;
        }
        self.nodes[l_root as usize].size()
    }
}

fn run(input: &str) -> usize {
    let mut parser = Parser::from_input(&input);
    let mut points = vec![];
    while let Some(x) = parser.parse_usize() {
        assert!(x <= 1 << 20, "Cannot handle number {} > 2^20", x);
        parser.cur += 1;
        let y = parser
            .parse_usize()
            .unwrap_or_else(|| panic!("failed to parse near {}", parser.cur));
        parser.cur += 1;
        assert!(y <= 1 << 20, "Cannot handle number {} > 2^20", y);
        let z = parser
            .parse_usize()
            .unwrap_or_else(|| panic!("failed to parse near {}", parser.cur));
        assert!(z <= 1 << 20, "Cannot handle number {} > 2^20", x);
        points.push((x as f64, y as f64, z as f64));
        parser.cur += 1;
    }

    let mut dists = Vec::with_capacity(points.len() * points.len());
    for (i, &(x1, y1, z1)) in points.iter().enumerate() {
        for (j, &(x2, y2, z2)) in points.iter().enumerate().skip(i + 1) {
            let dx = x2 - x1; // <= 2^20 by hypothesis
            let dy = y2 - y1;
            let dz = z2 - z1;
            // d^2 <= 3×2^40 < 2^41
            // => min difference between d and d' is sqrt(2^41)-sqrt(2^41-1) ~ 1.4×2^-22
            // => 24 bits of precision (f32) is enough to accurately distinguish all distances
            let d = (dx * dx + dy * dy + dz * dz).sqrt() as f32;
            // use to_bits as valid, positive floats can be compared by transmuting their bits to u32
            dists.push((d.to_bits(), i as u16, j as u16));
        }
    }
    dists.sort_unstable_by(|(d1, _, _), (d2, _, _)| d1.cmp(d2));

    let mut circuits = DisjointSetForest::new(points.len());
    for &(_, i, j) in &dists {
        if circuits.merge(i, j) as usize == points.len() {
            return (points[i as usize].0 * points[j as usize].0) as usize;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_enizor_day8() {
        assert_eq!(
            run("162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"),
            25272
        )
    }
}
