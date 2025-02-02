use std::collections::{HashMap, HashSet};

use helpers::Puzzle;

struct Day23;

impl Puzzle for Day23 {
    fn puzzle_1(contents: String) {
        let (graph, groups) = contents
            .lines()
            .map(|line| line.split_once("-").expect("has to have -"))
            .fold(
                (
                    HashMap::<&str, HashSet<&str>>::new(),
                    HashSet::<[&str; 3]>::new(),
                ),
                |(mut graph, mut groups), (first_node, second_node)| {
                    let first_connections = graph
                        .entry(first_node)
                        .and_modify(|set| {
                            set.insert(second_node);
                        })
                        .or_insert_with(|| HashSet::from_iter(vec![second_node]))
                        .clone();
                    let second_connections = graph
                        .entry(second_node)
                        .and_modify(|set| {
                            set.insert(first_node);
                        })
                        .or_insert_with(|| HashSet::from_iter(vec![first_node]))
                        .clone();
                    first_connections
                        .intersection(&second_connections)
                        .map(|third_node| {
                            let mut to_add = [first_node, second_node, third_node];
                            to_add.sort();
                            to_add
                        })
                        .for_each(|new_group| {
                            groups.insert(new_group);
                        });
                    (graph, groups)
                },
            );
        let mut t_groups = Vec::with_capacity(groups.len());
        for group in groups {
            println!("found {:?}", group);
            for item in group {
                if item.starts_with("t") {
                    t_groups.push(group);
                    break;
                }
            }
        }
        println!("number of t groups is {}", t_groups.len());
    }

    fn puzzle_2(contents: String) {
        let graph = contents
            .lines()
            .map(|line| line.split_once("-").expect("has to have -"))
            .fold(
                HashMap::<&str, HashSet<&str>>::new(),
                |mut graph, (first_node, second_node)| {
                    graph
                        .entry(first_node)
                        .and_modify(|set| {
                            set.insert(second_node);
                        })
                        .or_insert_with(|| HashSet::from_iter(vec![second_node]));
                    graph
                        .entry(second_node)
                        .and_modify(|set| {
                            set.insert(first_node);
                        })
                        .or_insert_with(|| HashSet::from_iter(vec![first_node]));
                    graph
                },
            );

        let mut maximum_group = Vec::new();
        for initial_node in graph.keys() {
            if let Some(connected) = graph.get(initial_node) {
                let mut current_group = vec![initial_node];
                for next_node in connected.iter() {
                    let in_all = match graph.get(next_node) {
                        Some(next_node_set) => current_group
                            .iter()
                            .all(|current_node| next_node_set.contains(*current_node)),
                        None => false,
                    };
                    if in_all {
                        current_group.push(next_node);
                    }
                }
                if current_group.len() > maximum_group.len() {
                    println!("found new maximum group!");
                    maximum_group = current_group;
                }
            }
        }
        maximum_group.sort();
        println!(
            "maximum group is: len({}) {:?}",
            maximum_group.len(),
            maximum_group
        );
        print!("password is: ");
        for val in maximum_group {
            print!("{},", val);
        }
        println!("");
    }
}

fn main() {
    Day23::run();
}
