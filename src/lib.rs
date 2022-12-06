use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

pub fn day1_part1(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .split("\n\n")
        .map(|f| f.lines().map(|f| f.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn day1_part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let mut nums = input
        .split("\n\n")
        .map(|f| f.lines().map(|f| f.parse::<u64>().unwrap()).sum())
        .collect::<Vec<u64>>();
    nums.sort();
    nums.reverse();
    nums[0..3].iter().sum::<u64>()
}

#[test]
fn day1_test() {
    assert_eq!(69693, day1_part1("inputs/1.txt"));
    assert_eq!(200945, day1_part2("inputs/1.txt"));
}

enum RPS {
    Rock,
    Paper,
    Scissors,
}

pub fn day2_part1(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .lines()
        .map(|f| {
            let opp = day2_opponent_code(f.chars().next().unwrap());
            let pla = day2_player_code(f.chars().nth(2).unwrap());
            day2_round_score(&opp, &pla)
        })
        .sum()
}

fn day2_opponent_code(chr: char) -> RPS {
    match chr {
        'A' => RPS::Rock,
        'B' => RPS::Paper,
        'C' => RPS::Scissors,
        _ => panic!(),
    }
}

fn day2_player_code(chr: char) -> RPS {
    match chr {
        'X' => RPS::Rock,
        'Y' => RPS::Paper,
        'Z' => RPS::Scissors,
        _ => panic!(),
    }
}

fn day2_round_score(opp: &RPS, pla: &RPS) -> u64 {
    let shape_score = match pla {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
    let outcome_score = match (opp, pla) {
        (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => 6,
        (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => 3,
        _ => 0,
    };
    shape_score + outcome_score
}

pub fn day2_part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .lines()
        .map(|f| {
            let opp = day2_opponent_code(f.chars().next().unwrap());
            match f.chars().nth(2).unwrap() {
                // lose
                'X' => {
                    /*0 + */
                    match opp {
                        RPS::Rock => 3,
                        RPS::Paper => 1,
                        RPS::Scissors => 2,
                    }
                }
                // draw
                'Y' => {
                    3 + match opp {
                        RPS::Rock => 1,
                        RPS::Paper => 2,
                        RPS::Scissors => 3,
                    }
                }
                // win
                'Z' => {
                    6 + match opp {
                        RPS::Rock => 2,
                        RPS::Paper => 3,
                        RPS::Scissors => 1,
                    }
                }
                _ => panic!(),
            }
        })
        .sum()
}

#[test]
fn day2_test() {
    assert_eq!(11449, day2_part1("inputs/2.txt"));
    assert_eq!(13187, day2_part2("inputs/2.txt"));
}

pub fn day3_part1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .lines()
        .map(|l| {
            let len = l.len();
            let fst = &l[0..len / 2].chars().collect::<HashSet<char>>();
            let snd = &l[len / 2..].chars().collect::<HashSet<char>>();
            let chr = *fst.intersection(snd).next().unwrap() as u32;
            if 'a' as u32 <= chr && 'z' as u32 >= chr {
                chr - ('a' as u32) + 1
            } else {
                chr - ('A' as u32) + 27
            }
        })
        .sum()
}

pub fn day3_part2(filename: &str) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|l| {
            let fst = &l[0].chars().collect::<HashSet<char>>();
            let snd = &l[1].chars().collect::<HashSet<char>>();
            let thr = &l[2].chars().collect::<HashSet<char>>();
            let chr = *fst
                .intersection(snd)
                .copied()
                .collect::<HashSet<char>>()
                .intersection(thr)
                .next()
                .unwrap() as u32;
            if 'a' as u32 <= chr && 'z' as u32 >= chr {
                chr - ('a' as u32) + 1
            } else {
                chr - ('A' as u32) + 27
            }
        })
        .sum()
}

#[test]
fn day3_test() {
    assert_eq!(7850, day3_part1("inputs/3.txt"));
    assert_eq!(2581, day3_part2("inputs/3.txt"));
}

pub fn day4_part1(filename: &str) -> usize {
    let re = Regex::new(r"(?P<fst>\d+)-(?P<snd>\d+),(?P<thr>\d+)-(?P<fth>\d+)").unwrap();
    let input = fs::read_to_string(filename).unwrap();
    input
        .lines()
        .filter(|&l| {
            let caps = re.captures(l).unwrap();
            let fst = caps["fst"].parse::<u32>().unwrap();
            let snd = caps["snd"].parse::<u32>().unwrap();
            let thr = caps["thr"].parse::<u32>().unwrap();
            let fth = caps["fth"].parse::<u32>().unwrap();
            (fst >= thr && snd <= fth) || (thr >= fst && fth <= snd)
        })
        .count()
}

pub fn day4_part2(filename: &str) -> usize {
    let re = Regex::new(r"(?P<fst>\d+)-(?P<snd>\d+),(?P<thr>\d+)-(?P<fth>\d+)").unwrap();
    let input = fs::read_to_string(filename).unwrap();
    input
        .lines()
        .filter(|&l| {
            let caps = re.captures(l).unwrap();
            let fst = caps["fst"].parse::<u32>().unwrap();
            let snd = caps["snd"].parse::<u32>().unwrap();
            let thr = caps["thr"].parse::<u32>().unwrap();
            let fth = caps["fth"].parse::<u32>().unwrap();
            (fst >= thr && snd <= fth)
                || (thr >= fst && fth <= snd)
                || (fst >= thr && fst <= fth)
                || (snd >= thr && snd <= fth)
        })
        .count()
}

#[test]
fn day4_test() {
    assert_eq!(471, day4_part1("inputs/4.txt"));
    assert_eq!(888, day4_part2("inputs/4.txt"));
}

pub fn day5_part1(filename: &str) -> String {
    let re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let v = input.split("\n\n").collect::<Vec<&str>>();
    let first_part = v[0];
    let second_part = v[1];
    let stacks_num = first_part
        .lines()
        .rev()
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..stacks_num {
        stacks.push(Vec::new());
    }
    for line in first_part.lines().rev().skip(1) {
        for (i, _) in line.match_indices('[') {
            let chr = line.chars().nth(i + 1).unwrap();
            stacks[i / 4].push(chr);
        }
    }

    for line in second_part.lines() {
        let caps = re.captures(line).unwrap();
        let count = caps["count"].parse::<u32>().unwrap();
        let from = caps["from"].parse::<usize>().unwrap();
        let to = caps["to"].parse::<usize>().unwrap();
        for _ in 0..count {
            let temp = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(temp);
        }
    }

    stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>()
}

pub fn day5_part2(filename: &str) -> String {
    let re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let v = input.split("\n\n").collect::<Vec<&str>>();
    let first_part = v[0];
    let second_part = v[1];
    let stacks_num = first_part
        .lines()
        .rev()
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..stacks_num {
        stacks.push(Vec::new());
    }
    for line in first_part.lines().rev().skip(1) {
        for (i, _) in line.match_indices('[') {
            let chr = line.chars().nth(i + 1).unwrap();
            stacks[i / 4].push(chr);
        }
    }

    for line in second_part.lines() {
        let caps = re.captures(line).unwrap();
        let count = caps["count"].parse::<u32>().unwrap();
        let from = caps["from"].parse::<usize>().unwrap();
        let to = caps["to"].parse::<usize>().unwrap();
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..count {
            temp.push(stacks[from - 1].pop().unwrap());
        }
        for chr in temp.iter().rev() {
            stacks[to - 1].push(*chr);
        }
    }

    stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>()
}

#[test]
fn day5_test() {
    assert_eq!("HNSNMTLHQ", day5_part1("inputs/5.txt"));
    assert_eq!("RNLFDJMCT", day5_part2("inputs/5.txt"));
}

pub fn day6_part1(filename: &str) -> u32 {
    let input = fs::read_to_string(filename)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let mut buffer: VecDeque<char> = VecDeque::from([input[0], input[1], input[2], input[3]]);
    let mut input_to_process = input.iter().skip(4).rev().copied().collect::<Vec<char>>();
    let mut i = 4;

    while buffer.iter().collect::<HashSet<&char>>().len() < 4 {
        buffer.pop_front();
        buffer.push_back(input_to_process.pop().unwrap());
        i += 1;
    }
    i
}

pub fn day6_part2(filename: &str) -> u32 {
    let input = fs::read_to_string(filename)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let mut buffer: VecDeque<char> = VecDeque::new();
    for i in &input[0..14] {
        buffer.push_back(*i);
    }
    let mut input_to_process = input.iter().skip(14).rev().copied().collect::<Vec<char>>();
    let mut i = 14;

    while buffer.iter().collect::<HashSet<&char>>().len() < 14 {
        buffer.pop_front();
        buffer.push_back(input_to_process.pop().unwrap());
        i += 1;
    }
    i
}

#[test]
fn day6_test() {
    assert_eq!(1929, day6_part1("inputs/6.txt"));
    assert_eq!(3298, day6_part2("inputs/6.txt"));
}
