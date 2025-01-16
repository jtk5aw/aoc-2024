use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

use helpers::{read_grid, Puzzle};

struct Day16;

fn main() {
    Day16::run()
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
    #[default]
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn cost(&self, other: Direction) -> Option<usize> {
        if self.is_reverse(&other) {
            return None;
        }
        if self.eq(&other) {
            return Some(1);
        }
        return Some(1001);
    }

    fn is_reverse(&self, other: &Direction) -> bool {
        let (lower, higher) = if self < other {
            (self, other)
        } else {
            (other, self)
        };
        match (lower, higher) {
            (Direction::Up, Direction::Down) | (Direction::Right, Direction::Left) => true,
            _ => false,
        }
    }

    fn sides(&self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Right | Direction::Left => [Direction::Up, Direction::Down],
        }
    }

    fn is_open(&self, coord: &Coord, grid: &Vec<Vec<char>>) -> bool {
        match self {
            Direction::Up if grid[coord.0 - 1][coord.1] != '#' => true,
            Direction::Down if grid[coord.0 + 1][coord.1] != '#' => true,
            Direction::Right if grid[coord.0][coord.1 + 1] != '#' => true,
            Direction::Left if grid[coord.0][coord.1 - 1] != '#' => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
struct Coord(usize, usize);

impl From<(usize, usize)> for Coord {
    fn from(val: (usize, usize)) -> Self {
        Self(val.0, val.1)
    }
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
struct DirectionalCoord(Coord, Direction);

impl From<((usize, usize), Direction)> for DirectionalCoord {
    fn from(val: ((usize, usize), Direction)) -> Self {
        Self((val.0 .0, val.0 .1).into(), val.1)
    }
}

impl From<(Coord, Direction)> for DirectionalCoord {
    fn from(val: (Coord, Direction)) -> Self {
        Self(val.0, val.1)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Edge(DirectionalCoord, usize);

impl From<(Coord, Direction, usize)> for Edge {
    fn from(val: (Coord, Direction, usize)) -> Self {
        Self((val.0, val.1).into(), val.2)
    }
}

struct CreateEdgeIterator<'a> {
    current_point: Option<(usize, usize)>,
    direction: Direction,
    grid: &'a Vec<Vec<char>>,
}

impl<'a> CreateEdgeIterator<'a> {
    fn new(start: Coord, direction: Direction, grid: &'a Vec<Vec<char>>) -> Self {
        Self {
            current_point: Some((start.0, start.1)),
            direction,
            grid,
        }
    }
}

impl<'a> Iterator for CreateEdgeIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let current_opt = self.current_point;

        let potential_current_point = match (current_opt, &self.direction) {
            (Some(current), Direction::Up) => Some((current.0 - 1, current.1)),
            (Some(current), Direction::Down) => Some((current.0 + 1, current.1)),
            (Some(current), Direction::Right) => Some((current.0, current.1 + 1)),
            (Some(current), Direction::Left) => Some((current.0, current.1 - 1)),
            (None, _) => None,
        };

        self.current_point = match potential_current_point {
            Some(coord) | Some(coord) if self.grid[coord.0][coord.1] == '#' => None,
            None => None,
            _ => potential_current_point,
        };

        current_opt
    }
}

#[derive(Debug)]
struct Graph {
    start: Coord,
    end: Coord,
    edges: HashMap<DirectionalCoord, Vec<Edge>>,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Default)]
struct HeapNode<T, H>
where
    T: Hash + Ord + Default + Clone,
    H: Hash + Ord,
{
    priority: usize,
    value: T,
    history: Vec<H>,
}

impl<T, H> HeapNode<T, H>
where
    T: Eq + PartialEq + Hash + Ord + Default + Clone,
    H: Eq + PartialEq + Hash + Ord,
{
    fn new(value: T) -> Self {
        HeapNode {
            priority: usize::MAX,
            value,
            history: Vec::new(),
        }
    }

    fn with_priority(value: T, priority: usize, history: Vec<H>) -> Self {
        HeapNode {
            priority,
            value,
            history,
        }
    }
}

impl<T, H> Ord for HeapNode<T, H>
where
    T: Hash + Eq + Ord + Default + Clone,
    H: Hash + Eq + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Flips ordering for min-heap
        other
            .priority
            .cmp(&self.priority)
            .then(other.value.cmp(&self.value))
    }
}

impl<T, H> PartialOrd for HeapNode<T, H>
where
    T: Hash + Eq + Ord + Default + Clone,
    H: Hash + Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    fn init_key(&mut self, key: DirectionalCoord) -> bool {
        if let Some(_) = self.edges.get(&key) {
            return false;
        }
        self.edges.insert(key, Vec::new());
        true
    }

    fn add_edge(&mut self, key: DirectionalCoord, edge: Edge) {
        self.edges.entry(key).or_insert_with(Vec::new).push(edge);
    }

    fn shortest_path(self, grid: Vec<Vec<char>>) -> (usize, HashSet<Coord>) {
        let mut unvisited = IndexedBinaryHeap::new();
        for key in self.edges.keys() {
            if key.0 == self.start {
                unvisited.push(HeapNode::with_priority(
                    key.clone(),
                    0,
                    vec![self.start.clone()],
                ));
            } else {
                unvisited.push(HeapNode::new(key.clone()));
            }
        }

        let mut locations = HashSet::new();
        let mut answer = usize::MAX;
        let mut i = 0;
        unvisited.print_binary_tree();
        while let Some(node) = unvisited.pop() {
            //println!("popped: {:?}", node);
            if node.priority > answer {
                continue;
            }

            self.edges
                .get(&node.value)
                .expect("every unvisited node should have edges")
                .iter()
                .for_each(|edge| {
                    //println!("edge: {:?}", edge);
                    let mut new_history = node.history.clone();
                    new_history.push(edge.0 .0.clone());
                    let new_priority = node.priority + edge.1;
                    match unvisited.attempt_decrement_key(&edge.0, new_priority) {
                        DecrementKeyResult::NoDecrementKeyDoesNotExist
                        | DecrementKeyResult::NoDecrementTooSmall => {}
                        DecrementKeyResult::NoDecrementEqual => {
                            unvisited.add_to_history(&edge.0, new_history)
                        }
                        DecrementKeyResult::SuccessfullyDecremented => {
                            if edge.0 .0 == self.end {
                                answer = node.priority;
                                for coord in new_history.iter() {
                                    locations.insert(coord.clone());
                                }
                            }
                            unvisited.update_history(&edge.0, new_history);
                        }
                    }
                });

            println!("{i}");
            //print_grid_with_costs(unvisited.clone(), grid.clone());
            i += 1;
        }
        print_grid_fill(&locations, grid.clone());
        (answer, locations)
    }
}

#[derive(Clone)]
struct IndexedBinaryHeap<T: Ord + Hash + Default + Clone, H: Ord + Hash> {
    values: Vec<HeapNode<T, H>>,
    indeces: HashMap<DirectionalCoord, usize>,
}

impl IndexedBinaryHeap<DirectionalCoord, Coord> {
    fn new() -> Self {
        let mut values = Vec::new();
        values.push(HeapNode::new(DirectionalCoord::default()));
        let indeces = HashMap::new();
        Self { values, indeces }
    }

    fn heapify_down(&mut self, index: usize) {
        let left_index = 2 * index;
        let right_index = 2 * index + 1;
        if let Some(curr_node) = self.values.get(index) {
            match (self.values.get(left_index), self.values.get(right_index)) {
                (None, None) => {}
                (Some(left_node), None) => {
                    self.swap_both(
                        (&left_node.value.clone(), left_index),
                        (&curr_node.value.clone(), index),
                    );
                }
                (Some(left_node), Some(right_node)) => {
                    if left_node.priority < right_node.priority {
                        self.swap_both(
                            (&left_node.value.clone(), left_index),
                            (&curr_node.value.clone(), index),
                        );
                        self.heapify_down(left_index);
                    } else {
                        self.swap_both(
                            (&right_node.value.clone(), right_index),
                            (&curr_node.value.clone(), index),
                        );
                        self.heapify_down(right_index);
                    }
                }
                (None, Some(_)) => panic!("bad heap shape"),
            };
        }
    }

    fn pop(&mut self) -> Option<HeapNode<DirectionalCoord, Coord>> {
        if self.values.len() < 2 {
            return None;
        }
        let original_len = self.values.len();
        let result = self.swap_start_and_end();
        self.heapify_down(1);

        let val = self.indeces.remove(&result.value);
        assert!(val.is_some());

        assert_eq!(self.values.len(), original_len - 1);
        Some(result)
    }

    // Only need to set one index because the other value is removed
    fn swap_start_and_end(&mut self) -> HeapNode<DirectionalCoord, Coord> {
        let result = self.values.swap_remove(1);
        if let Some(first_node) = self.values.get(1) {
            let index_to_update = self
                .indeces
                .get_mut(&first_node.value)
                .expect("should have key at this point");
            *index_to_update = 1;
        }
        result
    }

    // Update both indeces cause they're both real values
    fn swap_both(&mut self, first: (&DirectionalCoord, usize), second: (&DirectionalCoord, usize)) {
        let first_index_to_update = self
            .indeces
            .get_mut(first.0)
            .expect("should have key at this point");
        *first_index_to_update = second.1;
        let second_index_to_update = self
            .indeces
            .get_mut(second.0)
            .expect("should have key at thsi point");
        *second_index_to_update = first.1;
        self.values.swap(first.1, second.1);
    }

    fn heapify_up(&mut self, index: usize) {
        if index == 1 {
            return;
        }
        let parent_index = index / 2;
        let current_node = self
            .values
            .get(index)
            .expect("has to be value at current index");
        match self.values.get(parent_index) {
            Some(parent_node) => {
                if current_node.priority < parent_node.priority {
                    self.swap_both(
                        (&parent_node.value.clone(), parent_index),
                        (&current_node.value.clone(), index),
                    );
                    self.heapify_up(parent_index);
                }
            }
            None => panic!("shouldn't have invalid parent when heapifying up"),
        }
    }

    fn push(&mut self, node: HeapNode<DirectionalCoord, Coord>) {
        self.values.push(node.clone());
        if self
            .indeces
            .insert(node.value, self.values.len() - 1)
            .is_some()
        {
            panic!("already inserted this key");
        }
        self.heapify_up(self.values.len() - 1);
    }

    fn attempt_decrement_key(
        &mut self,
        key: &DirectionalCoord,
        new_priority: usize,
    ) -> DecrementKeyResult {
        if let Some(index) = self.indeces.get(key) {
            let priority_to_update = self
                .values
                .get_mut(*index)
                .expect("provided index must be correct");
            assert!(priority_to_update.value == *key);
            if new_priority < priority_to_update.priority {
                priority_to_update.priority = new_priority;
                self.heapify_up(*index);
                return DecrementKeyResult::SuccessfullyDecremented;
            } else if new_priority == priority_to_update.priority {
                return DecrementKeyResult::NoDecrementEqual;
            }
            return DecrementKeyResult::NoDecrementTooSmall;
        }
        return DecrementKeyResult::NoDecrementKeyDoesNotExist;
    }

    // TODO: This is not something the binary heap should have knowledge of. The history struct
    // value should really just be a part of the `value` of the heap node not be its own thing
    // but oh well for now
    fn update_history(&mut self, key: &DirectionalCoord, new_history: Vec<Coord>) {
        let index = self.indeces.get(key).expect("provided key must exist");
        self.values[*index].history = new_history;
    }

    fn add_to_history(&mut self, key: &DirectionalCoord, mut history: Vec<Coord>) {
        let index = self.indeces.get(key).expect("provided key must exist");
        self.values[*index].history.append(&mut history);
    }

    pub fn print_binary_tree(&self) {
        println!("===START===");
        for (idx, value) in self.values.iter().enumerate() {
            println!(
                "IDX: {idx}, priority: {} value: {:?}",
                value.priority, value.value
            );
        }
        let mut keys = self.indeces.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            println!(
                "key: {:?}, index: {:?}",
                key,
                self.indeces.get(key).expect("has to exist")
            );
        }
        println!("====END====");
    }
}

enum DecrementKeyResult {
    NoDecrementKeyDoesNotExist,
    NoDecrementTooSmall,
    NoDecrementEqual,
    SuccessfullyDecremented,
}

mod test_heap {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::{
        Coord, DecrementKeyResult, Direction, DirectionalCoord, HeapNode, IndexedBinaryHeap,
    };

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn with_priority(priority: usize) -> HeapNode<DirectionalCoord, Coord> {
        let count = COUNTER.fetch_add(1, Ordering::SeqCst);
        HeapNode::with_priority(((count, count), Direction::Up).into(), priority, Vec::new())
    }

    #[test]
    fn test_heap_basic() {
        let mut heap = IndexedBinaryHeap::new();
        heap.push(with_priority(5));
        heap.print_binary_tree();
        heap.push(with_priority(3));
        heap.push(with_priority(4));
        heap.print_binary_tree();
        heap.push(with_priority(2));
        heap.print_binary_tree();
        heap.push(with_priority(15));
        heap.push(with_priority(15));
        heap.push(with_priority(10));
        heap.push(with_priority(10));
        heap.push(with_priority(10));
        heap.print_binary_tree();
        heap.push(with_priority(4));
        heap.print_binary_tree();
        let two = heap.pop().unwrap();
        assert_eq!(2, two.priority);
        heap.print_binary_tree();
        let three = heap.pop().unwrap();
        assert_eq!(3, three.priority);
        let four = heap.pop().unwrap();
        assert_eq!(4, four.priority);
        let four = heap.pop().unwrap();
        assert_eq!(4, four.priority);
        heap.print_binary_tree();
    }

    #[test]
    fn test_heap_decrement_key() {
        let mut heap = IndexedBinaryHeap::new();
        heap.push(with_priority(5));
        heap.push(with_priority(3));
        heap.push(with_priority(4));
        heap.push(with_priority(2));
        heap.push(with_priority(15));
        heap.push(with_priority(15));
        heap.push(with_priority(10));
        heap.push(with_priority(10));
        let node_to_track = with_priority(15);
        heap.push(node_to_track.clone());
        heap.print_binary_tree();
        println!("node_to_track: {:?}", node_to_track);
        assert!(!matches!(
            heap.attempt_decrement_key(&node_to_track.value, 20),
            DecrementKeyResult::SuccessfullyDecremented
        ));
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_decrement_key(&node_to_track.value, 13),
            DecrementKeyResult::SuccessfullyDecremented
        ));
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_decrement_key(&node_to_track.value, 4),
            DecrementKeyResult::SuccessfullyDecremented
        ));
        heap.print_binary_tree();
        let value = heap.pop().expect("has to exist");
        assert_eq!(2_usize, value.priority);
        heap.print_binary_tree();
    }
}

fn print_grid_fill(locations: &HashSet<Coord>, grid: Vec<Vec<char>>) {
    for (row_idx, row) in grid.into_iter().enumerate() {
        for (col_idx, col) in row.into_iter().enumerate() {
            let coord = (row_idx, col_idx).into();
            if locations.contains(&coord) {
                print!("O");
            } else {
                print!("{}", col);
            }
        }
        println!("");
    }
}

fn print_grid_with_costs(heap: IndexedBinaryHeap<DirectionalCoord, Coord>, grid: Vec<Vec<char>>) {
    let costs_so_far = heap
        .values
        .into_iter()
        .filter(|node| node.priority < usize::MAX)
        .map(|node| (node.value, node.priority))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<Coord, (usize, Direction)>, val| {
                if let Some(curr) = acc.get(&val.0 .0) {
                    if val.1 < curr.0 {
                        acc.insert(val.0 .0, (val.1, val.0 .1));
                    }
                } else {
                    acc.insert(val.0 .0, (val.1, val.0 .1));
                }

                acc
            },
        );

    print!("  ");
    for i in 0..grid[0].len() {
        print!("[ {:0>2}  ]", i);
    }
    println!();
    for (row_idx, row) in grid.iter().enumerate() {
        print!("{:0>2}", row_idx);
        for (col_idx, space) in row.iter().enumerate() {
            let coord = (row_idx, col_idx).into();
            if let Some((cost, direction)) = costs_so_far.get(&coord) {
                let direction_char = match direction {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Right => '>',
                    Direction::Left => '<',
                };
                print!("[{:0>4}{direction_char}]", cost);
            } else if *space == '.' {
                print!("[.....]");
            } else if *space == '#' {
                print!("[#####]");
            } else if *space == 'S' {
                print!("[SSSSS]");
            } else if *space == 'E' {
                print!("[EEEEE]");
            } else {
                panic!("error with printing");
            }
        }
        println!("");
    }
}

fn find_start_and_end(grid: &Vec<Vec<char>>) -> Option<(Coord, Coord)> {
    let mut start = None;
    let mut end = None;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, space) in row.iter().enumerate() {
            match space {
                'S' => start = Some((row_idx, col_idx)),
                'E' => end = Some((row_idx, col_idx)),
                _ => {}
            }
        }
    }

    match (start, end) {
        (Some(start), Some(end)) => Some((start.into(), end.into())),
        _ => None,
    }
}

fn get_neighboring_nodes(coord: &Coord, direction: &Direction, grid: &Vec<Vec<char>>) -> Vec<Edge> {
    let mut result = Vec::with_capacity(4);
    if Direction::Down.is_open(coord, grid) {
        if let Some(cost) = direction.cost(Direction::Down) {
            let edge = ((coord.0 + 1, coord.1).into(), Direction::Down, cost).into();
            result.push(edge);
        }
    }
    if Direction::Up.is_open(coord, grid) {
        if let Some(cost) = direction.cost(Direction::Up) {
            let edge = ((coord.0 - 1, coord.1).into(), Direction::Up, cost).into();
            result.push(edge);
        }
    }
    if Direction::Left.is_open(coord, grid) {
        if let Some(cost) = direction.cost(Direction::Left) {
            let edge = ((coord.0, coord.1 - 1).into(), Direction::Left, cost).into();
            result.push(edge);
        }
    }
    if Direction::Right.is_open(coord, grid) {
        if let Some(cost) = direction.cost(Direction::Right) {
            let edge = ((coord.0, coord.1 + 1).into(), Direction::Right, cost).into();
            result.push(edge);
        }
    }
    result
}

impl TryFrom<Vec<Vec<char>>> for Graph {
    type Error = ();
    fn try_from(grid: Vec<Vec<char>>) -> Result<Self, ()> {
        if let Some((start, end)) = find_start_and_end(&grid) {
            let mut graph = Self {
                start: start.clone(),
                end: end.clone(),
                edges: HashMap::new(),
            };

            //println!("start: {:?}, end: {:?}", start, end);

            let mut to_visit = vec![DirectionalCoord(start.clone(), Direction::Right)];

            while !to_visit.is_empty() {
                let node = to_visit.remove(0);
                // We do the init key first to make sure the end ends up as a key of edges
                if !graph.init_key(node.clone()) || node.0 == graph.end {
                    continue;
                }
                let edges = get_neighboring_nodes(&node.0, &node.1, &grid);
                for edge in edges {
                    to_visit.push(edge.0.clone());
                    graph.add_edge(node.clone(), edge);
                }
            }

            return Ok(graph);
        }
        //println!("No start and end");
        Err(())
    }
}

impl Puzzle for Day16 {
    fn puzzle_1(contents: String) {
        let grid = read_grid(contents);
        let graph = Graph::try_from(grid.clone()).expect("has to be a graph or there's an issue");
        let cost = graph.shortest_path(grid.clone());
        println!(
            "The cost is: {} and the number of locations visited is {}",
            cost.0,
            cost.1.len()
        );
        println!("all locations: {:?}", cost.1);
    }

    fn puzzle_2(contents: String) {
        Self::puzzle_1(contents);
    }
}
