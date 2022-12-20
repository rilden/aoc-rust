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
    let (mut items, mut indexes) = parse_input(input, 1);

    mix(&mut items, &mut indexes);

    grove_coordinate(items)
}

fn part2(input: &str) -> i64 {
    let (mut items, mut indexes) = parse_input(input, 811589153i64);

    for _ in 0..10 {
        mix(&mut items, &mut indexes);
    }

    grove_coordinate(items)
}

#[derive(Debug, Clone, Copy)]
struct Item {
    value: i64,
    index: usize,
}

fn parse_input(input: &str, decryption_key: i64) -> (Vec<Item>, Vec<usize>) {
    let mut items = Vec::new();
    let mut indexes = Vec::new();
    for (index, line) in input.lines().enumerate() {
        items.push(Item {
            value: line.parse::<i64>().unwrap() * decryption_key,
            index,
        });
        indexes.push(index);
    }
    (items, indexes)
}

fn grove_coordinate(items: Vec<Item>) -> i64 {
    let index_zero = items.iter().position(|&x| x.value == 0).unwrap();
    let mut result = 0;
    for i in [1000, 2000, 3000] {
        result += items[(index_zero + i) % items.len()].value;
    }
    result
}

fn mix(items: &mut Vec<Item>, indexes: &mut [usize]) {
    let n = items.len();
    for i in 0..n {
        let index = indexes[i];
        let num_swaps = items[index].value.rem_euclid(n as i64 - 1) as usize;
        for j in index..(index + num_swaps) {
            let x = j % n;
            let y = (j + 1) % n;

            items.swap(x, y);
            indexes[items[x].index] = x;
            indexes[items[y].index] = y;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "\
1
2
-3
3
-2
0
4
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1623178306);
    }
}
