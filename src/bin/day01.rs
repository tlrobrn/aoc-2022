#![warn(clippy::all, clippy::pedantic)]
use aoc2022::input_lines;

fn main() {
    let input: Vec<Vec<u64>> = parse(input_lines());
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse<I>(lines: I) -> Vec<Vec<u64>>
where
    I: Iterator<Item = String>,
{
    let mut packs = vec![vec![]];

    for line in lines {
        if let Ok(result) = line.trim().parse::<u64>() {
            let i = packs.len() - 1;
            packs[i].push(result);
        } else {
            packs.push(vec![]);
        }
    }
    packs
}

fn part1(a: &[Vec<u64>]) -> u64 {
    a.iter().map(|pack| pack.iter().sum()).max().unwrap()
}

fn part2(a: &[Vec<u64>]) -> u64 {
    let mut packs = a.iter().map(|pack| pack.iter().sum()).collect::<Vec<u64>>();
    packs.sort_unstable();
    packs.iter().rev().take(3).sum()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    fn example_input() -> Vec<Vec<u64>> {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    }

    #[test]
    fn part1_example() {
        let result = part1(&example_input());
        assert_eq!(result, 24000);
    }

    #[test]
    fn part2_example() {
        let result = part2(&example_input());
        assert_eq!(result, 45000);
    }
}
