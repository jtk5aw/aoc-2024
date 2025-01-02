use std::{cmp::Ordering, collections::BinaryHeap};

use helpers::Puzzle;

fn main() {
    Day14::run()
}

struct Day14;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Robot {
    starting_pos: (i64, i64),
    velocity: (i64, i64),
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (pos_str, vel_str) = value.split_once(" ").expect("Has to be separate by space");

        let starting_pos_strs = pos_str[2..]
            .split_once(",")
            .expect("has to be separate by comma");
        let velocity_strs = vel_str[2..]
            .split_once(",")
            .expect("has to be separate by comma");

        let starting_pos = (
            starting_pos_strs
                .0
                .parse::<i64>()
                .expect("has to be a number"),
            starting_pos_strs
                .1
                .parse::<i64>()
                .expect("has to be a number"),
        );
        let velocity = (
            velocity_strs.0.parse::<i64>().expect("has to be a number"),
            velocity_strs.1.parse::<i64>().expect("has to be a number"),
        );

        Robot {
            starting_pos,
            velocity,
        }
    }
}

#[derive(Debug)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    MidLines,
}

impl From<((usize, usize), (usize, usize))> for Quadrant {
    fn from((pos, bounds): ((usize, usize), (usize, usize))) -> Self {
        let half_bounds = ((bounds.0 / 2) as usize, (bounds.1 / 2) as usize);
        let (x_usize, y_usize) = pos;
        //println!("location is: ({x_usize}, {y_usize})");
        match (x_usize < half_bounds.0, y_usize < half_bounds.1) {
            (true, true) => Quadrant::TopLeft,
            (true, false) if half_bounds.1 != y_usize => Quadrant::BottomLeft,
            (false, true) if half_bounds.0 != x_usize => Quadrant::TopRight,
            (false, false) if half_bounds.0 != x_usize && half_bounds.1 != y_usize => {
                Quadrant::BottomRight
            }
            _ => Quadrant::MidLines,
        }
    }
}

impl Quadrant {
    fn check_quadrant(&self) -> (i32, i32, i32, i32, i32) {
        match self {
            Quadrant::TopLeft => (1, 0, 0, 0, 0),
            Quadrant::TopRight => (0, 1, 0, 0, 0),
            Quadrant::BottomLeft => (0, 0, 1, 0, 0),
            Quadrant::BottomRight => (0, 0, 0, 1, 0),
            Quadrant::MidLines => (0, 0, 0, 0, 1),
        }
    }
}

impl Robot {
    fn step(&self, coords: (i64, i64), bounds: (i64, i64)) -> (i64, i64) {
        let (vel_x, vel_y) = self.velocity;
        let mut x = coords.0 + vel_x;
        let mut y = coords.1 + vel_y;

        if x < 0 {
            x = bounds.0 + 1 + x;
        }

        if y < 0 {
            y = bounds.1 + 1 + y;
        }

        if bounds.0 < x {
            x = x - bounds.0 - 1;
        }

        if bounds.1 < y {
            y = y - bounds.1 - 1;
        }

        (x, y)
    }

    fn step_n_times(&self, n: i64, bounds: (i64, i64)) -> Quadrant {
        let (mut x, mut y) = self.starting_pos;
        for _ in 0..n {
            (x, y) = self.step((x, y), bounds);
        }

        Quadrant::from((
            (x as usize, y as usize),
            (bounds.0 as usize, bounds.1 as usize),
        ))
    }
}

const TEST_BOUNDS: (i64, i64) = (10, 6);
const P1_BOUNDS: (i64, i64) = (100, 102);
const P1_BOUNDS_USIZE: (usize, usize) = (100, 102);

// WARNING: This one is weird for its tset input as it requires something the change that is not
// included as part of the input file.
// So if coming back to this and want to run the test input file you have to change rom P1_BOUNDS
// to TEST_BOUNDS
impl Puzzle for Day14 {
    fn puzzle_1(contents: String) {
        let quadrant_counts =
            contents
                .lines()
                .map(Robot::from)
                .fold((0, 0, 0, 0, 0), |acc, robot| {
                    println!("{:?}", robot);
                    let outcome = robot.step_n_times(100, P1_BOUNDS);
                    println!("outcome: {:?}", outcome);
                    let check_quadrant = outcome.check_quadrant();
                    (
                        acc.0 + check_quadrant.0,
                        acc.1 + check_quadrant.1,
                        acc.2 + check_quadrant.2,
                        acc.3 + check_quadrant.3,
                        acc.4 + check_quadrant.4,
                    )
                });
        println!("Quadrant_counts: {:?}", quadrant_counts);
        let result = quadrant_counts.0 * quadrant_counts.1 * quadrant_counts.2 * quadrant_counts.3;
        println!("the final result is: {result}");
    }

    fn puzzle_2(contents: String) {
        let mut robots = contents.lines().map(Robot::from).collect::<Vec<_>>();
        let mut heap = BinaryHeap::<State>::new();
        for i in 0..10_000 {
            let conv_count = get_conv_count(&robots, P1_BOUNDS_USIZE, 3);
            if heap.len() < 100
                || heap
                    .peek()
                    .filter(|min_value| min_value.conv_count < conv_count)
                    .is_some()
            {
                heap.push(State {
                    conv_count,
                    index: i,
                    vec: robots.clone(),
                });
            }
            robots = robots
                .iter()
                .map(|robot| Robot {
                    starting_pos: robot.step(robot.starting_pos, P1_BOUNDS),
                    velocity: robot.velocity,
                })
                .collect::<Vec<_>>();
        }

        println!("Size of bin_heap: {}", heap.len());

        while let Some(test) = heap.pop() {
            println!("conv_count: {}, index: {}", test.conv_count, test.index);
            if test.conv_count > 0 {
                print_grid(test.vec, P1_BOUNDS_USIZE);
            }
        }
    }
}

// 7503 is too high

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    conv_count: usize,
    index: usize,
    vec: Vec<Robot>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare index - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        // (above is copied from docs and technically breaks down if
        // two values can have the same index but for my implementation that's impossible)
        other
            .conv_count
            .cmp(&self.conv_count)
            .then_with(|| self.index.cmp(&other.index))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_bool_grid(robots: &Vec<Robot>, bounds: (usize, usize)) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; bounds.1 + 1]; bounds.0 + 1];

    for robot in robots {
        grid[robot.starting_pos.0 as usize][robot.starting_pos.1 as usize] = true;
    }

    grid
}

fn get_conv_count(robots: &Vec<Robot>, bounds: (usize, usize), conv_iterations: usize) -> usize {
    let mut grid = build_bool_grid(robots, bounds);

    for _ in 0..conv_iterations {
        let mut new_grid = Vec::with_capacity(grid.len());
        for x in 1..grid.len() - 1 {
            let mut row = Vec::with_capacity(grid[x].len());
            for y in 1..grid[x].len() - 1 {
                let all_marked = grid[x][y]
                    && grid[x - 1][y]
                    && grid[x + 1][y]
                    && grid[x][y - 1]
                    && grid[x][y + 1];
                row.push(all_marked);
            }
            new_grid.push(row);
        }
        grid = new_grid;
    }

    let mut true_count = 0;
    for row in grid {
        for space in row {
            if space {
                true_count += 1;
            }
        }
    }

    true_count
}

fn print_grid(robots: Vec<Robot>, bounds: (usize, usize)) {
    let grid = build_bool_grid(&robots, bounds);

    for row in grid {
        for space in row {
            if space {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
    println!("");
    println!("");
    println!("");
}
