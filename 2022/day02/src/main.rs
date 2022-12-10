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
    let mut total: i64 = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let opponent = to_shape(chars[0], "ABC");
        let you = to_shape(chars[2], "XYZ");
        total += outcome_score(&opponent, &you) + shape_score(&you);
    }
    total
}

fn part2(input: &str) -> i64 {
    let mut total: i64 = 0;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let opponent = to_shape(chars[0], "ABC");
        let outcome = match chars[2] {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Error converting {} to score", chars[2]),
        };
        let you = outcome_to_shape(&opponent, outcome);
        total += outcome_score(&opponent, &you) + shape_score(&you);
    }
    total
}

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn to_shape(c: char, mapping_str: &str) -> Shape {
    let mapping: Vec<char> = mapping_str.chars().collect();
    match c {
        x if x == mapping[0] => Shape::Rock,
        x if x == mapping[1] => Shape::Paper,
        x if x == mapping[2] => Shape::Scissors,
        _ => panic!("Error converting char {c} to shape using mapping {mapping_str}"),
    }
}

fn shape_score(s: &Shape) -> i64 {
    match s {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn outcome_to_shape(opponent: &Shape, outcome: i64) -> Shape {
    let mut result = Shape::Rock;
    for shape in [Shape::Rock, Shape::Paper, Shape::Scissors].iter() {
        if outcome_score(opponent, &shape) == outcome {
            result = shape.clone();
            break;
        }
    }
    result
}

fn outcome_score(opponent: &Shape, you: &Shape) -> i64 {
    match (opponent, you) {
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => 6,
        (x, y) if x == y => 3,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "\
A Y
B X
C Z
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 12);
    }
}
