use std::collections::HashSet;

use helpers::{read_grid, Puzzle};

fn main() {
    Day15::run();
}

struct Day15;

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn update_coords(&self, coords: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (coords.0 - 1, coords.1),
            Self::Down => (coords.0 + 1, coords.1),
            Self::Left => (coords.0, coords.1 - 1),
            Self::Right => (coords.0, coords.1 + 1),
        }
    }

    fn reverse_direction(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn reachable(&self, start_coord: (usize, usize), end_coord: (usize, usize)) -> bool {
        match self {
            Direction::Up if start_coord.0 >= end_coord.0 => true,
            Direction::Down if start_coord.0 <= end_coord.0 => true,
            Direction::Left if start_coord.1 >= end_coord.1 => true,
            Direction::Right if start_coord.1 <= end_coord.1 => true,
            _ => false,
        }
    }

    fn is_vertical(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            _ => false,
        }
    }

    fn sort_boxes(&self, boxes: HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut result = Vec::from_iter(boxes);
        result.sort_by(|a, b| match self {
            Direction::Up | Direction::Left => a.cmp(&b),
            Direction::Down | Direction::Right => b.cmp(&a),
        });
        result
    }
}

impl TryFrom<&char> for Direction {
    type Error = ();

    fn try_from(value: &char) -> Result<Self, ()> {
        match value {
            '^' => Ok(Self::Up),
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            'v' => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
enum Space {
    Box,
    Empty,
    Robot,
    Edge,
}

#[derive(Clone, Debug)]
enum DoubleSpace {
    LeftBox,
    RightBox,
    Empty,
    Robot,
    Edge,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Edge,
            'O' => Self::Box,
            '@' => Self::Robot,
            '.' => Self::Empty,
            _ => panic!("unexpected character seen: {value}"),
        }
    }
}

trait IsEdge {
    fn is_edge(&self) -> bool;
}

impl IsEdge for Space {
    fn is_edge(&self) -> bool {
        matches!(self, Space::Edge)
    }
}

impl IsEdge for DoubleSpace {
    fn is_edge(&self) -> bool {
        matches!(self, DoubleSpace::Edge)
    }
}

trait PrintSpace {
    fn print_space(&self);
}

impl PrintSpace for Space {
    fn print_space(&self) {
        match self {
            Space::Box => print!("O"),
            Space::Empty => print!("."),
            Space::Robot => print!("@"),
            Space::Edge => print!("#"),
        }
    }
}

impl PrintSpace for DoubleSpace {
    fn print_space(&self) {
        match self {
            DoubleSpace::LeftBox => print!("["),
            DoubleSpace::RightBox => print!("]"),
            DoubleSpace::Empty => print!("."),
            DoubleSpace::Robot => print!("@"),
            DoubleSpace::Edge => print!("#"),
        }
    }
}

trait Scorable {
    fn scorable(&self) -> bool;
}

impl Scorable for Space {
    fn scorable(&self) -> bool {
        matches!(self, Space::Box)
    }
}

impl Scorable for DoubleSpace {
    fn scorable(&self) -> bool {
        matches!(self, DoubleSpace::LeftBox)
    }
}

struct CoordSearcher {
    current_coords: Option<(usize, usize)>,
    target_coords: (usize, usize),
    direction: Direction,
}

impl CoordSearcher {
    fn try_new(
        start_coords: (usize, usize),
        target_coords: (usize, usize),
        direction: Direction,
    ) -> Result<Self, ()> {
        if !direction.reachable(start_coords, target_coords) {
            println!(
                "failed to create searcher in {:?} with start_coords ({}, {}) and target_coords ({}, {})",
                direction, start_coords.0, start_coords.1, target_coords.0, target_coords.1
            );
            return Err(());
        }
        Ok(Self {
            current_coords: Some(start_coords),
            target_coords,
            direction,
        })
    }
}

// Requires that the start_coords fully line up with the target_coords. Only checks that the
// direction of movement can ~possibly~ get there. Doesn't actually check that they're in line
// If they aren't, an out of bounds exception will be thrown or this will go on forever
impl Iterator for CoordSearcher {
    type Item = ((usize, usize), bool);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_coords;

        let mut is_target = false;
        if let Some(coords) = current {
            self.current_coords = if coords == self.target_coords {
                is_target = true;
                None
            } else {
                Some(self.direction.update_coords(coords))
            }
        }

        current.map(|coords| (coords, is_target))
    }
}

struct EdgeSearcher<'a, T: IsEdge> {
    current_coords: Option<(usize, usize)>,
    direction: Direction,
    grid: &'a Vec<Vec<T>>,
}

impl<'a, T: IsEdge> EdgeSearcher<'a, T> {
    fn new(start_coords: (usize, usize), direction: Direction, grid: &'a Vec<Vec<T>>) -> Self {
        Self {
            current_coords: Some(start_coords),
            direction,
            grid,
        }
    }
}

impl<'a, T: IsEdge> Iterator for EdgeSearcher<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_coords;

        if let Some(coords) = current {
            let potential_coords = self.direction.update_coords(coords);
            self.current_coords = if self.grid[potential_coords.0][potential_coords.1].is_edge() {
                None
            } else {
                Some(potential_coords)
            }
        };

        current
    }
}

struct EdgeSearcherWithTombstone<'a> {
    searcher: EdgeSearcher<'a, DoubleSpace>,
    is_tombstoned: bool,
}

impl<'a> EdgeSearcherWithTombstone<'a> {
    fn new(searcher: EdgeSearcher<'a, DoubleSpace>) -> Self {
        EdgeSearcherWithTombstone {
            searcher,
            is_tombstoned: false,
        }
    }

    fn tombstone(&mut self) {
        self.is_tombstoned = true;
    }
}

struct WideningEdgeSearcher<'a> {
    current_coords_list: Option<HashSet<(usize, usize)>>,
    found_coords: HashSet<(usize, usize)>,
    searchers: Vec<EdgeSearcherWithTombstone<'a>>,
    direction: Direction,
    grid: &'a Vec<Vec<DoubleSpace>>,
}

impl<'a> WideningEdgeSearcher<'a> {
    fn new(
        start_coords: (usize, usize),
        direction: Direction,
        grid: &'a Vec<Vec<DoubleSpace>>,
    ) -> Self {
        let mut searcher = EdgeSearcherWithTombstone::new(EdgeSearcher::new(
            start_coords,
            direction.clone(),
            grid,
        ));
        // Makes it so the first space isn't returned twice
        searcher.searcher.next();
        Self {
            searchers: vec![searcher],
            found_coords: HashSet::new(),
            current_coords_list: Some(HashSet::from_iter(vec![start_coords])),
            direction,
            grid,
        }
    }
}

// TODO TODO TODO: Riht now I extend every searcher forever. The searcher actually needs to stop
// once it finds an empty space. From there it no longer needs to continue

impl<'a> Iterator for WideningEdgeSearcher<'a> {
    type Item = HashSet<(usize, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current_coords_list.clone();

        if let Some(_) = current.as_ref() {
            let potential_coords_list = self
                .searchers
                .iter_mut()
                .enumerate()
                .flat_map(|(idx, searcher)| searcher.searcher.next().map(|val| (idx, val)))
                .collect::<Vec<_>>();

            self.current_coords_list = if potential_coords_list.len() != self.searchers.len() {
                None
            } else {
                let mut new_coords = potential_coords_list
                    .clone()
                    .into_iter()
                    .map(|(_, val)| val)
                    .collect::<Vec<_>>();
                for (searcher_idx, potential_coords) in potential_coords_list {
                    match (
                        &self.grid[potential_coords.0][potential_coords.1],
                        self.direction.is_vertical(),
                    ) {
                        (DoubleSpace::Empty, _) => {
                            self.found_coords.insert(potential_coords);
                            self.searchers[searcher_idx].tombstone();
                        }
                        (DoubleSpace::LeftBox, true) => {
                            let added_coord = (potential_coords.0, potential_coords.1 + 1);
                            let mut new_edge_searcher =
                                EdgeSearcher::new(added_coord, self.direction.clone(), self.grid);
                            new_coords
                                .push(new_edge_searcher.next().expect("first next has to be Some"));
                            self.searchers
                                .push(EdgeSearcherWithTombstone::new(new_edge_searcher));
                        }
                        (DoubleSpace::RightBox, true) => {
                            let added_coord = (potential_coords.0, potential_coords.1 - 1);
                            let mut new_edge_searcher =
                                EdgeSearcher::new(added_coord, self.direction.clone(), self.grid);
                            new_coords
                                .push(new_edge_searcher.next().expect("first next has to be Some"));
                            self.searchers
                                .push(EdgeSearcherWithTombstone::new(new_edge_searcher));
                        }
                        (_, _) => {}
                    }
                }
                self.searchers.retain(|searcher| !searcher.is_tombstoned);
                let mut updated_curr_coords = self.found_coords.clone();
                updated_curr_coords.extend(new_coords.into_iter());
                Some(updated_curr_coords)
            }
        }

        current
    }
}

fn get_moves(row_iter: impl Iterator<Item = char>) -> Vec<Direction> {
    row_iter
        .map(|move_char| {
            (&move_char).try_into().expect(&format!(
                "must be a valid move char at this point: {move_char}",
            ))
        })
        .collect::<Vec<Direction>>()
}

fn get_grid_row(
    row_idx: usize,
    row_iter: impl Iterator<Item = char>,
) -> (Option<(usize, usize)>, Vec<Space>) {
    let mut start_coords = None;
    let grid_row = row_iter
        .enumerate()
        .map(|(col_idx, space_char)| {
            let space = Space::from(space_char);
            if matches!(space, Space::Robot) {
                start_coords = Some((row_idx, col_idx));
            }
            space
        })
        .collect::<Vec<_>>();
    (start_coords, grid_row)
}

fn get_grid_row_2(
    row_idx: usize,
    row_iter: impl Iterator<Item = char>,
) -> (Option<(usize, usize)>, Vec<DoubleSpace>) {
    let mut start_coords = None;
    let mut grid_row = Vec::new();
    for space_char in row_iter {
        let space = Space::from(space_char);
        match space {
            Space::Box => {
                grid_row.push(DoubleSpace::LeftBox);
                grid_row.push(DoubleSpace::RightBox);
            }
            Space::Empty => {
                grid_row.push(DoubleSpace::Empty);
                grid_row.push(DoubleSpace::Empty);
            }
            Space::Robot => {
                start_coords = Some((row_idx, grid_row.len()));
                grid_row.push(DoubleSpace::Robot);
                grid_row.push(DoubleSpace::Empty);
            }
            Space::Edge => {
                grid_row.push(DoubleSpace::Edge);
                grid_row.push(DoubleSpace::Edge);
            }
        }
    }
    (start_coords, grid_row)
}

fn find_empty_space(
    curr_coords: (usize, usize),
    direction: Direction,
    grid: &Vec<Vec<Space>>,
) -> Option<(usize, usize)> {
    let check_coords = curr_coords;
    assert!(matches!(grid[check_coords.0][check_coords.1], Space::Robot));
    let mut searcher_iter = EdgeSearcher::new(check_coords, direction, grid).into_iter();

    let mut empty_space = None;
    while let Some(coords) = searcher_iter.next() {
        if matches!(grid[coords.0][coords.1], Space::Empty) {
            empty_space = Some(coords);
            break;
        }
    }
    empty_space
}

fn boxes_to_move(
    curr_coords: (usize, usize),
    direction: Direction,
    grid: &Vec<Vec<DoubleSpace>>,
) -> Option<HashSet<(usize, usize)>> {
    let check_coords = curr_coords;
    assert!(matches!(
        grid[check_coords.0][check_coords.1],
        DoubleSpace::Robot
    ));
    let mut widening_searcher_iter =
        WideningEdgeSearcher::new(check_coords, direction, grid).into_iter();

    let mut boxes_to_move = HashSet::new();
    // Skip the first robot space
    widening_searcher_iter.next();
    while let Some(coords_list) = widening_searcher_iter.next() {
        let mut all_empty = true;
        for coords in coords_list {
            match &grid[coords.0][coords.1] {
                DoubleSpace::LeftBox => {
                    boxes_to_move.insert(coords);
                    all_empty = false;
                }
                DoubleSpace::RightBox => {
                    boxes_to_move.insert((coords.0, coords.1 - 1));
                    all_empty = false;
                }
                DoubleSpace::Edge | DoubleSpace::Robot => {
                    panic!("Encountered an edge or a robot space where that should not happen");
                }
                DoubleSpace::Empty => {}
            }
        }
        if all_empty {
            return Some(boxes_to_move);
        };
    }
    None
}

fn print_grid<T: PrintSpace + Clone>(grid: Vec<Vec<T>>) {
    for row in grid.clone() {
        for space in row {
            space.print_space()
        }
        println!();
    }
}

fn run_sim(start_coords: (usize, usize), moves: Vec<Direction>, grid: &mut Vec<Vec<Space>>) {
    let mut curr_coords = start_coords;

    for next_move in moves {
        //println!("Current move is: {:?}", next_move);
        //print_grid(grid.clone());

        let empty_space = find_empty_space(curr_coords, next_move.clone(), &grid);

        if let Some(mut move_into_coords) = empty_space {
            let mut coord_searcher = CoordSearcher::try_new(
                move_into_coords,
                curr_coords,
                next_move.reverse_direction(),
            )
            .expect("starting point must be reachable after reversing direction")
            .into_iter();
            while let Some((move_coords, _is_target)) = coord_searcher.next() {
                grid[move_into_coords.0][move_into_coords.1] =
                    grid[move_coords.0][move_coords.1].clone();

                move_into_coords = move_coords;
            }
            grid[curr_coords.0][curr_coords.1] = Space::Empty;
            curr_coords = next_move.update_coords(curr_coords);
        }
    }
}

fn run_sim_2(
    start_coords: (usize, usize),
    moves: Vec<Direction>,
    grid: &mut Vec<Vec<DoubleSpace>>,
) {
    let mut curr_coords = start_coords;

    for next_move in moves {
        //println!("Current move is: {:?}", next_move);
        //print_grid(grid.clone());

        let boxes_to_move_opt = boxes_to_move(curr_coords, next_move.clone(), &grid);

        if let Some(boxes_to_move) = boxes_to_move_opt {
            // Boxes need to be sorted in the right order so moving them doesn't overwrite each
            // other
            let sorted_boxes = next_move.sort_boxes(boxes_to_move);
            // the blocks to move are the coords of the left block
            if sorted_boxes.len() >= 1 {
                //println!("moving boxes");
            }
            for left_box in sorted_boxes {
                let right_box = (left_box.0, left_box.1 + 1);
                grid[left_box.0][left_box.1] = DoubleSpace::Empty;
                grid[right_box.0][right_box.1] = DoubleSpace::Empty;

                let updated_left_box = next_move.update_coords(left_box);
                let updated_right_box = next_move.update_coords(right_box);
                grid[updated_left_box.0][updated_left_box.1] = DoubleSpace::LeftBox;
                grid[updated_right_box.0][updated_right_box.1] = DoubleSpace::RightBox;
            }
            grid[curr_coords.0][curr_coords.1] = DoubleSpace::Empty;
            curr_coords = next_move.update_coords(curr_coords);
            grid[curr_coords.0][curr_coords.1] = DoubleSpace::Robot;
        }
    }
    //print_grid(grid.clone());
}

fn get_score<T: Scorable>(grid: Vec<Vec<T>>) -> usize {
    grid.iter().enumerate().fold(0, |acc, (row_idx, row)| {
        let mini_fold = row
            .iter()
            .enumerate()
            .fold(0, |mini_acc, (col_idx, space)| {
                if space.scorable() {
                    mini_acc + (100 * row_idx + col_idx)
                } else {
                    mini_acc
                }
            });
        acc + mini_fold
    })
}

impl Puzzle for Day15 {
    fn puzzle_1(contents: String) {
        let mut start_coords = (0, 0);
        let (mut grid, moves) = read_grid(contents).into_iter().enumerate().fold(
            (Vec::<Vec<Space>>::new(), Vec::<Direction>::new()),
            |(mut acc_grid, mut acc_moves), (row_idx, row)| {
                let mut row_iter = row.into_iter().peekable();
                if let Some(char_to_check) = row_iter.peek() {
                    let is_move_char: Result<Direction, ()> = char_to_check.try_into();
                    if is_move_char.is_ok() {
                        acc_moves.append(&mut get_moves(row_iter));
                    } else {
                        let (start_coords_opt, grid_row) = get_grid_row(row_idx, row_iter);
                        start_coords_opt.map(|val| start_coords = val);
                        acc_grid.push(grid_row);
                    }
                }
                (acc_grid, acc_moves)
            },
        );

        run_sim(start_coords, moves, &mut grid);

        let score = get_score(grid);
        println!("Score is: {score}");
    }

    fn puzzle_2(contents: String) {
        let mut start_coords = (0, 0);
        let (mut grid, moves) = read_grid(contents).into_iter().enumerate().fold(
            (Vec::<Vec<DoubleSpace>>::new(), Vec::<Direction>::new()),
            |(mut acc_grid, mut acc_moves), (row_idx, row)| {
                let mut row_iter = row.into_iter().peekable();
                if let Some(char_to_check) = row_iter.peek() {
                    let is_move_char: Result<Direction, ()> = char_to_check.try_into();
                    if is_move_char.is_ok() {
                        acc_moves.append(&mut get_moves(row_iter));
                    } else {
                        let (start_coords_opt, grid_row) = get_grid_row_2(row_idx, row_iter);
                        start_coords_opt.map(|val| start_coords = val);
                        acc_grid.push(grid_row);
                    }
                }
                (acc_grid, acc_moves)
            },
        );

        run_sim_2(start_coords, moves, &mut grid);

        let score = get_score(grid);
        println!("Score is: {score}");
    }
}
