use std::collections::HashSet;

use helpers::{read_grid, Puzzle};

struct Day20;

#[derive(Clone)]
enum Space {
    Path(Option<usize>),
    Wall,
    Start,
    End(Option<usize>),
}

impl TryFrom<char> for Space {
    type Error = ();

    fn try_from(space_char: char) -> Result<Space, ()> {
        match space_char {
            '.' => Ok(Self::Path(None)),
            '#' => Ok(Self::Wall),
            'S' => Ok(Self::Start),
            'E' => Ok(Self::End(None)),
            _ => Err(()),
        }
    }
}

fn check_surrounding_spaces(
    distance: usize,
    debug_str: String,
    curr_coord: (usize, usize),
    row_len: usize,
    col_len: usize,
    mut fn_mut: impl FnMut((usize, usize)),
) {
    if curr_coord.0 >= distance {
        fn_mut((curr_coord.0 - distance, curr_coord.1))
    }
    if curr_coord.1 >= distance {
        fn_mut((curr_coord.0, curr_coord.1 - distance))
    }
    if curr_coord.0 < row_len - distance {
        fn_mut((curr_coord.0 + distance, curr_coord.1))
    }
    if curr_coord.1 < col_len - distance {
        fn_mut((curr_coord.0, curr_coord.1 + distance))
    }
}

fn find_skips(curr_coord: (usize, usize), grid: &Vec<Vec<Space>>) -> Vec<Skip> {
    let current_val = match &grid[curr_coord.0][curr_coord.1] {
        Space::Path(Some(val)) => val,
        Space::End(Some(val)) => val,
        _ => panic!("curr_coord has to be a path"),
    };
    let mut walls = Vec::with_capacity(4);
    let find_walls = |(row, col): (usize, usize)| match &grid[row][col] {
        Space::Wall => walls.push((row, col)),
        _ => {}
    };
    check_surrounding_spaces(
        1,
        "find_walls".to_string(),
        curr_coord,
        grid.len(),
        grid[curr_coord.0].len(),
        find_walls,
    );
    let mut skips = Vec::with_capacity(4);
    let mut find_skips = |(row, col): (usize, usize)| match &grid[row][col] {
        Space::Path(Some(val)) if (row, col) != curr_coord && *current_val > *val + 2 => skips
            .push(Skip {
                start: (row, col),
                start_count: *val,
                end: curr_coord.clone(),
                time_save: current_val - val - 2,
            }),
        Space::Start if (row, col) != curr_coord && *current_val > 2 => skips.push(Skip {
            start: (row, col),
            start_count: 0,
            end: curr_coord.clone(),
            time_save: current_val - 2,
        }),
        _ => {}
    };
    for wall in walls {
        check_surrounding_spaces(
            1,
            "through_walls".to_string(),
            wall,
            grid.len(),
            grid[curr_coord.0].len(),
            &mut find_skips,
        );
    }
    skips
}

fn find_next_space(
    curr_coord: (usize, usize),
    curr_count: usize,
    grid: &mut Vec<Vec<Space>>,
) -> Option<(usize, usize)> {
    let mut next_space = None;
    let row_len = grid.len();
    let col_len = grid[curr_coord.0].len();
    let fn_mut = |(row, col): (usize, usize)| match grid[row][col] {
        Space::Path(ref mut val) => {
            if val.is_none() {
                *val = Some(curr_count + 1);
                next_space = Some((row, col));
            }
            return;
        }
        Space::End(ref mut val) => {
            if next_space.is_some() {
                panic!("found end and empty space");
            }
            if val.is_some() {
                panic!("reached end that already has a count");
            }
            *val = Some(curr_count + 1);
            next_space = Some((row, col));
        }
        _ => {
            println!("found nothing");
        }
    };
    check_surrounding_spaces(
        1,
        "next_space".to_string(),
        curr_coord,
        row_len,
        col_len,
        fn_mut,
    );
    next_space
}

#[derive(Debug, Clone)]
struct Skip {
    end: (usize, usize),
    start: (usize, usize),
    start_count: usize,
    time_save: usize,
}

fn find_all_skips(grid: &mut Vec<Vec<Space>>, start_coord: (usize, usize)) -> Vec<Skip> {
    let mut curr_coord = start_coord;
    let mut count = 0;
    let mut all_skips = Vec::new();
    while let Some(next_coord) = find_next_space(curr_coord, count, grid) {
        assert!(matches!(
            grid[next_coord.0][next_coord.1],
            Space::Path(Some(_)) | Space::End(Some(_))
        ));
        curr_coord = next_coord;
        count += 1;
        all_skips.append(&mut find_skips(curr_coord, grid));
    }
    all_skips
}

fn get_path(grid: &mut Vec<Vec<Space>>, start_coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut curr_coord = start_coord;
    let mut count = 0;
    let mut result = Vec::new();
    result.push(curr_coord);
    while let Some(next_coord) = find_next_space(curr_coord, count, grid) {
        assert!(matches!(
            grid[next_coord.0][next_coord.1],
            Space::Path(Some(_)) | Space::End(Some(_))
        ));
        curr_coord = next_coord;
        count += 1;
        result.push(curr_coord);
    }
    result
}

impl Puzzle for Day20 {
    fn puzzle_1(contents: String) {
        let mut start_coord = None;
        let mut grid =
            read_grid(contents)
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut acc, (row_idx, row)| {
                    let new_row = row
                        .iter()
                        .enumerate()
                        .map(|(col_idx, space_char)| {
                            let space = Space::try_from(*space_char)
                                .expect(&format!("has to be non-err. char: {space_char}"));
                            if matches!(space, Space::Start) {
                                start_coord = Some((row_idx.clone(), col_idx.clone()));
                            }
                            space
                        })
                        .collect::<Vec<_>>();
                    acc.push(new_row);
                    acc
                });

        let mut skips = find_all_skips(
            &mut grid,
            start_coord.expect("start_coord has to have been set"),
        );
        print_grid(grid.clone(), skips.clone());
        skips.sort_by(|a, b| a.start_count.cmp(&b.start_count));
        for skip in skips.iter() {
            println!("skip: {:?}", skip);
        }
        println!("num skips: {}", skips.len());
        let greater_than_100 = skips.iter().filter(|skip| skip.time_save >= 100).count();
        println!("found {greater_than_100} skips greater than 100");
    }

    fn puzzle_2(contents: String) {
        let mut start_coord = None;
        let mut grid =
            read_grid(contents)
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut acc, (row_idx, row)| {
                    let new_row = row
                        .iter()
                        .enumerate()
                        .map(|(col_idx, space_char)| {
                            let space = Space::try_from(*space_char)
                                .expect(&format!("has to be non-err. char: {space_char}"));
                            if matches!(space, Space::Start) {
                                start_coord = Some((row_idx.clone(), col_idx.clone()));
                            }
                            space
                        })
                        .collect::<Vec<_>>();
                    acc.push(new_row);
                    acc
                });
        let mut possible_skips = HashSet::new();
        let path = get_path(&mut grid, start_coord.expect("has to have a start"));
        let time_to_save = 100;
        for range in time_to_save..path.len() {
            for idx in range..path.len() {
                let start = path[idx - range];
                let end = path[idx];
                let distance = manhattan_distance(start, end);
                if distance <= 20 && range - distance >= time_to_save {
                    possible_skips.insert((path[idx - range], path[idx]));
                }
            }
        }
        println!(
            "Found a total of {} skips greater than or equal to 100",
            possible_skips.len()
        );
    }
}

fn manhattan_distance(start: (usize, usize), end: (usize, usize)) -> usize {
    let row_diff = if start.0 > end.0 {
        start.0 - end.0
    } else {
        end.0 - start.0
    };
    let col_diff = if start.1 > end.1 {
        start.1 - end.1
    } else {
        end.1 - start.1
    };
    row_diff + col_diff
}

fn print_grid(grid: Vec<Vec<Space>>, skips: Vec<Skip>) {
    println!("====Start====");
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, space) in row.iter().enumerate() {
            match space {
                Space::Path(Some(val)) => print!("[{:0>3}]", val),
                Space::Path(None) => panic!("shouldn't happen"),
                Space::Wall => print!("[###]"),
                Space::Start => print!("[SSS]"),
                Space::End(_) => print!("[EEE]"),
            }
        }
        println!();
    }
    println!("====End====");
}

fn main() {
    Day20::run()
}
