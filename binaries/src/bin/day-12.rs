use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    slice::Iter,
};

use helpers::{read_grid, Puzzle};

fn main() {
    Day12::run();
}

struct Day12;

fn check_neighbor(grid: &Vec<Vec<char>>, direction: Direction, row: usize, col: usize) -> Neighbor {
    let plot_char = grid[row][col];
    match direction {
        Direction::Top if row != 0 && grid[row - 1][col] == plot_char => Some((row - 1, col)),
        Direction::TopLeft if row != 0 && col != 0 && grid[row - 1][col - 1] == plot_char => {
            Some((row - 1, col - 1))
        }
        Direction::TopRight
            if row != 0 && col != grid[row].len() - 1 && grid[row - 1][col + 1] == plot_char =>
        {
            Some((row - 1, col + 1))
        }
        Direction::Bottom if row != grid.len() - 1 && grid[row + 1][col] == plot_char => {
            Some((row + 1, col))
        }
        Direction::BottomLeft
            if row != grid.len() - 1 && col != 0 && grid[row + 1][col - 1] == plot_char =>
        {
            Some((row + 1, col - 1))
        }
        Direction::BottomRight
            if row != grid.len() - 1
                && col != grid[row].len() - 1
                && grid[row + 1][col + 1] == plot_char =>
        {
            Some((row + 1, col + 1))
        }
        Direction::Right if col != grid[row].len() - 1 && grid[row][col + 1] == plot_char => {
            Some((row, col + 1))
        }
        Direction::Left if col != 0 && grid[row][col - 1] == plot_char => Some((row, col - 1)),
        _ => None,
    }
    .map_or_else(
        || Neighbor::Other(direction.clone()),
        |coords| {
            Neighbor::Same(NeighborDetails {
                coords,
                direction: direction.clone(),
            })
        },
    )
}

fn get_surrounding_spaces(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<Neighbor> {
    Direction::all_directions()
        .into_iter()
        .map(|direction| check_neighbor(grid, direction, row, col))
        .collect()
}

#[derive(Debug)]
enum Neighbor {
    Same(NeighborDetails),
    Other(Direction),
}

#[derive(Debug)]
struct NeighborDetails {
    coords: (usize, usize),
    direction: Direction,
}

#[derive(Eq, Debug, Clone)]
struct Corner {
    coords: (usize, usize),
    direction: Direction,
}

// Always hash upper left corner. This means the usize values may go out of bounds of the grid
// but that doesn't matter.
impl Hash for Corner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let normalized_corner = normalize_corner(self);
        normalized_corner.direction.hash(state);
        normalized_corner.coords.hash(state);
    }
}

impl PartialEq for Corner {
    fn eq(&self, other: &Self) -> bool {
        let self_normalized = normalize_corner(self);
        let other_normalzied = normalize_corner(other);
        self_normalized.coords == other_normalzied.coords
            && self_normalized.direction == other_normalzied.direction
    }
}

fn normalize_corner(corner: &Corner) -> Corner {
    match corner.direction {
        Direction::TopLeft => corner.clone(),
        Direction::TopRight => Corner {
            coords: (corner.coords.0, corner.coords.1 + 1),
            direction: Direction::TopLeft,
        },
        Direction::BottomLeft => Corner {
            coords: (corner.coords.0 + 1, corner.coords.1),
            direction: Direction::TopLeft,
        },
        Direction::BottomRight => Corner {
            coords: (corner.coords.0 + 1, corner.coords.1 + 1),
            direction: Direction::TopLeft,
        },
        _ => panic!("Not a possible corner"),
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Direction {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Right,
    Left,
}

impl Direction {
    fn all_directions() -> Vec<Self> {
        let mut all_directions = Vec::with_capacity(8);
        all_directions.push(Direction::Top);
        all_directions.push(Direction::TopLeft);
        all_directions.push(Direction::TopRight);
        all_directions.push(Direction::Bottom);
        all_directions.push(Direction::BottomLeft);
        all_directions.push(Direction::BottomRight);
        all_directions.push(Direction::Right);
        all_directions.push(Direction::Left);
        all_directions
    }
}

trait SameOnly<'a> {
    fn only_same(self) -> impl Iterator<Item = &'a NeighborDetails>;
}

impl<'a> SameOnly<'a> for Iter<'a, Neighbor> {
    fn only_same(self) -> impl Iterator<Item = &'a NeighborDetails> {
        self.filter_map(|neighbor| match neighbor {
            Neighbor::Same(neighbor_details) => Some(neighbor_details),
            Neighbor::Other(_) => None,
        })
    }
}

trait NonDiagonalOnly<'a> {
    fn only_non_diagonal(self) -> impl Iterator<Item = &'a NeighborDetails>;
}
impl<'a, T: Iterator<Item = &'a NeighborDetails>> NonDiagonalOnly<'a> for T {
    fn only_non_diagonal(self) -> impl Iterator<Item = &'a NeighborDetails> {
        self.filter(|neighbor_details| match neighbor_details.direction {
            Direction::Top | Direction::Bottom | Direction::Right | Direction::Left => true,
            _ => false,
        })
    }
}

fn basic_fill(
    grid: &Vec<Vec<char>>,
    mut current_region: &mut HashSet<(usize, usize)>,
    row: usize,
    col: usize,
) {
    if current_region.contains(&(row, col)) {
        return;
    }
    current_region.insert((row, col));
    get_surrounding_spaces(grid, row, col)
        .iter()
        .only_same()
        .only_non_diagonal()
        .for_each(|neighbor_details| {
            basic_fill(
                grid,
                &mut current_region,
                neighbor_details.coords.0,
                neighbor_details.coords.1,
            )
        });
}

trait FillRegionTrait<R: Sized> {
    fn default_result() -> R;
    fn sub_answers(
        grid: &Vec<Vec<char>>,
        curr_region: &HashSet<(usize, usize)>,
        visited: &mut HashSet<(usize, usize)>,
        neighbors: &Vec<Neighbor>,
    ) -> R;
    fn partial_answer(
        row: usize,
        col: usize,
        neighbors: Vec<Neighbor>,
        curr_region: &HashSet<(usize, usize)>,
        sub_answers: R,
    ) -> R;

    fn fill_region(
        grid: &Vec<Vec<char>>,
        curr_region: &HashSet<(usize, usize)>,
        visited: &mut HashSet<(usize, usize)>,
        row: usize,
        col: usize,
    ) -> R {
        if visited.contains(&(row, col)) {
            return Self::default_result();
        }
        let neighbors = get_surrounding_spaces(grid, row, col);
        visited.insert((row, col));
        let sub_answers = Self::sub_answers(grid, curr_region, visited, &neighbors);

        Self::partial_answer(row, col, neighbors, curr_region, sub_answers)
    }

    fn update_sum(row: usize, col: usize, partial_answer: R) -> usize;
}

struct AreaAndSide;

impl FillRegionTrait<(usize, usize, HashSet<Corner>)> for AreaAndSide {
    fn default_result() -> (usize, usize, HashSet<Corner>) {
        (0, 0, HashSet::new())
    }

    fn sub_answers(
        grid: &Vec<Vec<char>>,
        current_region: &HashSet<(usize, usize)>,
        mut visited: &mut HashSet<(usize, usize)>,
        neighbors: &Vec<Neighbor>,
    ) -> (usize, usize, HashSet<Corner>) {
        neighbors
            .iter()
            .only_same()
            .only_non_diagonal()
            .map(|neighbor| neighbor.coords)
            .map(|(row, col)| Self::fill_region(grid, current_region, &mut visited, row, col))
            .fold(
                (0, 0, HashSet::new()),
                |(acc_area_count, acc_edge_case_count, mut unique_corners),
                 (neighbor_area_count, edge_case_count, neighbor_corners)| {
                    unique_corners.extend(neighbor_corners);
                    (
                        acc_area_count + neighbor_area_count,
                        acc_edge_case_count + edge_case_count,
                        unique_corners,
                    )
                },
            )
    }

    fn partial_answer(
        row: usize,
        col: usize,
        neighbors: Vec<Neighbor>,
        curr_region: &HashSet<(usize, usize)>,
        sub_answers: (usize, usize, HashSet<Corner>),
    ) -> (usize, usize, HashSet<Corner>) {
        let mut corner_counts: HashMap<Direction, usize> = HashMap::with_capacity(4);
        corner_counts.insert(Direction::TopLeft, 0);
        corner_counts.insert(Direction::TopRight, 0);
        corner_counts.insert(Direction::BottomLeft, 0);
        corner_counts.insert(Direction::BottomRight, 0);

        // TODO: This is a hack but I'm sick of this problem and just want to be done
        // Reason is that diagonals can be the same char but from another region. Need to mutate
        // them to other in these cases
        let converted_neighbors = neighbors
            .into_iter()
            .map(|neighbor| match &neighbor {
                Neighbor::Same(neighbor_details) => match neighbor_details.direction {
                    Direction::TopLeft
                    | Direction::TopRight
                    | Direction::BottomLeft
                    | Direction::BottomRight
                        if !curr_region
                            .contains(&(neighbor_details.coords.0, neighbor_details.coords.1)) =>
                    {
                        Neighbor::Other(neighbor_details.direction.clone())
                    }
                    _ => neighbor,
                },
                _ => neighbor,
            })
            .collect::<Vec<_>>();

        for neighbor in converted_neighbors.iter() {
            match neighbor {
                Neighbor::Other(Direction::Top) => {
                    inc_counter(&mut corner_counts, Direction::TopLeft);
                    inc_counter(&mut corner_counts, Direction::TopRight);
                }
                Neighbor::Other(Direction::Bottom) => {
                    inc_counter(&mut corner_counts, Direction::BottomLeft);
                    inc_counter(&mut corner_counts, Direction::BottomRight);
                }
                Neighbor::Other(Direction::Right) => {
                    inc_counter(&mut corner_counts, Direction::TopRight);
                    inc_counter(&mut corner_counts, Direction::BottomRight);
                }
                Neighbor::Other(Direction::Left) => {
                    inc_counter(&mut corner_counts, Direction::TopLeft);
                    inc_counter(&mut corner_counts, Direction::BottomLeft);
                }
                Neighbor::Other(Direction::TopLeft) => {
                    inc_counter(&mut corner_counts, Direction::TopLeft)
                }
                Neighbor::Other(Direction::TopRight) => {
                    inc_counter(&mut corner_counts, Direction::TopRight)
                }
                Neighbor::Other(Direction::BottomLeft) => {
                    inc_counter(&mut corner_counts, Direction::BottomLeft)
                }
                Neighbor::Other(Direction::BottomRight) => {
                    inc_counter(&mut corner_counts, Direction::BottomRight)
                }
                Neighbor::Same(_) => {}
            }
        }

        let mut unique_corners = HashSet::with_capacity(4);
        let mut edge_case_count = 0;
        for (corner, count) in corner_counts {
            // Check edge case like follows
            // FFF
            // F.F
            // .FF
            match &corner {
                Direction::TopLeft
                | Direction::TopRight
                | Direction::BottomLeft
                | Direction::BottomRight
                    if count == 2 =>
                {
                    let directional_neighbor = converted_neighbors
                        .iter()
                        .only_same()
                        .find(|neighor_detail| neighor_detail.direction == corner);
                    if let Some(_) = directional_neighbor {
                        //println!("TESTING TESTING TESTING");
                        edge_case_count += 1;
                    }
                }
                Direction::Top | Direction::Left | Direction::Right | Direction::Bottom => {
                    panic!("Not a corner direction")
                }
                _ => {}
            };

            // Make regular corner checks
            let is_corner = match &corner {
                Direction::TopLeft
                | Direction::TopRight
                | Direction::BottomLeft
                | Direction::BottomRight
                    if count == 1 || count == 3 =>
                {
                    true
                }
                Direction::Top | Direction::Left | Direction::Right | Direction::Bottom => {
                    panic!("Not a corner direction")
                }
                _ => false,
            };

            if is_corner {
                unique_corners.insert(Corner {
                    coords: (row, col),
                    direction: corner,
                });
            }
        }

        unique_corners.extend(sub_answers.2);
        (
            1 + sub_answers.0,
            sub_answers.1 + edge_case_count,
            unique_corners,
        )
    }

    fn update_sum(
        row: usize,
        col: usize,
        (area, edge_case_count, corners): (usize, usize, HashSet<Corner>),
    ) -> usize {
        //println!("unique_sides:");
        corners.iter().for_each(|corner| {
            // println!(
            //     "({}, {}) {:?}",
            //     corner.coords.0 as i64 - row as i64,
            //     corner.coords.1 as i64 - col as i64,
            //     corner.direction
            // )
        });
        println!("area: ({}) side_count: ({})", area, corners.len());
        if edge_case_count != 0 {
            println!("TESTING TESTING TESTING");
        }
        area * (edge_case_count + corners.len())
    }
}

fn inc_counter(corner_counts: &mut HashMap<Direction, usize>, direction: Direction) {
    corner_counts.entry(direction).and_modify(|val| *val += 1);
}

struct AreaAndPerimeter;

impl FillRegionTrait<(usize, usize)> for AreaAndPerimeter {
    fn default_result() -> (usize, usize) {
        (0, 0)
    }

    fn sub_answers(
        grid: &Vec<Vec<char>>,
        current_region: &HashSet<(usize, usize)>,
        mut visited: &mut HashSet<(usize, usize)>,
        neighbors: &Vec<Neighbor>,
    ) -> (usize, usize) {
        neighbors
            .iter()
            .only_same()
            .only_non_diagonal()
            .map(|neighbor| neighbor.coords)
            .map(|(row, col)| Self::fill_region(grid, current_region, &mut visited, row, col))
            .fold((0, 0), |acc, (area_count, perimeter_count)| {
                (acc.0 + area_count, acc.1 + perimeter_count)
            })
    }

    fn partial_answer(
        _: usize,
        _: usize,
        neighbors: Vec<Neighbor>,
        _: &HashSet<(usize, usize)>,
        (neighbors_area_count, neighbors_perimeter_count): (usize, usize),
    ) -> (usize, usize) {
        let same_neighbor_count = neighbors.iter().only_same().only_non_diagonal().count();
        (
            1 + neighbors_area_count,
            (4 - same_neighbor_count) + neighbors_perimeter_count,
        )
    }

    fn update_sum(_: usize, _: usize, (area_count, perimeter_count): (usize, usize)) -> usize {
        println!("area: ({area_count}) and perimeter_count ({perimeter_count})");
        area_count * perimeter_count
    }
}

fn solve<T, R>(grid: Vec<Vec<char>>) -> usize
where
    T: FillRegionTrait<R>,
    R: Sized,
{
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut sum = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, plot_char) in row.iter().enumerate() {
            if !visited[row_idx][col_idx] {
                // Need to know the bounds of the region before finding sides so that is done here
                let mut current_region = HashSet::new();
                basic_fill(&grid, &mut current_region, row_idx, col_idx);
                let current_region = current_region;

                let mut visited_set = HashSet::new();
                let partial_result =
                    T::fill_region(&grid, &current_region, &mut visited_set, row_idx, col_idx);
                for (row, col) in visited_set {
                    visited[row][col] = true;
                }
                println!(
                    "region char: {} starting_coords: ({row_idx}, {col_idx})",
                    plot_char
                );
                sum += T::update_sum(row_idx, col_idx, partial_result);
            }
        }
    }

    sum
}

impl Puzzle for Day12 {
    fn puzzle_1(contents: String) {
        let grid = read_grid(contents);
        let sum = solve::<AreaAndPerimeter, (usize, usize)>(grid);
        println!("The sum is {sum}");
    }

    fn puzzle_2(contents: String) {
        let grid = read_grid(contents);
        let sum = solve::<AreaAndSide, (usize, usize, HashSet<Corner>)>(grid);
        println!("The sum is {sum}");
    }
}
