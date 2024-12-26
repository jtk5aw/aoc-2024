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
                if *col == grid[*row].len() {
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
}

fn find_exit(
    start_pos: (usize, usize),
    grid: &mut Vec<Vec<Space>>,
    mut update_fn: impl FnMut(&mut Space, Direction),
) {
    let (mut curr_r, mut curr_c) = start_pos;
    let mut curr_direction = Direction::Up;

    while !curr_direction.is_exiting(&grid, &curr_r, &curr_c) {
        // println!(
        //     "(curr_r, curr_c, char): ({curr_r}, {curr_c}, {}",
        //     grid[curr_r][curr_c]
        // );
        update_fn(&mut grid[curr_r][curr_c], curr_direction.clone());

        if curr_direction.should_turn(&grid, &curr_r, &curr_c) {
            curr_direction = curr_direction.turn();
        }

        curr_direction.update_pos(&mut curr_r, &mut curr_c);
    }
}

impl Puzzle for Day6 {
    fn puzzle_1(contents: String) {
        let (mut space_grid, start_pos) = build_space_grid(contents);

        // Have to start at 1 cause the final location won't be marked
        let mut spaces_covered = 1;
        let update_fn = |space_val: &mut Space, curr_direction: Direction| {
            if let Space::Empty = space_val {
                *space_val = Space::Visited(SpaceInfo {
                    travel_direction: curr_direction.clone(),
                    count: spaces_covered,
                });
                spaces_covered += 1;
            }
        };

        find_exit(start_pos, &mut space_grid, update_fn);

        //println!("Final grid: {:?}", space_grid);

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

        // Have to start at 1 cause the final location won't be marked
        let mut spaces_covered = 1;
        let update_fn = |space_val: &mut Space, curr_direction: Direction| {
            if let Space::Empty = space_val {
                *space_val = Space::Visited(SpaceInfo {
                    travel_direction: curr_direction.clone(),
                    count: spaces_covered,
                });
                spaces_covered += 1;
            }
        };

        find_exit(start_pos.clone(), &mut grid, update_fn);

        // TODO TODO TODO: Need to do the second trace through at this point looking for places to
        // stop. Just need to implement as another FnMut
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

#[derive(PartialEq, Eq, Debug)]
enum Space {
    Empty,
    Visited(SpaceInfo),
    Obstacle,
}

#[derive(PartialEq, Eq, Debug)]
struct SpaceInfo {
    travel_direction: Direction,
    count: i64,
}
