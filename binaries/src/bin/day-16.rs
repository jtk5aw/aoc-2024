use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

use helpers::{read_grid, Puzzle};

struct Day16;

fn main() {
    Day16::run()
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
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
            return Some(0);
        }
        return Some(1000);
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
            Direction::Up if grid[coord.0 - 1][coord.1] == '.' => true,
            Direction::Down if grid[coord.0 + 1][coord.1] == '.' => true,
            Direction::Right if grid[coord.0][coord.1 + 1] == '.' => true,
            Direction::Left if grid[coord.0][coord.1 - 1] == '.' => true,
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
struct Coord(usize, usize);

impl From<(usize, usize)> for Coord {
    fn from(val: (usize, usize)) -> Self {
        Self(val.0, val.1)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
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

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct HeapNode<T: Hash + Ord> {
    priority: usize,
    value: T,
}

impl<T: Eq + PartialEq + Hash + Ord> HeapNode<T> {
    fn new(value: T) -> Self {
        HeapNode {
            priority: usize::MAX,
            value,
        }
    }

    fn with_priority(value: T, priority: usize) -> Self {
        HeapNode { priority, value }
    }
}

impl<T: Hash + Eq + Ord> Ord for HeapNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Flips ordering for min-heap
        other
            .priority
            .cmp(&self.priority)
            .then(other.value.cmp(&self.value))
    }
}

impl<T: Hash + Eq + Ord> PartialOrd for HeapNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    fn init_key(&mut self, key: DirectionalCoord) {
        match self.edges.get(&key) {
            Some(_) => panic!("never call init key when values already exist"),
            None => {
                self.edges.insert(key, Vec::new());
            }
        }
    }

    fn add_edge(&mut self, key: DirectionalCoord, edge: Edge) {
        self.edges.entry(key).or_insert_with(Vec::new).push(edge);
    }

    fn shortest_path(self, grid: Vec<Vec<char>>) -> usize {
        let mut unvisited = BinaryHeap::new();
        for key in self.edges.keys() {
            if key.0 == self.start {
                unvisited.push(HeapNode::with_priority(key.clone(), 0));
            } else {
                unvisited.push(HeapNode::new(key.clone()));
            }
        }

        let mut final_priority = usize::MAX;
        let mut i = 0;
        while let Some(node) = unvisited.pop() {
            println!("popped: {:?}", node);
            if self.final_edge_coords.contains(&node.value) {
                final_priority = if final_priority < node.priority {
                    final_priority
                } else {
                    node.priority
                };
                continue;
            }

            assert_ne!(node.priority, usize::MAX);

            let mut neighbors = self
                .edges
                .get(&node.value)
                .expect("every unvisited node should have edges")
                .iter()
                .map(|edge| {
                    println!("edge: {:?}", edge);
                    (
                        edge.0.clone(),
                        HeapNode::with_priority(edge.0.clone(), node.priority + edge.1),
                    )
                })
                .collect::<HashMap<_, _>>();

            let mut to_insert = Vec::with_capacity(neighbors.keys().len());
            unvisited.retain(
                |unvisited_node| match neighbors.remove(&unvisited_node.value) {
                    Some(neighbor) if neighbor.priority < unvisited_node.priority => {
                        to_insert.push(neighbor);
                        false
                    }
                    _ => true,
                },
            );
            for node in to_insert {
                println!("inserting: {:?}", node);
                unvisited.push(node);
            }
            println!("{i}");
            print_grid_with_costs(unvisited.clone(), grid.clone());
            i += 1;
        }
        panic!("Should always find the end");
    }
}

fn print_grid_with_costs(heap: BinaryHeap<HeapNode<DirectionalCoord>>, grid: Vec<Vec<char>>) {
    let costs_so_far = heap
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

fn get_neighboring_nodes(
    coord: &Coord,
    direction: &Direction,
    grid: &Vec<Vec<char>>,
) -> Vec<(Edge, Direction)> {
    let mut result = Vec::with_capacity(4);
    if Direction::Down.is_open(coord, grid) {
        if let Some(cost) = direction.cost(Direction::Down) {
            let edge = ((coord.0 + 1, coord.1).into(), Direction::Down, cost).into();
            result.push((edge, Direction::Down));
        }
    }
    if Direction::Up.is_open(coord, grid) {
        if let Some(cost) = direction.cost(Direction::Up) {
            let edge = ((coord.0 - 1, coord.1).into(), Direction::Up, cost).into();
            result.push((edge, Direction::Up));
        }
    }
    if Direction::Left.is_open(coord, grid) {
        if let Some(cost) = direction.cost(Direction::Left) {
            let edge = ((coord.0, coord.1 - 1).into(), Direction::Left, cost).into();
            result.push((edge, Direction::Left));
        }
    }
    if Direction::Right.is_open(coord, grid) {
        if let Some(cost) = direction.cost(Direction::Right) {
            let edge = ((coord.0, coord.1 + 1).into(), Direction::Right, cost).into();
            result.push((edge, Direction::Right));
        }
    }
    result
}

fn begin_edges(
    from_coord: &Coord,
    start_coord: &DirectionalCoord,
    direction: Direction,
    grid: &Vec<Vec<char>>,
) -> Vec<(Edge, Direction, DirectionalCoord)> {
    get_neighboring_nodes(&from_coord, &direction, &grid)
        .into_iter()
        .map(|val| (val.0, val.1, start_coord.clone()))
        .collect::<Vec<_>>()
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

            println!("start: {:?}, end: {:?}", start, end);

            let mut edge_beginnings = Vec::new();
            edge_beginnings.append(&mut begin_edges(
                &start,
                &(start.clone(), Direction::Right).into(),
                Direction::Right,
                &grid,
            ));
            let mut visited = HashSet::new();

            while !edge_beginnings.is_empty() {
                let (mut edge, direction, start_node) = edge_beginnings.remove(0);
                println!(
                    "edge: {:?}, direction: {:?}, start_node: {:?}",
                    edge, direction, start_node
                );
                if !visited.insert((edge.0.clone(), direction.clone())) || graph.end == edge.0 .0 {
                    graph.add_edge(key, edge);
                    continue;
                }
                let mut create_edge =
                    CreateEdgeIterator::new(edge.0 .0.clone(), direction.clone(), &grid);
                let mut last_coord = None;
                let sides = direction.sides();

                graph.init_key(edge.0.clone());
                while let Some(next_coord) = create_edge.next() {
                    println!("next_coord: {:?}", next_coord);
                    last_coord = Some(next_coord.into());
                    edge.1 += 1;

                    if Coord::from(next_coord.clone()) == graph.end {
                        graph.add_edge(start_node.clone(), edge.clone());
                        graph.final_edge_coords.insert(start_node);
                        break;
                    }
                    if sides[0].is_open(&next_coord.into(), &grid)
                        || sides[1].is_open(&next_coord.into(), &grid)
                    {
                        graph.add_edge(start_node, edge.clone());
                        break;
                    }
                }
                let last_coord = last_coord.ok_or_else(|| {
                    println!("Failed to set last coord. shouldn't happen");
                    ()
                })?;
                edge_beginnings.append(&mut begin_edges(
                    &last_coord,
                    &edge.0,
                    direction.clone(),
                    &grid,
                ));
            }

            return Ok(graph);
        }
        println!("No start and end");
        Err(())
    }
}

impl Puzzle for Day16 {
    fn puzzle_1(contents: String) {
        let grid = read_grid(contents);
        let graph = Graph::try_from(grid.clone()).expect("has to be a graph or there's an issue");
        let mut keys = graph.edges.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            println!("key: {:?} edges: {:?}", key, graph.edges.get(&key));
        }
        let cost = graph.shortest_path(grid.clone());
        println!("The cost is: {cost}");
    }

    fn puzzle_2(contents: String) {
        todo!()
    }
}
