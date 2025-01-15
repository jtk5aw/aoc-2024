use std::{collections::HashMap, env, fs};

use helpers::Puzzle;

fn main() {
    Day1::run();
}

struct Day1;

impl Puzzle for Day1 {
    fn puzzle_1(contents: String) {
        let mut list_1 = Vec::with_capacity(contents.len());
        let mut list_2 = Vec::with_capacity(contents.len());

        for line in contents.lines() {
            let mut split_line = line.split_whitespace();
            list_1.push(
                split_line
                    .next()
                    .expect("Has to have first element")
                    .parse::<i64>()
                    .expect("Has to be a number"),
            );
            list_2.push(
                split_line
                    .next()
                    .expect("Has to have second element")
                    .parse::<i64>()
                    .expect("Has to be a number"),
            );
        }

        list_1.sort();
        list_2.sort();

        let val = list_1
            .iter()
            .zip(list_2.iter())
            .fold(0, |acc, pair| acc + (pair.0 - pair.1).abs());

        println!("The diff in location ids is: {val}");
    }

    fn puzzle_2(contents: String) {
        let mut list_1 = Vec::with_capacity(contents.len());
        let mut map = HashMap::new();

        for line in contents.lines() {
            let mut split_line = line.split_whitespace();
            list_1.push(
                split_line
                    .next()
                    .expect("Has to have first element")
                    .parse::<i64>()
                    .expect("Has to be a number"),
            );
            map.entry(
                split_line
                    .next()
                    .expect("Has to have second element")
                    .parse::<i64>()
                    .expect("Has to be a number"),
            )
            .and_modify(|count| *count += 1)
            .or_insert(1);
        }

        let result = list_1.iter().fold(0, |acc, val| {
            let count = map.get(&val).unwrap_or_else(|| &0);
            acc + (val * count)
        });

        println!("Similarity score is: {result}");
    }
}
