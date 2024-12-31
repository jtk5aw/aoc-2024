use std::{collections::HashSet, iter, thread::current};

use helpers::{read_grid, Puzzle};

fn main() {
    Day10::run();
}

struct Day10;

fn hike<T>(heights: &Vec<Vec<u32>>, row: usize, col: usize, end_fn: &mut T)
where
    T: FnMut((usize, usize)),
{
    let current_height = heights[row][col];
    if current_height == 9 {
        end_fn((row, col))
    }

    if row > 0 && heights[row - 1][col] == current_height + 1 {
        hike(heights, row - 1, col, end_fn);
    }
    if col > 0 && heights[row][col - 1] == current_height + 1 {
        hike(heights, row, col - 1, end_fn);
    }
    if row < heights.len() - 1 && heights[row + 1][col] == current_height + 1 {
        hike(heights, row + 1, col, end_fn);
    }
    if col < heights[row].len() - 1 && heights[row][col + 1] == current_height + 1 {
        hike(heights, row, col + 1, end_fn);
    }
}

fn search_grid<T>(grid: &Vec<Vec<u32>>, search_fn: &mut T) -> usize
where
    T: FnMut(&Vec<Vec<u32>>, (usize, usize)) -> usize,
{
    let mut sum = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, height) in row.iter().enumerate() {
            if *height == 0 {
                sum += search_fn(grid, (row_idx, col_idx));
            }
        }
    }
    sum
}

fn convert_grid(char_grid: Vec<Vec<char>>) -> Vec<Vec<u32>> {
    char_grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|height_char| height_char.to_digit(10).expect("has to be a digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

impl Puzzle for Day10 {
    fn puzzle_1(contents: String) {
        let grid = convert_grid(read_grid(contents));

        let mut search_fn = |heights: &Vec<Vec<u32>>, coords: (usize, usize)| {
            let mut result = HashSet::new();
            let mut end_fn = |inner_coords: (usize, usize)| {
                result.insert(inner_coords);
            };
            hike(heights, coords.0, coords.1, &mut end_fn);
            result.len()
        };
        let sum = search_grid(&grid, &mut search_fn);

        println!("trailhead score is: {sum}");
    }

    fn puzzle_2(contents: String) {
        let grid = convert_grid(read_grid(contents));

        let mut search_fn = |heights: &Vec<Vec<u32>>, coords: (usize, usize)| {
            let mut result = 0;
            let mut end_fn = |_: (usize, usize)| {
                result += 1;
            };
            hike(heights, coords.0, coords.1, &mut end_fn);
            result
        };
        let sum = search_grid(&grid, &mut search_fn);

        println!("trailhead rank is: {sum}");
    }
}
