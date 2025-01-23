use std::collections::{HashMap, HashSet};

use helpers::Puzzle;

struct Day19;

struct Spa {
    towels: HashMap<usize, HashSet<String>>,
    designs: Vec<String>,
}

impl From<String> for Spa {
    fn from(val: String) -> Self {
        let mut lines = val.lines();
        let towel_line = lines.next().expect("has to have at least one line");
        let towels = towel_line.split(", ").map(|val| val.to_string()).fold(
            HashMap::new(),
            |mut acc, val| {
                acc.entry(val.len())
                    .or_insert_with(|| HashSet::new())
                    .insert(val);
                acc
            },
        );

        // skip the blank line
        lines.next();

        let designs = lines
            .into_iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>();

        Self { towels, designs }
    }
}

impl Spa {
    fn count_valid_designs(self) -> usize {
        //println!("towels: {:?}", self.towels);
        self.designs
            .iter()
            .enumerate()
            .filter(|(idx, design_str)| {
                println!("{idx}");
                println!("design_str: {design_str}");
                let mut potentials = Vec::<&str>::new();
                self.get_potentials(design_str, &mut potentials);
                println!("potentials: {:?}", potentials);
                while let Some(curr) = potentials.pop() {
                    println!("len({})", curr.len());
                    if curr.len() == 0 {
                        return true;
                    }
                    self.get_potentials(curr, &mut potentials);
                    //println!("potentials: {:?}", potentials);
                }
                false
            })
            .count()
    }

    fn total_valid_designs(self) -> usize {
        println!("towels: {:?}", self.towels);
        self.designs
            .iter()
            .enumerate()
            .map(|(idx, design_str)| {
                println!("{idx}");
                println!("design_str: {design_str}");

                let mut counts = vec![0; design_str.len() + 1];
                counts[0] = 1;

                for idx in 0..counts.len() {
                    println!("counts: {:?}", counts);
                    if counts[idx] == 0 {
                        continue;
                    }
                    for (key, set) in self.towels.iter() {
                        let next_idx = idx + *key;
                        if next_idx < counts.len() && set.contains(&design_str[idx..next_idx]) {
                            counts[next_idx] += counts[idx];
                        }
                    }
                }

                println!("counts: {:?}", counts);
                println!("to_add: {}", counts[counts.len() - 1]);
                counts[counts.len() - 1]
            })
            .sum()
    }

    fn get_potentials<'a>(&'a self, curr_str: &'a str, potentials: &mut Vec<&'a str>) {
        for (key, set) in self.towels.iter() {
            if *key <= curr_str.len() && set.contains(&curr_str[0..*key]) {
                let to_push = &curr_str[*key..];
                assert!(to_push.len() < curr_str.len());
                println!("curr_str: {curr_str}");
                println!("set: {:?}", set);
                println!("to_push: {to_push}");
                potentials.push(to_push);
            }
        }
    }
}

impl Puzzle for Day19 {
    fn puzzle_1(contents: String) {
        let spa = Spa::from(contents);
        println!("total num designs: {}", spa.designs.len());
        let num_valid = spa.count_valid_designs();
        println!("found {num_valid} valid designs");
    }

    fn puzzle_2(contents: String) {
        let spa = Spa::from(contents);
        println!("total num designs: {}", spa.designs.len());
        let total_valid = spa.total_valid_designs();
        println!("found {total_valid} valid designs");
    }
}

fn main() {
    Day19::run()
}
