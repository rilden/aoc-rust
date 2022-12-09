use std::{fs, process};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        eprintln!("Problem reading input.txt: {err}");
        process::exit(1);
    });
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    sum_of_calories(input).max().unwrap()
}

fn part2(input: &str) -> i64 {
    let mut v: Vec<i64> = sum_of_calories(input).collect();
    v.sort();
    v.iter().rev().take(3).sum()
}

fn sum_of_calories<'a>(text: &'a str) -> impl Iterator<Item = i64> + 'a {
    text.split("\n\n").map(|calories| {
        calories
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 45000);
    }
}
