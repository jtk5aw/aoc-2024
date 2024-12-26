use helpers::Puzzle;
use regex::{Match, Regex};

fn main() {
    Day3::run();
}

struct Day3;

fn split_mult(match_val: Match<'_>) -> (&str, &str) {
    match_val
        .as_str()
        .strip_prefix("mul(")
        .expect("Had to start with mul(")
        .strip_suffix(")")
        .expect("Had to end with )")
        .split_once(",")
        .expect("Had to have one comma")
}

impl Puzzle for Day3 {
    fn puzzle_1(contents: String) {
        let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("Should be valid regex");

        let result = regex
            .find_iter(&contents)
            .map(|mult_str| split_mult(mult_str))
            .fold(0, |acc, (first_str, second_str)| {
                let first_num = first_str.parse::<i64>().expect("Has to be a num");
                let second_num = second_str.parse::<i64>().expect("Has to be a num");
                acc + (first_num * second_num)
            });

        println!("mult result: {result}");
    }

    fn puzzle_2(contents: String) {
        let regex =
            Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").expect("Should be valid regex");

        let mult_list = regex
            .find_iter(&contents)
            .map(|match_val| match match_val.as_str() {
                "do()" => MultType::Do,
                "don't()" => MultType::Dont,
                _ => MultType::from(split_mult(match_val)),
            })
            .collect::<Vec<_>>();

        let mut should_mult = true;
        let mut product = 0;

        for mult_val in mult_list {
            match mult_val {
                MultType::Do => {
                    should_mult = true;
                }
                MultType::Dont => {
                    should_mult = false;
                }
                MultType::Mult(val) => {
                    if should_mult {
                        product += val
                    }
                }
            }
        }

        println!("Mult result: {:?}", product);
    }
}

enum MultType {
    Do,
    Dont,
    Mult(i64),
}

impl From<(&str, &str)> for MultType {
    fn from(value: (&str, &str)) -> Self {
        let first_num = value.0.parse::<i64>().expect("Has to be a num");
        let second_num = value.1.parse::<i64>().expect("Has to be a num");
        MultType::Mult(first_num * second_num)
    }
}
