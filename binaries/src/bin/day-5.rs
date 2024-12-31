use std::{
    borrow::{Borrow, Cow},
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
    str::Lines,
};

use helpers::Puzzle;

fn main() {
    Day5::run();
}

struct Day5;

#[derive(Debug)]
struct Dag {
    graph: HashMap<String, Vec<String>>,
    // The structure as a whole actually isn't a dag so this isn't super useful
    // It's only a dag when cut into subsegments. So rather than relying on roots
    // you can just insert all the elements being currently considered
    roots: Vec<String>,
}

// NOTE: This might be worthless, I thought it might be helpful but :shrug:
struct TopologicalSortErr {
    line: Vec<String>,
    too_early_idx: usize,
    too_late_idx: usize,
}

fn is_topological_sort(dag: &Dag, candidate: &Vec<&str>) -> Result<(), TopologicalSortErr> {
    let mut indices = HashMap::new();
    for (idx, value) in candidate.iter().enumerate() {
        indices.insert(value.to_string(), idx);
    }

    let mut to_visit = VecDeque::new();
    for val in candidate {
        to_visit.push_front(val.to_string());
    }
    let mut visited = HashSet::new();

    while !to_visit.is_empty() {
        let curr_val = to_visit.pop_back().expect("Already checked if empty");

        if visited.contains(&curr_val) {
            continue;
        }

        let connected_nodes = dag
            .graph
            .get(&curr_val)
            .map_or_else(|| Cow::from(vec![]), |val| Cow::from(val));

        if let Some(curr_idx) = indices.get(&curr_val) {
            let other_idxs = connected_nodes.iter().filter_map(|node| indices.get(node));
            for connected_idx in other_idxs {
                if connected_idx < curr_idx {
                    return Err(TopologicalSortErr {
                        line: candidate.iter().map(|str| str.to_string()).collect(),
                        too_late_idx: *connected_idx,
                        too_early_idx: *curr_idx,
                    });
                }
            }
        }

        for node in connected_nodes.iter() {
            to_visit.push_front(node.to_string());
        }

        visited.insert(curr_val);
    }

    Ok(())
}

fn generate_dag(line_iter: &mut Lines<'_>) -> Dag {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut indegree: HashMap<String, i64> = HashMap::new();
    let mut curr_line = line_iter.next().expect("has to have at least one line");
    while !curr_line.is_empty() {
        let (before_str, after_str) = curr_line.split_once("|").expect("Has to have a |");

        // Creates the dag itself
        graph
            .entry(before_str.to_string())
            .and_modify(|list| list.push(after_str.to_string()))
            .or_insert(vec![after_str.to_string()]);
        // Used to determine roots
        indegree.entry(before_str.to_string()).or_insert(0);
        indegree
            .entry(after_str.to_string())
            .and_modify(|val| *val += 1)
            .or_insert(1);

        curr_line = line_iter.next().expect("Has to have a next line");
    }

    Dag {
        graph,
        roots: indegree
            .into_iter()
            .filter(|(_, indegree_num)| indegree_num == &0)
            .map(|(node, _)| node)
            .collect(),
    }
}

impl Puzzle for Day5 {
    fn puzzle_1(contents: String) {
        let mut line_iter = contents.lines();

        let dag = generate_dag(&mut line_iter);

        println!("The dag is: {:?}", dag);

        let result = line_iter
            .map(|line| line.split(",").collect::<Vec<_>>())
            .filter(|line| is_topological_sort(&dag, line).is_ok())
            .fold(0, |acc, sorted| {
                acc + sorted[sorted.len() / 2]
                    .parse::<i64>()
                    .expect("has to be a number")
            });

        println!("Sum of sorted middles is: {result}");
    }

    fn puzzle_2(contents: String) {
        let mut line_iter = contents.lines();

        let dag = generate_dag(&mut line_iter);

        println!("The dag is: {:?}", dag);

        let result = line_iter
            .map(|line| line.split(",").collect::<Vec<_>>())
            .filter_map(|line| match is_topological_sort(&dag, &line) {
                Err(data) => Some(data),
                Ok(_) => None,
            })
            .map(|err_data| sort_topologically(&dag, err_data))
            .fold(0, |acc, sorted| {
                println!("sorted is: {:?}", sorted);
                acc + sorted[sorted.len() / 2]
                    .parse::<i64>()
                    .expect("has to be a number")
            });

        println!("Sum of sorted middles is: {result}");
    }
}

// NOTE: I almost assuredly complicated the hell out of this but :shrug:
fn sort_topologically(dag: &Dag, err_data: TopologicalSortErr) -> Vec<String> {
    let nodes_to_consider: HashSet<String> = HashSet::from_iter(err_data.line);
    let mut indegrees = HashMap::new();
    let mut mini_dag = HashMap::new();

    for node in nodes_to_consider.clone() {
        let connected_nodes = dag
            .graph
            .get(&node)
            .map(|list| {
                list.iter()
                    .filter(|val| nodes_to_consider.contains(&val.to_string()))
                    .collect::<Vec<_>>()
            })
            .map_or_else(|| Vec::new(), |list| list.to_owned());

        indegrees.entry(node.clone()).or_insert(0);

        for connected_node in connected_nodes.iter() {
            indegrees
                .entry(connected_node.to_string())
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }

        mini_dag.insert(node, connected_nodes);
    }

    let roots = indegrees
        .iter()
        .filter(|(_, indegree)| **indegree == 0)
        .map(|(node, _)| node.to_string())
        .collect::<Vec<_>>();

    let mut result = Vec::with_capacity(nodes_to_consider.len());
    let mut queue = VecDeque::with_capacity(nodes_to_consider.len());
    for node in roots {
        queue.push_front(node);
    }

    while !queue.is_empty() {
        let curr_val = queue.pop_back().expect("Can't be empty");
        let connected_nodes = mini_dag
            .get(&curr_val)
            .map_or_else(|| Cow::from(vec![]), |val| Cow::from(val));

        for connected_node in connected_nodes.iter() {
            let indegree = indegrees
                .get_mut(&connected_node.to_string())
                .expect("has to have an indegree");
            *indegree -= 1;
            if *indegree == 0 {
                queue.push_front(connected_node.to_string());
            }
        }

        result.push(curr_val);
    }

    result
}
