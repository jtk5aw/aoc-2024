use std::{cell::RefCell, rc::Rc};

use helpers::{read_grid, Puzzle};

fn main() {
    Day6::run();
}

struct Day6;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn should_turn(&self, grid: &Vec<Vec<Space>>, row: &usize, col: &usize) -> bool {
        match &self {
            Direction::Up => {
                if let Space::Obstacle = grid[row - 1][*col] {
                    return true;
                }
            }
            Direction::Down => {
                if let Space::Obstacle = grid[row + 1][*col] {
                    return true;
                }
            }
            Direction::Left => {
                if let Space::Obstacle = grid[*row][col - 1] {
                    return true;
                }
            }
            Direction::Right => {
                if let Space::Obstacle = grid[*row][col + 1] {
                    return true;
                }
            }
        }
        false
    }

    fn matches(&self, space: &Space) -> Option<SpaceInfo> {
        match space {
            Space::Visited(space_info) => {
                if space_info.travel_direction == *self {
                    Some(space_info.clone())
                } else {
                    None
                }
            }
            Space::Intersection(space_infos) => space_infos
                .iter()
                .find(|space_info| space_info.travel_direction == *self)
                .map(|space_info| space_info.clone()),
            Space::Empty => None,
            Space::Obstacle => panic!("Can't compare a direction with an obstacle space"),
        }
    }

    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn is_exiting(&self, grid: &Vec<Vec<Space>>, row: &usize, col: &usize) -> bool {
        match &self {
            Direction::Up => {
                if *row == 0 {
                    return true;
                }
            }
            Direction::Down => {
                if *row == grid.len() - 1 {
                    return true;
                }
            }
            Direction::Left => {
                if *col == 0 {
                    return true;
                }
            }
            Direction::Right => {
                if *col == grid[*row].len() - 1 {
                    return true;
                }
            }
        }
        false
    }

    fn update_pos(&self, row: &mut usize, col: &mut usize) {
        match &self {
            Direction::Up => *row -= 1,
            Direction::Down => *row += 1,
            Direction::Left => *col -= 1,
            Direction::Right => *col += 1,
        }
    }

    fn peek_update_pos(&self, row: &usize, col: &usize) -> (usize, usize) {
        match &self {
            Direction::Up => (*row - 1, *col),
            Direction::Down => (*row + 1, *col),
            Direction::Left => (*row, *col - 1),
            Direction::Right => (*row, *col + 1),
        }
    }
}

fn traverse_grid(
    start_pos: (usize, usize),
    start_direction: Direction,
    grid: &mut Vec<Vec<Space>>,
    mut should_continue_fn: impl FnMut(&Vec<Vec<Space>>, (&usize, &usize), &Direction) -> bool,
    mut update_fn: impl FnMut(&mut Vec<Vec<Space>>, (usize, usize), Direction),
) {
    let (mut curr_r, mut curr_c) = start_pos;
    let mut curr_direction = start_direction.clone();

    while should_continue_fn(grid, (&curr_r, &curr_c), &curr_direction) {
        update_fn(grid, (curr_r, curr_c), curr_direction.clone());

        if curr_direction.should_turn(&grid, &curr_r, &curr_c) {
            curr_direction = curr_direction.turn();
        } else {
            curr_direction.update_pos(&mut curr_r, &mut curr_c);
        }
    }
}

fn is_exiting(
    grid: &Vec<Vec<Space>>,
    coords: (&usize, &usize),
    curr_direction: &Direction,
) -> bool {
    !curr_direction.is_exiting(grid, coords.0, coords.1)
}

fn noop_update_fn(grid: &mut Vec<Vec<Space>>, coords: (usize, usize), direction: Direction) {}

fn mark_path_to_exit(
    start_pos: (usize, usize),
    grid: &mut Vec<Vec<Space>>,
) -> (i64, i64, &mut Vec<Vec<Space>>) {
    // Have to start at 1 cause the final location won't be marked
    let mut spaces_covered = 1;
    let mut pos_count = 0;
    let update_fn =
        |grid: &mut Vec<Vec<Space>>, coords: (usize, usize), curr_direction: Direction| {
            let space_val = &mut grid[coords.0][coords.1];
            let new_space_info = SpaceInfo {
                travel_direction: curr_direction.clone(),
                count: pos_count,
            };
            pos_count += 1;
            match space_val {
                Space::Empty => {
                    *space_val = Space::Visited(new_space_info);
                    spaces_covered += 1;
                }
                Space::Visited(space_info) => {
                    let space_infos = vec![space_info.clone(), new_space_info];
                    *space_val = Space::Intersection(space_infos)
                }
                Space::Intersection(space_infos) => space_infos.push(new_space_info),
                Space::Obstacle => panic!("Can't be 'on' an obstacle while marking the path"),
            }
        };

    traverse_grid(start_pos, Direction::Up, grid, is_exiting, update_fn);

    (spaces_covered, pos_count, grid)
}

impl Puzzle for Day6 {
    fn puzzle_1(contents: String) {
        let (mut space_grid, start_pos) = build_space_grid(contents);

        let (spaces_covered, _, _) = mark_path_to_exit(start_pos, &mut space_grid);

        println!("Number of spaces covered: {spaces_covered}");
    }

    /**
     * Plan:
     * 1. Trace through the map to see where I would go normally keeping a count on each space
     * 2. At each location set a flag of what directions I've moved through thsi location in and
     *    the number of this space (start at 0 and increment up)
     * 3. After tracing the path, start over again tracing the path. At each space (if not about to
     *    turn) check the direction in which you would turn here to see if you'll create a loop
     *    a. If you go to the right and either hit a path going in the same direction or a wall
     *    that makes you turn in the right direction you can create a loop.
     * 4. Do this till you exit and then return the count
     */

    fn puzzle_2(contents: String) {
        let (mut grid, start_pos) = build_space_grid(contents);

        let (_, final_count, grid) = mark_path_to_exit(start_pos, &mut grid);

        let mut loops_found = 0;
        let find_loops_fn = |grid: &mut Vec<Vec<Space>>,
                             starting_coords: (usize, usize),
                             curr_direction: Direction| {
            let grid = &mut grid.clone();
            let (starting_row, starting_col) = starting_coords;

            let starting_count = match &grid[starting_row][starting_col] {
                Space::Visited(space_info) => space_info.count,
                Space::Intersection(space_infos) => {
                    let matching_direction = space_infos
                        .iter()
                        .filter(|space_info| space_info.travel_direction == curr_direction)
                        .collect::<Vec<_>>();
                    assert!(matching_direction.len() == 1);
                    matching_direction[0].count
                }
                _ => panic!("second lap must be on traveled spaces only"),
            };

            let peek = curr_direction.peek_update_pos(&starting_row, &starting_col);
            match &grid[peek.0][peek.1] {
                Space::Obstacle => return,
                Space::Visited(space_info) => {
                    if space_info.count < starting_count {
                        return;
                    }
                }
                Space::Intersection(space_infos) => {
                    if space_infos
                        .iter()
                        .find(|space_info| space_info.count < starting_count)
                        .is_some()
                    {
                        return;
                    }
                }
                _ => {}
            }

            let new_obstacle_space = &mut grid[peek.0][peek.1];
            *new_obstacle_space = Space::Obstacle;

            let new_spaces_count = Rc::new(RefCell::new(final_count));
            let find_loop_should_continue_fn =
                |grid: &Vec<Vec<Space>>, coords: (&usize, &usize), direction: &Direction| {
                    let is_exiting = direction.is_exiting(grid, coords.0, coords.1);

                    // Fist square will always match but we don't care about this, quit
                    if *new_spaces_count.borrow() == final_count && !is_exiting {
                        return true;
                    }
                    if let Some(space_info) = direction.matches(&grid[*coords.0][*coords.1]) {
                        if space_info.count < starting_count || space_info.count > final_count {
                            loops_found += 1;
                            return false;
                        }
                    }
                    !is_exiting
                };

            let find_loop_update_fn =
                |grid: &mut Vec<Vec<Space>>,
                 inner_coords: (usize, usize),
                 inner_direction: Direction| {
                    let (row, col) = inner_coords;

                    // TODO: Determine if its best practice to underscore this like this
                    *new_spaces_count.borrow_mut() += 1;
                    let space = &mut grid[row][col];
                    let new_space_info = SpaceInfo {
                        travel_direction: inner_direction,
                        count: *new_spaces_count.borrow(),
                    };
                    match space {
                        Space::Visited(space_info) => {
                            let space_infos = vec![space_info.clone(), new_space_info];
                            *space = Space::Intersection(space_infos);
                        }
                        Space::Intersection(space_infos) => {
                            space_infos.push(new_space_info);
                        }
                        Space::Empty => {
                            *space = Space::Visited(new_space_info);
                        }
                        Space::Obstacle => panic!("Shouldn't be able to be 'on' an obstacle here"),
                    };
                };
            traverse_grid(
                starting_coords,
                curr_direction,
                grid,
                find_loop_should_continue_fn,
                find_loop_update_fn,
            );
        };
        traverse_grid(start_pos, Direction::Up, grid, is_exiting, find_loops_fn);

        println!("number of ways to create a loop: {loops_found}");
    }
}

fn build_space_grid(contents: String) -> (Vec<Vec<Space>>, (usize, usize)) {
    let mut start_pos = (0, 0);
    let grid = read_grid(contents)
        .iter()
        .enumerate()
        .map(|(curr_r, row)| {
            row.iter()
                .enumerate()
                .map(|(curr_c, character)| match character {
                    '#' => Space::Obstacle,
                    '.' => Space::Empty,
                    '^' => {
                        start_pos = (curr_r, curr_c);
                        Space::Empty
                    }
                    _ => panic!("this character ({character}) shouldn't happen"),
                })
                .collect()
        })
        .collect();

    (grid, start_pos)
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Space {
    Empty,
    Visited(SpaceInfo),
    Intersection(Vec<SpaceInfo>),
    Obstacle,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct SpaceInfo {
    travel_direction: Direction,
    count: i64,
}
