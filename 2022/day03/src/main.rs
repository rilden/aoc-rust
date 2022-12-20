use std::{collections::HashSet, fs, process};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        eprintln!("Problem reading input.txt: {err}");
        process::exit(1);
    });
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut total: u32 = 0;

    for line in input.lines() {
        let n = line.as_bytes().len();
        let set1: HashSet<char> = line[0..n / 2].chars().collect();
        let set2: HashSet<char> = line[n / 2..n].chars().collect();
        let &common = set1.intersection(&set2).next().unwrap();
        total += priority(common);
    }
    total
}

fn part2(input: &str) -> u32 {
    let mut total = 0;
    let mut lines = input.lines();
    while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
        let a: HashSet<char> = a.chars().collect();
        let b: HashSet<char> = b.chars().collect();
        let c: HashSet<char> = c.chars().collect();
        let a: HashSet<char> = a.intersection(&b).cloned().collect();
        let &common = a.intersection(&c).next().unwrap();
        total += priority(common);
    }
    total
}

fn priority(ch: char) -> u32 {
    match ch {
        'A'..='Z' => ch as u32 - 'A' as u32 + 27,
        'a'..='z' => ch as u32 - 'a' as u32 + 1,
        c => panic!("character {c} is not a letter"),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 70);
    }
}
