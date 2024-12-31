use std::{collections::HashMap, path::Ancestors};

use helpers::{read_grid, Puzzle};

fn main() {
    Day8::run();
}

struct Day8;

type Coords = (usize, usize);

enum NodePair {
    TopLeftBottomRight(Coords, Coords),
    TopRightBottomLeft(Coords, Coords),
}

impl From<(Coords, Coords)> for NodePair {
    fn from((first_val, second_val): (Coords, Coords)) -> Self {
        match (first_val.0 > second_val.0, first_val.1 > second_val.1) {
            (true, true) => NodePair::TopLeftBottomRight(second_val, first_val),
            (true, false) => NodePair::TopRightBottomLeft(second_val, first_val),
            (false, true) => NodePair::TopRightBottomLeft(first_val, second_val),
            (false, false) => NodePair::TopLeftBottomRight(first_val, second_val),
        }
    }
}

impl NodePair {
    fn find_antinodes(self, row_max: usize, col_max: usize) -> Vec<(Coords, Coords, Coords)> {
        let mut result = Vec::with_capacity(4);
        match self {
            NodePair::TopLeftBottomRight(top_left, bottom_right) => {
                let row_diff = bottom_right.0 - top_left.0;
                let col_diff = bottom_right.1 - top_left.1;

                if top_left.0 >= row_diff && top_left.1 >= col_diff {
                    result.push((
                        top_left,
                        bottom_right,
                        (top_left.0 - row_diff, top_left.1 - col_diff),
                    ));
                }

                if bottom_right.0 + row_diff < row_max && bottom_right.1 + col_diff < col_max {
                    result.push((
                        top_left,
                        bottom_right,
                        (bottom_right.0 + row_diff, bottom_right.1 + col_diff),
                    ));
                }
            }
            NodePair::TopRightBottomLeft(top_right, bottom_left) => {
                let row_diff = bottom_left.0 - top_right.0;
                let col_diff = top_right.1 - bottom_left.1;

                if top_right.0 >= row_diff && top_right.1 + col_diff < col_max {
                    result.push((
                        top_right,
                        bottom_left,
                        (top_right.0 - row_diff, top_right.1 + col_diff),
                    ));
                }

                if bottom_left.0 + row_diff < row_max && bottom_left.1 >= col_diff {
                    result.push((
                        top_right,
                        bottom_left,
                        (bottom_left.0 + row_diff, bottom_left.1 - col_diff),
                    ));
                }
            }
        }
        result
    }

    fn find_limitless_antinodes(
        self,
        row_max: usize,
        col_max: usize,
    ) -> Vec<(Coords, Coords, Coords)> {
        let mut result = Vec::with_capacity(4);
        match self {
            NodePair::TopLeftBottomRight(top_left, bottom_right) => {
                let row_diff = bottom_right.0 - top_left.0;
                let col_diff = bottom_right.1 - top_left.1;

                let mut top_left = top_left;
                let mut bottom_right = bottom_right;

                result.push((top_left, bottom_right, top_left));
                result.push((top_left, bottom_right, bottom_right));

                while top_left.0 >= row_diff && top_left.1 >= col_diff {
                    result.push((
                        top_left,
                        bottom_right,
                        (top_left.0 - row_diff, top_left.1 - col_diff),
                    ));
                    top_left.0 -= row_diff;
                    top_left.1 -= col_diff;
                }

                while bottom_right.0 + row_diff < row_max && bottom_right.1 + col_diff < col_max {
                    result.push((
                        top_left,
                        bottom_right,
                        (bottom_right.0 + row_diff, bottom_right.1 + col_diff),
                    ));
                    bottom_right.0 += row_diff;
                    bottom_right.1 += col_diff;
                }
            }
            NodePair::TopRightBottomLeft(top_right, bottom_left) => {
                let row_diff = bottom_left.0 - top_right.0;
                let col_diff = top_right.1 - bottom_left.1;

                let mut top_right = top_right;
                let mut bottom_left = bottom_left;

                result.push((top_right, bottom_left, top_right));
                result.push((top_right, bottom_left, bottom_left));

                while top_right.0 >= row_diff && top_right.1 + col_diff < col_max {
                    result.push((
                        top_right,
                        bottom_left,
                        (top_right.0 - row_diff, top_right.1 + col_diff),
                    ));
                    top_right.0 -= row_diff;
                    top_right.1 += col_diff;
                }

                while bottom_left.0 + row_diff < row_max && bottom_left.1 >= col_diff {
                    result.push((
                        top_right,
                        bottom_left,
                        (bottom_left.0 + row_diff, bottom_left.1 - col_diff),
                    ));
                    bottom_left.0 += row_diff;
                    bottom_left.1 -= col_diff;
                }
            }
        }
        result
    }
}

fn get_antennas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas = HashMap::new();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, space_char) in row.iter().enumerate() {
            if *space_char != '.' {
                antennas
                    .entry(space_char.clone())
                    .and_modify(|locations: &mut Vec<(usize, usize)>| {
                        locations.push((row_idx, col_idx))
                    })
                    .or_insert(vec![(row_idx, col_idx)]);
            }
        }
    }
    antennas
}

fn calculate_antinodes<T>(
    grid: &mut Vec<Vec<char>>,
    antennas: HashMap<char, Vec<Coords>>,
    find_antinodes: &mut T,
) -> i64
where
    T: FnMut(NodePair, usize, usize) -> Vec<(Coords, Coords, Coords)>,
{
    let mut antinode_count = 0;
    for (antenna_char, vals) in antennas.into_iter() {
        for first_idx in 0..vals.len() {
            let first_val = vals[first_idx];
            for second_idx in first_idx + 1..vals.len() {
                let second_val = vals[second_idx];

                let node_pair = NodePair::from((first_val, second_val));

                let antinodes = find_antinodes(node_pair, grid.len(), grid[0].len());
                for (first, second, (row, col)) in antinodes {
                    println!("antinode found!: ({row}, {col})");
                    println!("{antenna_char} from antennas ({}, {}), ({}, {}) antinode at ({row}, {col})", first.0, first.1, second.0, second.1);
                    if grid[row][col] != '#' {
                        grid[row][col] = '#';
                        antinode_count += 1;
                    }
                }
            }
        }
    }
    antinode_count
}

impl Puzzle for Day8 {
    fn puzzle_1(contents: String) {
        let grid = read_grid(contents);

        let antennas = get_antennas(&grid);

        let mut grid = grid;
        let mut find_antinodes =
            |node_pair: NodePair, row_max, col_max| -> Vec<(Coords, Coords, Coords)> {
                node_pair.find_antinodes(row_max, col_max)
            };

        let antinode_count = calculate_antinodes(&mut grid, antennas, &mut find_antinodes);

        println!("Number of antinodes is: {antinode_count}");
    }

    fn puzzle_2(contents: String) {
        let grid = read_grid(contents);

        let antennas = get_antennas(&grid);

        let mut grid = grid;
        let mut find_antinodes =
            |node_pair: NodePair, row_max, col_max| -> Vec<(Coords, Coords, Coords)> {
                node_pair.find_limitless_antinodes(row_max, col_max)
            };

        let antinode_count = calculate_antinodes(&mut grid, antennas, &mut find_antinodes);

        println!("number of anitnodes is: {antinode_count}");
    }
}
