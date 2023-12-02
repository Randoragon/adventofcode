use std::fs::read_to_string;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref DIGIT_NAMES: HashMap<&'static str, u8> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
}

// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines(fpath: &str) -> Vec<String> {
    read_to_string(fpath)
        .unwrap()
         .lines()
        .map(String::from)
        .collect()
}

fn parse_line(line: &str) -> u32 {
    lazy_static! {
        static ref RE_FIRST_AND_LAST_DIGIT: Regex = Regex::new(r"(?<first>\d)(.*(?<last>\d))?").unwrap();
    }

    let caps = RE_FIRST_AND_LAST_DIGIT.captures(line).unwrap();
    let mut str = caps.name("first").unwrap().as_str().to_string();

    match caps.name("last") {
        Some(x) => str.push_str(x.as_str()),
        None => str.push_str(&str.clone()),
    };
    str.parse::<u32>().unwrap()
}

fn str_digit(s: &str) -> Option<u8> {
    if s.is_empty() {
        return None;
    }
    let first_char = s.chars().nth(0).unwrap();
    for i in 1u8..=9u8 {
        if first_char == (('1' as u8) - 1 + i) as char {
            return Some(i);
        }
    }
    if let Some((_, v)) = DIGIT_NAMES.iter().find(|(k, _)| s.starts_with(*k)) {
        return Some(*v);
    }
    None
}

fn find_first_digit(line: &str) -> u8 {
    for i in 0usize..line.len() {
        if let Some(x) = str_digit(&line[i..]) {
            return x;
        }
    }
    panic!("no digits in line");
}

fn find_last_digit(line: &str) -> u8 {
    for i in (0usize..line.len()).rev() {
        if let Some(x) = str_digit(&line[i..]) {
            return x;
        }
    }
    panic!("no digits in line");
}

fn part1(fpath: &str) -> u32 {
    read_lines(fpath)
        .iter()
        .map(|x| parse_line(&x))
        .sum()
}

fn part2(fpath: &str) -> u32 {
    read_lines(fpath)
        .iter()
        .map(|x| {
            let mut ret = find_first_digit(x).to_string();
            ret.push_str(&find_last_digit(x).to_string());
            ret.parse::<u32>().unwrap()
        })
        .sum()
}

fn main() {
    const INPUT: &str = "data/day01.txt";

    let part1_result = part1(INPUT);
    println!("[PART1] Final sum: {}", part1_result);

    let part2_result = part2(INPUT);
    println!("[PART2] Final sum: {}", part2_result);
}


#[test]
fn test_part1_example() {
    assert_eq!(part1("data/day01_example1.txt"), 142);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2("data/day01_example2.txt"), 281);
}

#[test]
fn test_str_digit() {
    assert_eq!(str_digit(""), None);
    assert_eq!(str_digit("abc"), None);
    assert_eq!(str_digit("1bc"), Some(1u8));
    assert_eq!(str_digit("two"), Some(2u8));
    assert_eq!(str_digit("8two"), Some(8u8));
    assert_eq!(str_digit("threeight"), Some(3u8));
    assert_eq!(str_digit("seven9askljthes"), Some(7u8));
}

// ABANDONED IMPLEMENTATION (keeping for historical/educational reasons)
// ---------------------------------------------------------------------
// /// Replaces `"one"`, `"two"`, ... substrings in `s` with `"1"`, `"2"`, ... .
// /// Works in-place, but also returns the input for convenience.
// fn words2digits(s: &mut String) -> &mut String {
//     lazy_static! {
//         static ref RE_DIGIT_WORD: Regex = Regex::new(r"((one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))").unwrap();
//     }

//     let mut idx: usize = 0;
//     while let Some(caps) = RE_DIGIT_WORD.captures(&s[idx..]) {
//         let digit = (1usize..=9usize)
//             .find(|&i| caps.get(i).is_some())
//             .unwrap();
//         let mat = caps.get(digit).unwrap();
//         s.replace_range(idx..=(idx + mat.len()), &digit.to_string());
//         idx += mat.start() + 1;  // +1 to skip the digit that the match was changed into
//     }
//     s
// }
