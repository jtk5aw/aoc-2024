use std::{collections::HashMap, str::SplitWhitespace};

use helpers::Puzzle;

fn main() {
    Day11::run();
}

struct Day11;

fn process_stone(
    stone_str: &str,
    val: usize,
    mut memoized: &mut HashMap<(String, usize), usize>,
) -> usize {
    if val == 0 {
        println!("{stone_str}");
        return 1;
    }

    if let Some(count) = memoized.get(&(stone_str.to_string(), val)) {
        return *count;
    }

    if stone_str == "0" {
        let result = process_stone("1", val - 1, &mut memoized);
        memoized.insert((stone_str.to_string(), val), result);
        return result;
    }

    if stone_str.len() % 2 == 0 {
        let front_str = &stone_str[0..stone_str.len() / 2];
        let mut back_str_start = stone_str.len() / 2;
        let mut back_chars = stone_str[back_str_start..].chars();
        while let Some(next_char) = back_chars.next() {
            if back_str_start == stone_str.len() - 1 || next_char != '0' {
                break;
            }
            back_str_start += 1;
        }
        let back_str = &stone_str[back_str_start..];

        let result = process_stone(front_str, val - 1, &mut memoized)
            + process_stone(back_str, val - 1, &mut memoized);
        memoized.insert((stone_str.to_string(), val), result);
        return result;
    }

    let new_num = stone_str.parse::<u64>().expect("has to be a number") * 2024;
    let new_num_string = new_num.to_string();
    let result = process_stone(&new_num_string, val - 1, &mut memoized);
    memoized.insert((stone_str.to_string(), val), result);
    result
}

fn process_blinks(strs: SplitWhitespace<'_>, num_blinks: usize) -> usize {
    let mut sum = 0;
    let mut memoized = HashMap::new();
    for stone_str in strs {
        sum += process_stone(stone_str, num_blinks, &mut memoized);
    }
    sum
}

impl Puzzle for Day11 {
    fn puzzle_1(contents: String) {
        let sum = process_blinks(contents.split_whitespace(), 25);
        println!("the sum is: {sum}");
    }

    fn puzzle_2(contents: String) {
        let sum = process_blinks(contents.split_whitespace(), 75);
        println!("the sum is: {sum}");
    }
}
