use std::collections::{hash_map::Entry, HashMap, HashSet};

use helpers::Puzzle;

struct Day21;

struct Coord(isize, isize);
trait Coords {
    fn get_coord(&self) -> Coord;
    fn avoid_coord(&self) -> Coord;
}

trait TransitionSequence<T> {
    fn get_transition_sequence(&self, other: &Self) -> Vec<T>;
}

struct DistancesWithHint {
    distances: Distances,
    blocked_starting_direction: Option<Direction>,
}

impl<Input, Output> TransitionSequence<Output> for Input
where
    Input: Coords,
    Vec<Output>: From<DistancesWithHint>,
{
    /// Order of values in the returned Vec may cause a robot to panic.
    /// This is because it may go over the empty square
    fn get_transition_sequence(&self, end: &Input) -> Vec<Output> {
        let start_coord = self.get_coord();
        // println!(
        //     "self_coord ({:?}): ({}, {})",
        //     self, self_coord.0, self_coord.1
        // );
        let end_coord = end.get_coord();
        // println!(
        //     "other_coord ({:?}): ({}, {})",
        //     other, other_coord.0, other_coord.1
        // );
        let distances = distances(&start_coord, &end_coord);
        //println!("distances: ({}, {})", distances.0, distances.1);
        let coord_to_avoid = self.avoid_coord();
        let other_coord_to_avoid = end.avoid_coord();
        assert!(
            coord_to_avoid.0 == other_coord_to_avoid.0
                && coord_to_avoid.1 == other_coord_to_avoid.1
        );
        let blocked_starting_direction = if start_coord.0 == coord_to_avoid.0
            && start_coord.1 + distances.1 == coord_to_avoid.1
        {
            assert!(distances.1.is_negative());
            Some(Direction::Left)
        } else if start_coord.1 == coord_to_avoid.1
            && start_coord.0 + distances.0 == coord_to_avoid.0
        {
            Some(if distances.0.is_positive() {
                Direction::Up
            } else {
                Direction::Down
            })
        } else {
            None
        };
        //println!("horiztonal_first: {horizontal_first}");

        DistancesWithHint {
            distances,
            blocked_starting_direction,
        }
        .into()
    }
}

#[derive(Clone, Debug)]
enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    Activate,
}

struct Distances(isize, isize);

fn distances(first: &Coord, second: &Coord) -> Distances {
    Distances(second.0 - first.0, second.1 - first.1)
}

impl TryFrom<char> for Number {
    type Error = ();

    fn try_from(num_char: char) -> Result<Self, ()> {
        match num_char {
            'A' => Ok(Self::Activate),
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            _ => Err(()),
        }
    }
}

impl Coords for Number {
    ///
    /// +---+---+---+
    /// | 7 | 8 | 9 |
    /// +---+---+---+
    /// | 4 | 5 | 6 |
    /// +---+---+---+
    /// | 1 | 2 | 3 |
    /// +---+---+---+
    ///     | 0 | A |
    ///     +---+---+
    /// With the blank space being (0, 0): (row, col)
    fn get_coord(&self) -> Coord {
        match self {
            Number::Zero => Coord(0, 1),
            Number::Activate => Coord(0, 2),
            Number::One => Coord(1, 0),
            Number::Two => Coord(1, 1),
            Number::Three => Coord(1, 2),
            Number::Four => Coord(2, 0),
            Number::Five => Coord(2, 1),
            Number::Six => Coord(2, 2),
            Number::Seven => Coord(3, 0),
            Number::Eight => Coord(3, 1),
            Number::Nine => Coord(3, 2),
        }
    }

    fn avoid_coord(&self) -> Coord {
        Coord(0, 0)
    }
}

impl Coords for Direction {
    ///
    ///     +---+---+
    ///     | ^ | A |
    /// +---+---+---+
    /// | < | v | > |
    /// +---+---+---+
    /// where (0, 0) : (row, col) is the < space
    fn get_coord(&self) -> Coord {
        match self {
            Direction::Left => Coord(0, 0),
            Direction::Down => Coord(0, 1),
            Direction::Right => Coord(0, 2),
            Direction::Up => Coord(1, 1),
            Direction::Activate => Coord(1, 2),
        }
    }

    fn avoid_coord(&self) -> Coord {
        Coord(1, 0)
    }
}

impl From<DistancesWithHint> for Vec<Direction> {
    fn from(value: DistancesWithHint) -> Self {
        let distances = value.distances;
        let mut vertical_directions = if distances.0.is_positive() {
            vec![Direction::Up; distances.0 as usize]
        } else if distances.0.is_negative() {
            vec![Direction::Down; distances.0.abs() as usize]
        } else {
            vec![]
        };
        let mut horizontal_directions = if distances.1.is_positive() {
            vec![Direction::Right; distances.1 as usize]
        } else if distances.1.is_negative() {
            vec![Direction::Left; distances.1.abs() as usize]
        } else {
            vec![]
        };
        let mut result =
            Vec::with_capacity(vertical_directions.len() + horizontal_directions.len() + 1);
        match value.blocked_starting_direction {
            Some(blocked_direction) => {
                match (vertical_directions.get(0), horizontal_directions.get(0)) {
                    (Some(vertical), Some(horizontal)) => {
                        if blocked_direction == *vertical {
                            result.append(&mut horizontal_directions);
                            result.append(&mut vertical_directions);
                        } else if blocked_direction == *horizontal {
                            result.append(&mut vertical_directions);
                            result.append(&mut horizontal_directions);
                        } else {
                            panic!("has to equal one of the directions");
                        }
                    }
                    (Some(_), None) | (None, Some(_)) => {
                        panic!("shouldn't block for uni-directional")
                    }
                    (None, None) => panic!("impossible, can't block for same space"),
                }
            }
            None => match (vertical_directions.get(0), horizontal_directions.get(0)) {
                // Not going anywhere (distances are both 0)
                (None, None) => {}
                // Going somewhere for all of these
                (None, Some(_)) => result.append(&mut horizontal_directions),
                (Some(_), None) => result.append(&mut vertical_directions),
                (Some(_), Some(Direction::Left)) => {
                    result.append(&mut horizontal_directions);
                    result.append(&mut vertical_directions);
                }
                (Some(_), Some(Direction::Right)) => {
                    result.append(&mut vertical_directions);
                    result.append(&mut horizontal_directions);
                }
                _ => panic!("above match arms should cover all legitamate (spelling?) cases"),
            },
        }
        result.push(Direction::Activate);
        result
    }
}

fn next_sequence(initial_sequence: Vec<Direction>) -> Vec<Direction> {
    let next_sequence = initial_sequence
        .into_iter()
        // Second robots directional keypad
        .fold(
            (Vec::new(), Direction::Activate),
            |(mut sequence, previous_direction), direction| {
                sequence.append(&mut previous_direction.get_transition_sequence(&direction));
                (sequence, direction)
            },
        )
        .0;
    // println!(
    //     "next_sequence: len({}) {:?}",
    //     next_sequence.len(),
    //     next_sequence
    // );
    next_sequence
}

impl Puzzle for Day21 {
    fn puzzle_1(contents: String) {
        let final_sum: usize = contents
            .lines()
            .map(|line| {
                println!("sequence being created for {line}");
                let num = line[0..3].parse::<usize>().expect("has to be a usize");
                let first_sequence = line
                    .chars()
                    .map(|char| Number::try_from(char).expect("should be valid number"))
                    // First robots directional keypad
                    .fold(
                        (Vec::new(), Number::Activate),
                        |(mut sequence, previous_number), number| {
                            sequence.append(&mut previous_number.get_transition_sequence(&number));
                            //println!("sequence: {:?}", sequence);
                            (sequence, number)
                        },
                    )
                    .0;
                println!(
                    "first_sequence: len({}) {:?}",
                    first_sequence.len(),
                    first_sequence
                );
                let second_sequence = next_sequence(first_sequence);
                let third_sequence = next_sequence(second_sequence);
                num * third_sequence.len()
            })
            .sum();
        println!("final sum is: {final_sum}");
    }

    fn puzzle_2(contents: String) {
        let final_sum: usize = contents
            .lines()
            .map(|line| {
                println!("sequence being created for {line}");
                let num = line[0..3].parse::<usize>().expect("has to be a usize");
                let first_sequence = line
                    .chars()
                    .map(|char| Number::try_from(char).expect("should be valid number"))
                    // First robots directional keypad
                    .fold(
                        (Vec::new(), Number::Activate),
                        |(mut sequence, previous_number), number| {
                            sequence.append(&mut previous_number.get_transition_sequence(&number));
                            //println!("sequence: {:?}", sequence);
                            (sequence, number)
                        },
                    )
                    .0;
                println!(
                    "first_sequence: len({}) {:?}",
                    first_sequence.len(),
                    first_sequence
                );
                let mut memoized = HashMap::new();
                let total_len = calculate_sequence(&mut memoized, first_sequence, 1, 26);
                println!("total_len is {total_len}");
                num * total_len
            })
            .sum();
        println!("final sum is: {final_sum}");
    }
}

fn calculate_sequence(
    memoized: &mut HashMap<MemoizedKey, usize>,
    sequence: Vec<Direction>,
    depth: usize,
    max_depth: usize,
) -> usize {
    if depth == max_depth {
        return sequence.len();
    }
    let key = MemoizedKey {
        sequence: sequence.clone(),
        depth,
    };
    if let Some(memoized_value) = memoized.get(&key) {
        println!("cache hit!");
        return *memoized_value;
    }
    let next_sequence = next_sequence(sequence);
    let result = split_on_activate(next_sequence)
        .into_iter()
        .map(|curr_sequence| calculate_sequence(memoized, curr_sequence, depth + 1, max_depth))
        .sum();
    memoized.insert(key, result);
    result
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct MemoizedKey {
    sequence: Vec<Direction>,
    depth: usize,
}

fn split_on_activate(full_sequence: Vec<Direction>) -> Vec<Vec<Direction>> {
    let mut result = Vec::new();
    let mut intermediate = Vec::new();
    for direction in full_sequence {
        intermediate.push(direction.clone());
        if matches!(direction, Direction::Activate) {
            result.push(intermediate.clone());
            intermediate.clear();
        }
    }
    result
}

fn main() {
    Day21::run();
}
