use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    iter::Take,
    str::Lines,
};

use helpers::Puzzle;

struct Day18;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Space {
    count: Option<usize>,
    kind: SpaceKind,
}

impl Space {
    fn blocked(&mut self) -> bool {
        if !matches!(self.kind, SpaceKind::Blocked) {
            self.kind = SpaceKind::Blocked;
            true
        } else {
            false
        }
    }

    fn should_visit(&self) -> bool {
        match self.kind {
            SpaceKind::Empty => self.count.is_none(),
            SpaceKind::Blocked => false,
        }
    }

    fn set_count(&mut self, count: usize) {
        self.count = Some(count);
    }
}

impl Default for Space {
    fn default() -> Self {
        Self {
            count: None,
            kind: SpaceKind::Empty,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum SpaceKind {
    Empty,
    Blocked,
}

fn coords_vec(contents: String) -> Vec<(usize, usize)> {
    contents
        .lines()
        .map(|line| line.split_once(",").expect("has to have a comma"))
        .map(|(x, y)| {
            (
                y.parse().expect("has to be a usize"),
                x.parse().expect("has to be a usize"),
            )
        })
        .collect::<Vec<_>>()
}

fn read_coords(coords: impl Iterator<Item = (usize, usize)>) -> (Vec<Vec<Space>>, usize) {
    let mut num_blocked = 0;
    let result = coords.fold(
        vec![vec![Space::default(); 71]; 71],
        |mut grid, (row, col): (usize, usize)| {
            if grid[row][col].blocked() {
                num_blocked += 1;
            }
            grid
        },
    );

    (result, num_blocked)
}

fn expand_search(grid: &mut Vec<Vec<Space>>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);

    if coords.0 > 0 && grid[coords.0 - 1][coords.1].should_visit() {
        result.push((coords.0 - 1, coords.1));
    }
    if coords.1 > 0 && grid[coords.0][coords.1 - 1].should_visit() {
        result.push((coords.0, coords.1 - 1));
    }
    if coords.0 < grid.len() - 1 && grid[coords.0 + 1][coords.1].should_visit() {
        result.push((coords.0 + 1, coords.1));
    }
    if coords.1 < grid[coords.0].len() - 1 && grid[coords.0][coords.1 + 1].should_visit() {
        result.push((coords.0, coords.1 + 1));
    }

    result
}

fn find_shortest_path(grid: &mut Vec<Vec<Space>>) -> Result<usize, ()> {
    let mut to_search = VecDeque::new();
    grid[0][0].set_count(0);
    to_search.push_back((0, 0));
    while let Some(coords) = to_search.pop_front() {
        let current_count = grid[coords.0][coords.1]
            .count
            .expect("current space has to have a count");
        if coords.0 == grid.len() - 1 && coords.1 == grid[coords.0].len() - 1 {
            return Ok(current_count);
        }
        expand_search(grid, coords)
            .into_iter()
            .for_each(|(row, col)| {
                grid[row][col].set_count(current_count + 1);
                to_search.push_back((row, col));
            });
        print_grid(grid.clone());
    }
    Err(())
}

struct Path {
    upper_bounds: (usize, usize),
    blocked: HashSet<(usize, usize)>,
    ordered_path: Vec<(usize, usize)>,
    path_set: HashSet<(usize, usize)>,
}

struct CoordWithDepth {
    coord: (usize, usize),
    depth: usize,
}

impl CoordWithDepth {
    fn new(coord: (usize, usize), depth: usize) -> Self {
        Self { coord, depth }
    }
}

impl Path {
    fn new(starting_coord: (usize, usize), upper_bounds: (usize, usize)) -> Self {
        let ordered_path = vec![starting_coord];
        let path_set = HashSet::from_iter(ordered_path.clone());
        let blocked = HashSet::new();

        let mut result = Self {
            upper_bounds,
            ordered_path,
            path_set,
            blocked,
        };
        result.dfs((upper_bounds.0 - 1, upper_bounds.1 - 1));
        result
    }

    // TODO TODO TODO: move this out of the struct cause it should ust be a helper method
    // a dfs that is mut can be in here but the business logic should be elsewhere
    // thats why there's returns that don't match
    fn dfs(&mut self, target: (usize, usize)) {
        let mut potential_new_path = self.ordered_path.clone();
        let mut to_visit = Vec::new();
        let mut visited = HashSet::new();

        let depth = self.ordered_path.len();
        let starting_point = CoordWithDepth::new(self.ordered_path[depth - 1], depth);
        to_visit.push(starting_point);

        while let Some(node) = to_visit.pop() {
            visited.insert(node.coord);
            potential_new_path.truncate(node.depth);
            potential_new_path.push(node.coord);
            if node.coord == target {
                return Some(potential_new_path);
            }
            assert!(potential_new_path.len() >= self.ordered_path.len());
            let mut neighbors = self
                .expand_search(node.coord, &visited)
                .into_iter()
                .map(|coord| CoordWithDepth::new(coord, potential_new_path.len()))
                .collect::<Vec<_>>();
            to_visit.append(&mut neighbors);
        }
        None
    }

    fn expand_search(
        &self,
        coord: (usize, usize),
        visited: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(4);

        if coord.0 > 0 && !visited.contains(&(coord.0 - 1, coord.1)) {
            result.push((coord.0 - 1, coord.1));
        }
        if coord.1 > 0 && !visited.contains(&(coord.0, coord.1 - 1)) {
            result.push((coord.0, coord.1 - 1));
        }
        if coord.0 < self.upper_bounds.0 && !visited.contains(&(coord.0 + 1, coord.1)) {
            result.push((coord.0 + 1, coord.1));
        }
        if coord.1 < self.upper_bounds.1 && !visited.contains(&(coord.0, coord.1 + 1)) {
            result.push((coord.0, coord.1 + 1));
        }

        result
    }

    fn in_path(&self, coord: &(usize, usize)) -> bool {
        self.path_set.contains(coord)
    }

    fn cut_in_half(&mut self) {
        let halfway_point = self.ordered_path[self.ordered_path.len() / 2].clone();
        self.cut_path(&halfway_point);
    }

    fn cut_path(&mut self, coord: &(usize, usize)) {
        let cut_from = self
            .ordered_path
            .iter()
            .position(|val| val == coord)
            .expect("only cut when value is known to be in the path");
        for idx in cut_from..self.ordered_path.len() {
            self.path_set.remove(&self.ordered_path[idx]);
        }
        self.ordered_path.truncate(cut_from);
    }
}

const NUM_TO_TAKE_PUZZLE_1: usize = 1024;

impl Puzzle for Day18 {
    fn puzzle_1(contents: String) {
        let coords_vec = coords_vec(contents);
        let (mut grid, num_blocked) =
            read_coords(coords_vec.into_iter().take(NUM_TO_TAKE_PUZZLE_1));
        // For the test puzzle we lop off a lot rows and columns
        if num_blocked < NUM_TO_TAKE_PUZZLE_1 {
            grid.truncate(7);
            for row in grid.iter_mut() {
                row.truncate(7);
            }
        }

        let steps = find_shortest_path(&mut grid).expect("has to find end");

        println!("shortest path has {steps} steps");
    }

    fn puzzle_2(contents: String) {
        let coords = coords_vec(contents);
        while let Some(coord) = coords.iter().next() {}
    }
}

fn print_grid(grid: Vec<Vec<Space>>) {
    println!("===Start===");
    for row in grid {
        for space in row {
            match (space.count, space.kind) {
                (Some(_), SpaceKind::Empty) => print!("O"),
                (None, SpaceKind::Empty) => print!("."),
                (_, SpaceKind::Blocked) => print!("#"),
            }
        }
        println!("");
    }
    println!("===End===");
}

fn main() {
    Day18::run()
}
