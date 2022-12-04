use regex::Regex;
use std::fs;
use std::collections::HashSet;

pub fn day1_part1(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    input.split("\n\n")
         .map(|f| f.lines()
              .map(|f| f.parse::<u64>().unwrap()).sum())
        .max().unwrap()
}

pub fn day1_part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    let mut nums = input.split("\n\n")
         .map(|f| f.lines()
              .map(|f| f.parse::<u64>().unwrap()).sum()).collect::<Vec<u64>>();
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
    ROCK,
    PAPER,
    SCISSORS
}

pub fn day2_part1(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    input.lines()
         .map(|f| {
             let opp = day2_opponent_code(f.chars().nth(0).unwrap());
             let pla = day2_player_code(f.chars().nth(2).unwrap());
             day2_round_score(&opp, &pla)
         })
        .sum()
}

fn day2_opponent_code(chr: char) -> RPS {
    match chr {
        'A' => RPS::ROCK,
        'B' => RPS::PAPER,
        'C' => RPS::SCISSORS,
        _ => panic!()
    }
}

fn day2_player_code(chr: char) -> RPS {
    match chr {
        'X' => RPS::ROCK,
        'Y' => RPS::PAPER,
        'Z' => RPS::SCISSORS,
        _ => panic!()
    }
}

fn day2_round_score(opp: &RPS, pla: &RPS) -> u64 {
    let shape_score = match pla {
        RPS::ROCK => 1,
        RPS::PAPER => 2,
        RPS::SCISSORS => 3,
    };
    let outcome_score = match (opp, pla) {
        (RPS::ROCK, RPS::PAPER) | (RPS::PAPER, RPS::SCISSORS) | (RPS::SCISSORS, RPS::ROCK) => 6,
        (RPS::ROCK, RPS::ROCK) | (RPS::PAPER, RPS::PAPER) | (RPS::SCISSORS, RPS::SCISSORS)  => 3,
        _ => 0
    };
    shape_score + outcome_score
}

pub fn day2_part2(filename: &str) -> u64 {
    let input = fs::read_to_string(filename).unwrap();
    input.lines()
         .map(|f| {
             let opp = day2_opponent_code(f.chars().nth(0).unwrap());
             match f.chars().nth(2).unwrap() {
                 // lose
                 'X' => 0 + match opp {
                     RPS::ROCK => 3,
                     RPS::PAPER => 1,
                     RPS::SCISSORS => 2,
                 },
                 // draw
                 'Y' => 3 + match opp {
                     RPS::ROCK => 1,
                     RPS::PAPER => 2,
                     RPS::SCISSORS => 3,
                 },
                 // win
                 'Z' => 6 + match opp {
                     RPS::ROCK => 2,
                     RPS::PAPER => 3,
                     RPS::SCISSORS => 1,
                 },
                 _ => panic!()
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
    input.lines()
         .map(|l| {
             let len = l.len();
             let fst = &l[0..len/2].chars().collect::<HashSet<char>>();
             let snd = &l[len/2..].chars().collect::<HashSet<char>>();
             let chr = fst.intersection(snd).next().unwrap().clone() as u32;
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
    input.lines().collect::<Vec<&str>>().chunks(3)
         .map(|l| {
             let fst = &l[0].chars().collect::<HashSet<char>>();
             let snd = &l[1].chars().collect::<HashSet<char>>();
             let thr = &l[2].chars().collect::<HashSet<char>>();
             let chr = fst.intersection(snd).map(|f|f.clone()).collect::<HashSet<char>>().intersection(thr).next().unwrap().clone() as u32;
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
    input.lines()
         .filter(|&l| {
             let caps = re.captures(l).unwrap();
             let fst = *(&caps["fst"].parse::<u32>().unwrap());
             let snd = *(&caps["snd"].parse::<u32>().unwrap());
             let thr = *(&caps["thr"].parse::<u32>().unwrap());
             let fth = *(&caps["fth"].parse::<u32>().unwrap());
             (fst >= thr && snd <= fth) || (thr >= fst && fth <= snd)
         })
         .count()
}

pub fn day4_part2(filename: &str) -> usize {
    let re = Regex::new(r"(?P<fst>\d+)-(?P<snd>\d+),(?P<thr>\d+)-(?P<fth>\d+)").unwrap();
    let input = fs::read_to_string(filename).unwrap();
    input.lines()
         .filter(|&l| {
             let caps = re.captures(l).unwrap();
             let fst = *(&caps["fst"].parse::<u32>().unwrap());
             let snd = *(&caps["snd"].parse::<u32>().unwrap());
             let thr = *(&caps["thr"].parse::<u32>().unwrap());
             let fth = *(&caps["fth"].parse::<u32>().unwrap());
             (fst >= thr && snd <= fth) || (thr >= fst && fth <= snd) || (fst >= thr && fst <= fth) || (snd >= thr && snd <= fth)
         })
         .count()
}

#[test]
fn day4_test() {
    assert_eq!(471, day4_part1("inputs/4.txt"));
    assert_eq!(888, day4_part2("inputs/4.txt"));
}
