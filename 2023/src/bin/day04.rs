use std::fs::read_to_string;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_NUMBERS: Regex = Regex::new(r"\d+").unwrap();
}

fn parse_line(mut line: &str) -> (Vec<u32>, Vec<u32>) {
    line = &line[(line.find(':').unwrap() + 1)..];
    let mut series_iter = line.split('|');
    let mut parse_next_series = || -> Vec<u32> {
        let nums_str = series_iter.next().unwrap();
        RE_NUMBERS.find_iter(nums_str)
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect()
    };
    let winning: Vec<u32> = parse_next_series();
    let scratched: Vec<u32> = parse_next_series();
    (winning, scratched)
}

fn compute_score(winning: Vec<u32>, scratched: Vec<u32>) -> u32 {
    let mut ret = 0;
    for num in scratched {
        if winning.contains(&num) {
            ret = if ret == 0 { 1 } else { ret * 2 };
        }
    }
    ret
}

fn part1(fpath: &str) -> u32 {
    read_to_string(fpath)
        .unwrap()
        .lines()
        .map(parse_line)
        .map(|x| compute_score(x.0, x.1))
        .sum()
}

fn part2(fpath: &str) -> u32 {
    let text = read_to_string(fpath).unwrap();
    let lines: Vec<&str> = text.lines().collect();

    let mut counts = vec![1u32; lines.len()];

    for i in 0..lines.len() {
        let line = lines[i];
        let (winning, scratched) = parse_line(line);
        let win_count = scratched.iter()
            .filter(|x| winning.contains(&x))
            .count();
        for j in (i + 1)..(i + 1 + win_count) {
            counts[j] += counts[i];
        }
    }

    counts.iter().sum()
}

fn main() {
    const INPUT: &str = "data/day04.txt";

    let part1_result = part1(INPUT);
    println!("[PART1] Final sum: {}", part1_result);
    let part2_result = part2(INPUT);
    println!("[PART2] Final sum: {}", part2_result);
}

#[test]
fn test_part1() {
    assert_eq!(13, part1("data/day04_example.txt"));
}

#[test]
fn test_part2() {
    assert_eq!(30, part2("data/day04_example.txt"));
}
