use regex::Regex;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::HashMap;
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

pub fn day7_part1(filename: &str) -> u64 {
    let binding = fs::read_to_string(filename).unwrap();
    let mut input = binding.lines().skip(1).collect::<Vec<&str>>();
    let tree = day7_create_tree("/", &mut input);
    day7_get_dir_sizes(&tree)
        .iter()
        .filter(|&s| *s <= 100000)
        .sum()
}

pub fn day7_part2(filename: &str) -> u64 {
    let binding = fs::read_to_string(filename).unwrap();
    let mut input = binding.lines().skip(1).collect::<Vec<&str>>();
    let tree = day7_create_tree("/", &mut input);
    let total_diskspace = 70000000;
    let total_used_space = day7_get_node_size(&tree);
    let current_unused_space = total_diskspace - total_used_space;
    let space_required_to_free = 30000000 - current_unused_space;
    *day7_get_dir_sizes(&tree)
        .iter()
        .filter(|&s| *s >= space_required_to_free)
        .min()
        .unwrap()
}

#[derive(Debug)]
enum TreeNode {
    File(String, u64),
    Directory(String, Vec<TreeNode>),
}

fn day7_create_tree(dirname: &str, inp_lines: &mut Vec<&str>) -> TreeNode {
    let mut v_tree: Vec<TreeNode> = Vec::new();
    while !inp_lines.is_empty() {
        let line = inp_lines.remove(0);
        if line == "$ ls" {
            while !inp_lines.is_empty() && !inp_lines[0].starts_with('$') {
                let sub_line = inp_lines.remove(0);
                if sub_line.starts_with('d') {
                    continue;
                }
                let size = sub_line.split(' ').next().unwrap().parse::<u64>().unwrap();
                let filename = sub_line.split(' ').nth(1).unwrap();
                v_tree.push(TreeNode::File(filename.to_string(), size));
            }
        } else if line == "$ cd .." {
            break;
        } else {
            let new_dirname = line.split(' ').nth(2).unwrap();
            v_tree.push(day7_create_tree(new_dirname, inp_lines));
        }
    }
    TreeNode::Directory(String::from(dirname), v_tree)
}

fn day7_get_dir_sizes(node: &TreeNode) -> Vec<u64> {
    match node {
        TreeNode::File(_, _) => vec![],
        TreeNode::Directory(_, elems) => {
            let mut ret = Vec::new();
            for elem in elems.iter() {
                match elem {
                    TreeNode::Directory(_, _) => {
                        ret.push(day7_get_node_size(elem));
                        ret.extend(day7_get_dir_sizes(elem));
                    }
                    TreeNode::File(_, _) => {}
                }
            }
            ret
        }
    }
}

fn day7_get_node_size(node: &TreeNode) -> u64 {
    match node {
        TreeNode::File(_, filesize) => *filesize,
        TreeNode::Directory(_, elems) => elems.iter().map(day7_get_node_size).sum(),
    }
}

#[test]
fn day7_test() {
    assert_eq!(1141028, day7_part1("inputs/7.txt"));
    assert_eq!(8278005, day7_part2("inputs/7.txt"));
}

pub fn day8_part1(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let grid = binding
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let grid_width = grid[0].len();
    let interior_visible_trees = (grid_width * 2) + ((grid_width - 2) * 2);
    let mut res = interior_visible_trees;
    for row in 1..(grid_width - 1) {
        for col in 1..(grid_width - 1) {
            if day8_tree_is_visible(&grid, row, col) {
                res += 1;
            }
        }
    }
    res
}

fn day8_tree_is_visible(grid: &[Vec<u32>], row: usize, col: usize) -> bool {
    let main_tree_height = grid[row][col];
    // Left
    if grid[row][0..col]
        .iter()
        .all(|tree_height| tree_height < &main_tree_height)
    {
        return true;
    }
    // Right
    if grid[row][col + 1..]
        .iter()
        .all(|tree_height| tree_height < &main_tree_height)
    {
        return true;
    }
    // Top
    let mut b = true;
    for r in grid[0..row].iter() {
        if r[col] >= main_tree_height {
            b = false;
        }
    }
    if b {
        return b;
    }
    // Bottom
    let mut b = true;
    for r in grid[row + 1..].iter() {
        if r[col] >= main_tree_height {
            b = false;
        }
    }
    if b {
        return b;
    }
    false
}

pub fn day8_part2(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let grid = binding
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let grid_width = grid[0].len();
    let mut res = day8_scenic_score(&grid, 0, 0);
    for row in 1..(grid_width - 1) {
        for col in 1..(grid_width - 1) {
            let score = day8_scenic_score(&grid, row, col);
            if score > res {
                res = score;
            }
        }
    }
    res
}

fn day8_scenic_score(grid: &[Vec<u32>], row: usize, col: usize) -> usize {
    let grid_size = grid[0].len();
    if row == 0 || col == 0 || row == (grid_size - 1) || col == (grid_size - 1) {
        return 0;
    }
    let main_tree_height = grid[row][col];
    // Left
    let e = grid[row][..col]
        .iter()
        .rev()
        .take_while(|&tree_height| *tree_height < main_tree_height)
        .count();
    let left_score = if e < col { e + 1 } else { e };
    // Right
    let e = grid[row][col + 1..]
        .iter()
        .take_while(|&tree_height| *tree_height < main_tree_height)
        .count();
    let right_score = if e < (grid_size - col - 1) { e + 1 } else { e };
    // Top
    let e = grid[..row]
        .iter()
        .rev()
        .map(|r| r[col])
        .take_while(|&tree_height| tree_height < main_tree_height)
        .count();
    let top_score = if e < row { e + 1 } else { e };
    // Bottom
    let e = grid[row + 1..]
        .iter()
        .map(|r| r[col])
        .take_while(|&tree_height| tree_height < main_tree_height)
        .count();
    let bottom_score = if e < (grid_size - row - 1) { e + 1 } else { e };

    left_score * right_score * top_score * bottom_score
}

#[test]
fn day8_test() {
    assert_eq!(1700, day8_part1("inputs/8.txt"));
    assert_eq!(470596, day8_part2("inputs/8.txt"));
}

pub fn day9_part1(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let motions = binding
        .lines()
        .map(|line| {
            (
                day9_convert_to_direction(line.split(' ').next().unwrap()),
                line.split(' ').nth(1).unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(Direction, u32)>>();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut tail_positions = HashSet::from([tail_pos]);
    for (direction, count) in motions {
        for _ in 0..count {
            head_pos = day9_move_head(&direction, head_pos);
            tail_pos = day9_move_tail(head_pos, tail_pos);
            tail_positions.insert(tail_pos);
        }
    }
    tail_positions.len()
}

pub fn day9_part2(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let motions = binding
        .lines()
        .map(|line| {
            (
                day9_convert_to_direction(line.split(' ').next().unwrap()),
                line.split(' ').nth(1).unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(Direction, u32)>>();
    let mut positions = vec![(0, 0); 10];
    let mut tail_positions = HashSet::from([*positions.last().unwrap()]);
    for (direction, count) in motions {
        for _ in 0..count {
            positions[0] = day9_move_head(&direction, positions[0]);
            for i in 1..positions.len() {
                positions[i] = day9_move_tail(positions[i - 1], positions[i]);
            }
            tail_positions.insert(*positions.last().unwrap());
        }
    }
    tail_positions.len()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn day9_convert_to_direction(c: &str) -> Direction {
    match c {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!(),
    }
}

fn day9_move_head(direction: &Direction, head_pos: (i32, i32)) -> (i32, i32) {
    match direction {
        Direction::Up => (head_pos.0, head_pos.1 + 1),
        Direction::Down => (head_pos.0, head_pos.1 - 1),
        Direction::Left => (head_pos.0 - 1, head_pos.1),
        Direction::Right => (head_pos.0 + 1, head_pos.1),
    }
}

fn day9_move_tail(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    match tail_pos {
        // close enough that tail doesn't move
        x if x == head_pos
            || x == (head_pos.0 + 1, head_pos.1)
            || x == (head_pos.0 - 1, head_pos.1)
            || x == (head_pos.0, head_pos.1 + 1)
            || x == (head_pos.0, head_pos.1 - 1)
            || x == (head_pos.0 + 1, head_pos.1 + 1)
            || x == (head_pos.0 - 1, head_pos.1 + 1)
            || x == (head_pos.0 + 1, head_pos.1 - 1)
            || x == (head_pos.0 - 1, head_pos.1 - 1) =>
        {
            tail_pos
        }
        // 2 away in straight lines
        x if x == (head_pos.0 + 2, head_pos.1) => (tail_pos.0 - 1, tail_pos.1),
        x if x == (head_pos.0 - 2, head_pos.1) => (tail_pos.0 + 1, tail_pos.1),
        x if x == (head_pos.0, head_pos.1 + 2) => (tail_pos.0, tail_pos.1 - 1),
        x if x == (head_pos.0, head_pos.1 - 2) => (tail_pos.0, tail_pos.1 + 1),
        // diagonals
        x if x == (head_pos.0 - 1, head_pos.1 - 2) => (tail_pos.0 + 1, tail_pos.1 + 1),
        x if x == (head_pos.0 - 2, head_pos.1 - 1) => (tail_pos.0 + 1, tail_pos.1 + 1),
        x if x == (head_pos.0 - 2, head_pos.1 + 1) => (tail_pos.0 + 1, tail_pos.1 - 1),
        x if x == (head_pos.0 - 1, head_pos.1 + 2) => (tail_pos.0 + 1, tail_pos.1 - 1),
        x if x == (head_pos.0 + 1, head_pos.1 + 2) => (tail_pos.0 - 1, tail_pos.1 - 1),
        x if x == (head_pos.0 + 2, head_pos.1 + 1) => (tail_pos.0 - 1, tail_pos.1 - 1),
        x if x == (head_pos.0 + 1, head_pos.1 - 2) => (tail_pos.0 - 1, tail_pos.1 + 1),
        x if x == (head_pos.0 + 2, head_pos.1 - 1) => (tail_pos.0 - 1, tail_pos.1 + 1),
        // Longer diagonals (added to complete part 2)
        x if x == (head_pos.0 - 2, head_pos.1 - 2) => (tail_pos.0 + 1, tail_pos.1 + 1),
        x if x == (head_pos.0 - 2, head_pos.1 + 2) => (tail_pos.0 + 1, tail_pos.1 - 1),
        x if x == (head_pos.0 + 2, head_pos.1 + 2) => (tail_pos.0 - 1, tail_pos.1 - 1),
        x if x == (head_pos.0 + 2, head_pos.1 - 2) => (tail_pos.0 - 1, tail_pos.1 + 1),
        _ => panic!("{:?} {:?}", tail_pos, head_pos),
    }
}

#[test]
fn day9_test() {
    assert_eq!(5710, day9_part1("inputs/9.txt"));
    assert_eq!(2259, day9_part2("inputs/9.txt"));
}

pub fn day10_part1(filename: &str) -> i32 {
    let binding = fs::read_to_string(filename).unwrap();
    let mut program = binding
        .lines()
        .rev()
        .map(day10_parse_instruction)
        .collect::<Vec<Day10Instruction>>();
    let mut current_cycle = 1;
    let mut current_instruction = program.pop().unwrap();
    let mut add_counter = 0;
    let mut x_register = 1;
    let mut res = 0;
    while current_cycle <= 220 {
        if HashSet::from([20, 60, 100, 140, 180, 220]).contains(&current_cycle) {
            res += current_cycle * x_register;
        }
        match current_instruction {
            Day10Instruction::Noop => match program.pop() {
                Some(instruction) => {
                    current_instruction = instruction;
                }
                None => {
                    break;
                }
            },
            Day10Instruction::Add(to_add) if add_counter == 1 => {
                x_register += to_add;
                add_counter -= 1;
                match program.pop() {
                    Some(instruction) => {
                        current_instruction = instruction;
                    }
                    None => {
                        break;
                    }
                }
            }
            Day10Instruction::Add(_) => {
                add_counter = 1;
            }
        }
        current_cycle += 1;
    }
    res
}

pub fn day10_part2(filename: &str) -> String {
    let binding = fs::read_to_string(filename).unwrap();
    let mut program = binding
        .lines()
        .rev()
        .map(day10_parse_instruction)
        .collect::<Vec<Day10Instruction>>();
    let mut current_cycle = 1;
    let mut current_instruction = program.pop().unwrap();
    let mut add_counter = 0;
    let mut x_register = 1;
    let mut res = String::with_capacity(240);
    while current_cycle <= 240 {
        let cycle_horiz_position = (current_cycle % 40) - 1;
        if cycle_horiz_position == x_register - 1
            || cycle_horiz_position == x_register
            || cycle_horiz_position == x_register + 1
        {
            res.push('#');
        } else {
            res.push('.');
        }
        if current_cycle % 40 == 0 {
            res.push('\n');
        }
        match current_instruction {
            Day10Instruction::Noop => match program.pop() {
                Some(instruction) => {
                    current_instruction = instruction;
                }
                None => {
                    break;
                }
            },
            Day10Instruction::Add(to_add) if add_counter == 1 => {
                x_register += to_add;
                add_counter -= 1;
                match program.pop() {
                    Some(instruction) => {
                        current_instruction = instruction;
                    }
                    None => {
                        break;
                    }
                }
            }
            Day10Instruction::Add(_) => {
                add_counter = 1;
            }
        }
        current_cycle += 1;
    }
    res
}

#[derive(Debug)]
enum Day10Instruction {
    Noop,
    Add(i32),
}

fn day10_parse_instruction(line: &str) -> Day10Instruction {
    match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
        ["noop"] => Day10Instruction::Noop,
        ["addx", n] => Day10Instruction::Add(n.parse().unwrap()),
        _ => panic!(),
    }
}

#[test]
fn day10_test() {
    assert_eq!(14860, day10_part1("inputs/10.txt"));
    assert_eq!(
        "\
        ###...##..####.####.#..#.#..#.###..#..##\n\
        #..#.#..#....#.#....#..#.#..#.#..#.#.#.#\n\
        #..#.#......#..###..####.#..#.#..#.##...\n\
        ###..#.##..#...#....#..#.#..#.###..#.#.#\n\
        #.#..#..#.#....#....#..#.#..#.#.#..#.#.#\n\
        #..#..###.####.####.#..#..##..#..#.#..#.\n",
        day10_part2("inputs/10.txt")
    );
}

pub fn day11_part1(filename: &str) -> u32 {
    let binding = fs::read_to_string(filename).unwrap();
    let mut notes = binding
        .split("\n\n")
        .map(day11_parse_note)
        .collect::<Vec<_>>();
    let mut inspections = vec![0; notes.len()];
    for _ in 0..20 {
        for i in 0..notes.len() {
            while !notes[i].items.is_empty() {
                let note = &mut notes[i];
                let mut item = note.items.pop_front().unwrap();
                item = (note.operation)(item);
                item /= 3;
                let divisible_by = note.divisible_by;
                let true_pass = note.true_pass;
                let false_pass = note.false_pass;
                if item % divisible_by == 0 {
                    notes[true_pass].items.push_back(item);
                } else {
                    notes[false_pass].items.push_back(item);
                }
                inspections[i] += 1;
            }
        }
    }
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

pub fn day11_part2(filename: &str) -> u64 {
    let binding = fs::read_to_string(filename).unwrap();
    let mut notes = binding
        .split("\n\n")
        .map(day11_parse_note)
        .collect::<Vec<_>>();
    let divisor: u64 = notes.iter().map(|n| n.divisible_by).product();
    let mut inspections = vec![0; notes.len()];
    for _ in 0..10000 {
        for i in 0..notes.len() {
            while !notes[i].items.is_empty() {
                let note = &mut notes[i];
                let mut item = note.items.pop_front().unwrap();
                item = (note.operation)(item);
                item %= divisor;
                let divisible_by = note.divisible_by;
                let true_pass = note.true_pass;
                let false_pass = note.false_pass;
                if item % divisible_by == 0 {
                    notes[true_pass].items.push_back(item);
                } else {
                    notes[false_pass].items.push_back(item);
                }
                inspections[i] += 1;
            }
        }
    }
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

struct Day11Note {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divisible_by: u64,
    true_pass: usize,
    false_pass: usize,
}

fn day11_parse_note(note: &str) -> Day11Note {
    let note_vec = note.lines().skip(1).collect::<Vec<_>>();
    Day11Note {
        items: note_vec[0][note_vec[0].find(':').unwrap() + 2..]
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect(),
        operation: {
            let parts = note_vec[1].split(' ').rev().take(2).collect::<Vec<_>>();
            if parts == ["old", "*"] {
                Box::new(|n| n * n)
            } else {
                let num: u64 = parts[0].parse().unwrap();
                match parts[1] {
                    "+" => Box::new(move |n| n + num),
                    "*" => Box::new(move |n| n * num),
                    _ => panic!(),
                }
            }
        },
        divisible_by: note_vec[2]
            .split(' ')
            .rev()
            .next()
            .unwrap()
            .parse()
            .unwrap(),
        true_pass: note_vec[3]
            .split(' ')
            .rev()
            .next()
            .unwrap()
            .parse()
            .unwrap(),
        false_pass: note_vec[4]
            .split(' ')
            .rev()
            .next()
            .unwrap()
            .parse()
            .unwrap(),
    }
}

#[test]
fn day11_test() {
    assert_eq!(67830, day11_part1("inputs/11.txt"));
    assert_eq!(15305381442, day11_part2("inputs/11.txt"));
}

pub fn day12_part1(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let heightmap = binding
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut current_point = (0, 0);
    let mut destination_point = (0, 0);
    let mut distances = HashMap::with_capacity(heightmap.len() * heightmap[0].len());
    let mut unvisited = HashSet::new();
    for (i, row) in heightmap.iter().enumerate() {
        for (j, &elevation) in row.iter().enumerate() {
            if elevation == 'S' {
                current_point = (i, j);
            }
            if elevation == 'E' {
                destination_point = (i, j);
            }
            distances.insert((i, j), 2 * heightmap.len() * heightmap[0].len());
            unvisited.insert((i, j));
        }
    }
    distances.insert(current_point, 0);
    loop {
        let neighbours = day12_neighbours_of(current_point, &heightmap)
            .iter()
            .filter(|neighbour| unvisited.contains(neighbour))
            .copied()
            .collect::<Vec<_>>();
        for neighbour in neighbours {
            let min_distance = distances[&neighbour].min(distances[&current_point] + 1);
            distances.insert(neighbour, min_distance);
        }
        unvisited.remove(&current_point);
        if !unvisited.contains(&destination_point) {
            break;
        }
        current_point = *unvisited.iter().min_by_key(|node| distances[node]).unwrap();
    }
    distances[&destination_point]
}

fn day12_neighbours_of(
    (start_i, start_j): (usize, usize),
    heightmap: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    let row_len = heightmap[0].len() as i64;
    let col_len = heightmap.len() as i64;
    let start_elevation = day12_get_elevation((start_i, start_j), heightmap);
    let sstart_i = start_i as i64;
    let sstart_j = start_j as i64;
    [
        (sstart_i - 1, sstart_j),
        (sstart_i + 1, sstart_j),
        (sstart_i, sstart_j - 1),
        (sstart_i, sstart_j + 1),
    ]
    .iter()
    .filter(|(i, j)| i >= &0 && j >= &0 && i < &col_len && j < &row_len)
    .map(|(i, j)| (*i as usize, *j as usize))
    .filter(|(i, j)| (start_elevation + 1) >= day12_get_elevation((*i, *j), heightmap))
    .collect()
}

fn day12_get_elevation((i, j): (usize, usize), heightmap: &[Vec<char>]) -> u32 {
    match heightmap[i][j] {
        'S' => 0,
        'E' => ('z' as u32) - ('a' as u32),
        c => (c as u32) - ('a' as u32),
    }
}

fn day12_neighbours_of_2(
    (start_i, start_j): (usize, usize),
    heightmap: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    let row_len = heightmap[0].len() as i64;
    let col_len = heightmap.len() as i64;
    let start_elevation = day12_get_elevation((start_i, start_j), heightmap) as i64;
    let sstart_i = start_i as i64;
    let sstart_j = start_j as i64;
    [
        (sstart_i - 1, sstart_j),
        (sstart_i + 1, sstart_j),
        (sstart_i, sstart_j - 1),
        (sstart_i, sstart_j + 1),
    ]
    .iter()
    .filter(|(i, j)| i >= &0 && j >= &0 && i < &col_len && j < &row_len)
    .map(|(i, j)| (*i as usize, *j as usize))
    .filter(|(i, j)| (start_elevation - 1) <= (day12_get_elevation((*i, *j), heightmap) as i64))
    .collect()
}

pub fn day12_part2(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let heightmap = binding
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let effective_infinity = 2 * heightmap.len() * heightmap[0].len();
    let mut current_point = (0, 0);
    let mut destination_points = HashSet::new();
    let mut distances = HashMap::with_capacity(heightmap.len() * heightmap[0].len());
    let mut unvisited = HashSet::new();
    for (i, row) in heightmap.iter().enumerate() {
        for (j, &elevation) in row.iter().enumerate() {
            if elevation == 'E' {
                current_point = (i, j);
            }
            if elevation == 'S' || elevation == 'a' {
                destination_points.insert((i, j));
            }
            distances.insert((i, j), effective_infinity);
            unvisited.insert((i, j));
        }
    }
    distances.insert(current_point, 0);
    loop {
        let neighbours = day12_neighbours_of_2(current_point, &heightmap)
            .iter()
            .filter(|neighbour| unvisited.contains(neighbour))
            .copied()
            .collect::<Vec<_>>();
        for neighbour in neighbours {
            let min_distance = distances[&neighbour].min(distances[&current_point] + 1);
            distances.insert(neighbour, min_distance);
        }
        unvisited.remove(&current_point);
        if unvisited.is_empty()
            || unvisited.iter().map(|node| distances[node]).min().unwrap() == effective_infinity
        {
            break;
        }
        current_point = *unvisited.iter().min_by_key(|node| distances[node]).unwrap();
    }
    destination_points
        .iter()
        .map(|node| distances[node])
        .min()
        .unwrap()
}

#[test]
fn day12_test() {
    assert_eq!(497, day12_part1("inputs/12.txt"));
    assert_eq!(492, day12_part2("inputs/12.txt"));
}

pub fn day13_part1(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let pairs = binding
        .split("\n\n")
        .map(|pair| {
            let mut parsed_pair = pair
                .lines()
                .map(|l| serde_json::from_str::<Day13List>(l).unwrap())
                .collect::<Vec<_>>();
            (parsed_pair.remove(0), parsed_pair.remove(0))
        })
        .collect::<Vec<_>>();
    let mut res = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if day13_pair_in_right_order(&pair.0, &pair.1) == Ordering::Less {
            res += i + 1;
        }
    }
    res
}

pub fn day13_part2(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let mut pairs = binding
        .split("\n\n")
        .flat_map(|pair| pair.lines())
        .map(serde_json::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let first_divider = Day13List::List(vec![Day13List::List(vec![Day13List::Num(2)])]);
    let second_divider = Day13List::List(vec![Day13List::List(vec![Day13List::Num(6)])]);
    pairs.push(first_divider.clone());
    pairs.push(second_divider.clone());

    pairs.sort_by(day13_pair_in_right_order);

    let first_pos = pairs
        .iter()
        .position(|pair| pair == &first_divider)
        .unwrap()
        + 1;
    let second_pos = pairs
        .iter()
        .position(|pair| pair == &second_divider)
        .unwrap()
        + 1;
    first_pos * second_pos
}

#[derive(Clone, Debug, Deserialize, serde::Serialize)]
#[serde(untagged)]
#[derive(PartialEq)]
enum Day13List {
    Num(u32),
    List(Vec<Day13List>),
}

fn day13_pair_in_right_order(left: &Day13List, right: &Day13List) -> Ordering {
    match (left, right) {
        (Day13List::Num(l), Day13List::Num(r)) => l.cmp(r),
        (Day13List::Num(_), Day13List::List(_)) => {
            day13_pair_in_right_order(&Day13List::List(vec![left.clone()]), right)
        }
        (Day13List::List(_), Day13List::Num(_)) => {
            day13_pair_in_right_order(left, &Day13List::List(vec![right.clone()]))
        }
        (Day13List::List(l), Day13List::List(r)) => {
            for (ll, rr) in l.iter().zip(r) {
                match day13_pair_in_right_order(ll, rr) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            l.len().cmp(&r.len())
        }
    }
}

#[test]
fn day13_test() {
    assert_eq!(4894, day13_part1("inputs/13.txt"));
    assert_eq!(24180, day13_part2("inputs/13.txt"));
}

pub fn day14_part1(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let mut points = day14_parse_input(&binding);
    let lowest_point = *points.iter().map(|(_, y)| y).max().unwrap();
    let sand_spawn_point = (500, 0);
    let mut caught_sand = 0;
    loop {
        let mut sand_grain = sand_spawn_point;
        loop {
            if sand_grain.1 > (lowest_point + 100) {
                return caught_sand;
            }
            let one_below = (sand_grain.0, sand_grain.1 + 1);
            if !points.contains(&one_below) {
                sand_grain = one_below;
            } else {
                let one_below_left = (sand_grain.0 - 1, sand_grain.1 + 1);
                if !points.contains(&one_below_left) {
                    sand_grain = one_below_left;
                } else {
                    let one_below_right = (sand_grain.0 + 1, sand_grain.1 + 1);
                    if !points.contains(&one_below_right) {
                        sand_grain = one_below_right;
                    } else {
                        // Sand is blocked all ways
                        points.insert(sand_grain);
                        caught_sand += 1;
                        break;
                    }
                }
            }
        }
    }
}

pub fn day14_part2(filename: &str) -> usize {
    let binding = fs::read_to_string(filename).unwrap();
    let mut points = day14_parse_input(&binding);
    let floor_height = 2 + points.iter().map(|(_, y)| y).max().unwrap();
    let sand_spawn_point = (500, 0);
    let mut caught_sand = 0;
    loop {
        let mut sand_grain = sand_spawn_point;
        loop {
            if sand_grain.1 + 1 == floor_height {
                // Sand grain is on the floor and can't go any further
                points.insert(sand_grain);
                caught_sand += 1;
                break;
            }
            let one_below = (sand_grain.0, sand_grain.1 + 1);
            if !points.contains(&one_below) {
                sand_grain = one_below;
            } else {
                let one_below_left = (sand_grain.0 - 1, sand_grain.1 + 1);
                if !points.contains(&one_below_left) {
                    sand_grain = one_below_left;
                } else {
                    let one_below_right = (sand_grain.0 + 1, sand_grain.1 + 1);
                    if !points.contains(&one_below_right) {
                        sand_grain = one_below_right;
                    } else {
                        // Sand is blocked all ways
                        points.insert(sand_grain);
                        caught_sand += 1;
                        // If sand grain is still at the spawn point, then return result
                        if sand_grain == sand_spawn_point {
                            return caught_sand;
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn day14_parse_input(input: &str) -> HashSet<(u32, u32)> {
    input
        .lines()
        .flat_map(|line| {
            let points = line
                .split(" -> ")
                .map(|point| {
                    (
                        point.split(',').next().unwrap().parse().unwrap(),
                        point.split(',').nth(1).unwrap().parse().unwrap(),
                    )
                })
                .collect::<Vec<(u32, u32)>>();
            points
                .iter()
                .zip(points.iter().skip(1))
                .flat_map(|((first_x, first_y), (second_x, second_y))| {
                    if first_x == second_x {
                        match first_y.cmp(second_y) {
                            Ordering::Greater => {
                                (*second_y..(*first_y + 1)).map(|y| (*first_x, y)).collect()
                            }
                            Ordering::Less => {
                                (*first_y..(*second_y + 1)).map(|y| (*first_x, y)).collect()
                            }
                            Ordering::Equal => HashSet::from([(*first_x, *first_y)]),
                        }
                    } else if first_y == second_y {
                        match first_x.cmp(second_x) {
                            Ordering::Greater => {
                                (*second_x..(*first_x + 1)).map(|x| (x, *first_y)).collect()
                            }
                            Ordering::Less => {
                                (*first_x..(*second_x + 1)).map(|x| (x, *first_y)).collect()
                            }
                            Ordering::Equal => HashSet::from([(*first_x, *first_y)]),
                        }
                    } else {
                        panic!();
                    }
                })
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>()
}

#[test]
fn day14_test() {
    assert_eq!(979, day14_part1("inputs/14.txt"));
    assert_eq!(29044, day14_part2("inputs/14.txt"));
}
