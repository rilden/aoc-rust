use std::{fs, process};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        eprintln!("Problem reading input.txt: {err}");
        process::exit(1);
    });
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    count_if(input, |a, b| a.contains(b) || b.contains(a))
}

fn part2(input: &str) -> u32 {
    count_if(input, |a, b| a.overlaps(b))
}

fn count_if(input: &str, f: fn(&Range, &Range) -> bool) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        if let Some((a, b)) = line.split_once(',') {
            let a = Range::from(a);
            let b = Range::from(b);
            if f(&a, &b) {
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug, PartialEq)]
struct Range {
    lower: u8,
    upper: u8,
}

impl Range {
    fn from(s: &str) -> Self {
        if let Some((a, b)) = s.split_once('-') {
            if let (Ok(a), Ok(b)) = (a.parse(), b.parse()) {
                Range { lower: a, upper: b }
            } else {
                eprintln!("Error parsing from {}", s);
                process::exit(1);
            }
        } else {
            eprintln!("Error parsing Range from {}", s);
            process::exit(1);
        }
    }

    fn contains(&self, b: &Self) -> bool {
        self.lower <= b.lower && self.upper >= b.upper
    }

    fn overlaps(&self, b: &Self) -> bool {
        b.lower <= self.upper && b.upper >= self.lower
    }
}

// fn part2(input: &str) -> u32 {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }

    #[test]
    fn test_range_from() {
        assert_eq!(
            Range::from("13-42"),
            Range {
                lower: 13,
                upper: 42
            }
        );
    }

    #[test]
    fn test_range_contains() {
        assert!(Range::from("11-42").contains(&Range::from("11-13")));
        assert!(!Range::from("11-42").contains(&Range::from("1-13")));
    }

    #[test]
    fn test_range_overlaps() {
        assert!(Range::from("11-42").overlaps(&Range::from("11-42")));
        assert!(Range::from("11-42").overlaps(&Range::from("1-13")));
        assert!(Range::from("7-32").overlaps(&Range::from("11-42")));
        assert!(Range::from("11-42").overlaps(&Range::from("12-40")));
    }
}
