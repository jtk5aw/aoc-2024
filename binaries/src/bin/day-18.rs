use std::collections::{HashSet, VecDeque};

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

#[derive(Clone)]
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
        if let Err(reason) = result.dfs() {
            panic!("Failed to do the initial dfs because: {reason}");
        }
        result
    }

    fn dfs(&mut self) -> Result<(), String> {
        loop {
            let new_path_opt = self.dfs_helper(self.upper_bounds);
            match new_path_opt {
                Some(new_path) => {
                    self.ordered_path = new_path.clone();
                    self.path_set = HashSet::from_iter(new_path);
                    break;
                }
                None => self.cut_in_half()?,
            }
        }
        Ok(())
    }

    fn dfs_helper(&self, target: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let mut potential_new_path = self.ordered_path.clone();
        let mut visited = HashSet::new();

        let depth = self.ordered_path.len();
        let starting_point = CoordWithDepth::new(self.ordered_path[depth - 1], depth);
        let initial_neighbors = self
            .expand_search(starting_point.coord, &visited, &self.blocked)
            .into_iter()
            .map(|coord| CoordWithDepth::new(coord, potential_new_path.len()))
            .collect::<Vec<_>>();
        let mut to_visit = Vec::from_iter(initial_neighbors);

        while let Some(node) = to_visit.pop() {
            visited.insert(node.coord);
            potential_new_path.truncate(node.depth);
            assert!(node.coord != potential_new_path[potential_new_path.len() - 1]);
            potential_new_path.push(node.coord);
            if node.coord == target {
                return Some(potential_new_path);
            }
            assert!(potential_new_path.len() >= self.ordered_path.len());
            let mut neighbors = self
                .expand_search(node.coord, &visited, &self.blocked)
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
        blocked: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(4);

        let mut try_add_coord = |coord| {
            if !visited.contains(&coord) && !blocked.contains(&coord) {
                result.push(coord);
            }
        };

        if coord.0 > 0 {
            try_add_coord((coord.0 - 1, coord.1));
        }
        if coord.1 > 0 {
            try_add_coord((coord.0, coord.1 - 1));
        }
        if coord.0 < self.upper_bounds.0 {
            try_add_coord((coord.0 + 1, coord.1));
        }
        if coord.1 < self.upper_bounds.1 {
            try_add_coord((coord.0, coord.1 + 1));
        }

        result
    }

    fn block_coord(&mut self, coord: (usize, usize)) {
        self.blocked.insert(coord);
    }

    fn in_path(&self, coord: &(usize, usize)) -> bool {
        self.path_set.contains(coord)
    }

    fn cut_in_half(&mut self) -> Result<(), String> {
        if self.ordered_path.len() <= 1 {
            return Err("can't cut ordered path in half with length <= 1".to_string());
        }
        let halfway_point = self.ordered_path[self.ordered_path.len() / 2].clone();
        self.cut_path(&halfway_point);
        Ok(())
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
    }

    fn puzzle_2(contents: String) {
        let mut coords = coords_vec(contents).into_iter();
        let mut path = Path::new((0, 0), (70, 70));
        while let Some(coord) = coords.next() {
            println!("blocking: ({}, {})", coord.0, coord.1);
            path.block_coord(coord.clone());
            if path.in_path(&coord) {
                path.cut_path(&coord);
                if let Err(reason) = path.dfs() {
                    println!(
                        "Failed to find a solution at coord ({},{}) due to: {reason}",
                        coord.0, coord.1
                    );
                    break;
                }
            }
            print_grid_from_path(path.clone());
        }
    }
}

fn print_grid_from_path(path: Path) {
    println!("===Start===");
    for row_idx in 0..=path.upper_bounds.0 {
        for col_idx in 0..=path.upper_bounds.1 {
            let coord = (row_idx, col_idx);
            match (path.blocked.contains(&coord), path.in_path(&coord)) {
                (true, true) => panic!("This shouldn't happen"),
                (false, true) => print!("O"),
                (true, false) => print!("#"),
                (false, false) => print!("."),
            };
        }
        println!("");
    }
    println!("===End===");
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
