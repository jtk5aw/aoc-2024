use helpers::Puzzle;

fn main() {
    Day7::run()
}

struct Day7;

impl Puzzle for Day7 {
    fn puzzle_1(contents: String) {
        let mut sum = 0;
        for line in contents.lines() {
            let (target_val_str, rest_of_line) = line.split_once(":").expect("Has to have a :");
            let target_val = target_val_str
                .parse::<i64>()
                .expect("target has to be an i64");
            let vals = rest_of_line
                .strip_prefix(" ")
                .expect("has to start with a space")
                .split_whitespace()
                .map(|val_str| {
                    val_str
                        .parse::<i64>()
                        .expect("each individual value has to be an i64")
                })
                .collect::<Vec<_>>();

            assert!(vals.len() >= 2);

            let mut intermediate_results = Vec::with_capacity(2);

            intermediate_results.push(vals[0] + vals[1]);
            intermediate_results.push(vals[0] * vals[1]);

            for new_val_idx in 2..vals.len() {
                let new_val = vals[new_val_idx];
                let mut new_intermediate_results = Vec::with_capacity(2 ^ new_val_idx);
                for old_val in intermediate_results {
                    new_intermediate_results.push(old_val + new_val);
                    new_intermediate_results.push(old_val * new_val);
                }
                intermediate_results = new_intermediate_results;
            }

            for val in intermediate_results {
                if val == target_val {
                    println!("met the target_val!: {target_val}");
                    sum += target_val;
                    break;
                }
            }
        }

        println!("Sum is: {sum}");
    }

    fn puzzle_2(contents: String) {
        let mut sum = 0;
        for line in contents.lines() {
            let (target_val_str, rest_of_line) = line.split_once(":").expect("Has to have a :");
            let target_val = target_val_str
                .parse::<i64>()
                .expect("target has to be an i64");
            let mut vals = rest_of_line
                .strip_prefix(" ")
                .expect("has to start with a space")
                .split_whitespace()
                .map(|val_str| {
                    (
                        val_str,
                        val_str
                            .parse::<i64>()
                            .expect("each individual value has to be an i64"),
                    )
                })
                .map(NumAndStr::from)
                .collect::<Vec<_>>();

            assert!(vals.len() >= 2);

            let mut intermediate_results = Vec::with_capacity(3);

            let first = vals.remove(0);
            let second = vals.remove(0);
            let (added, multed, concated) = first.perform_operators(&second);

            intermediate_results.push(added);
            intermediate_results.push(multed);
            intermediate_results.push(concated);

            // remove the mut
            let vals = vals;

            for (new_val_idx, new_val) in vals.iter().enumerate() {
                let mut new_intermediate_results = Vec::with_capacity(3 ^ new_val_idx);
                for old_val in intermediate_results {
                    let (added, multed, concated) = old_val.perform_operators(&new_val);
                    new_intermediate_results.push(added);
                    new_intermediate_results.push(multed);
                    new_intermediate_results.push(concated);
                }
                intermediate_results = new_intermediate_results;
            }

            for val in intermediate_results {
                if val.num_val == target_val {
                    println!("met the target_val!: {target_val}");
                    sum += target_val;
                    break;
                }
            }
        }

        println!("Sum is: {sum}");
    }
}

struct NumAndStr {
    str_val: String,
    num_val: i64,
}

impl From<(&str, i64)> for NumAndStr {
    fn from(value: (&str, i64)) -> Self {
        NumAndStr {
            str_val: value.0.to_string(),
            num_val: value.1,
        }
    }
}

impl NumAndStr {
    fn perform_operators(mut self, rhs: &Self) -> (Self, Self, Self) {
        let added = NumAndStr {
            num_val: self.num_val + rhs.num_val,
            str_val: (self.num_val + rhs.num_val).to_string(),
        };
        let multed = NumAndStr {
            num_val: self.num_val * rhs.num_val,
            str_val: (self.num_val * rhs.num_val).to_string(),
        };
        self.str_val.push_str(&rhs.str_val);
        let num_val = self
            .str_val
            .parse::<i64>()
            .expect("Concated value has to be an i64");
        let concated = NumAndStr {
            str_val: self.str_val,
            num_val,
        };

        (added, multed, concated)
    }
}
