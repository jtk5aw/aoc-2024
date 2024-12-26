use helpers::Puzzle;

fn main() {
    Day2::run();
}

struct Day2;

impl Puzzle for Day2 {
    fn puzzle_1(contents: String) {
        let mut safe_count = 0;
        for line in contents.lines() {
            let levels = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().expect("Has to be a number"));
            if solve(levels).0 {
                safe_count += 1;
            }
        }

        println!("Number safe: {safe_count}");
    }

    fn puzzle_2(contents: String) {
        let mut safe_count = 0;
        for line in contents.lines() {
            let levels: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().expect("Has to be a number"))
                .collect();
            let first_try = solve(levels.clone().into_iter());
            if first_try.0 {
                safe_count += 1;
                continue;
            }

            let mut remove_first = levels.clone();
            remove_first.remove(0);
            if solve(remove_first.into_iter()).0 {
                safe_count += 1;
                continue;
            }

            let mut remove_idx = levels.clone();
            remove_idx.remove(first_try.1);
            if solve(remove_idx.into_iter()).0 {
                safe_count += 1;
                continue;
            }

            let mut remove_idx_minus_one = levels.clone();
            remove_idx_minus_one.remove(first_try.1 - 1);
            if solve(remove_idx_minus_one.into_iter()).0 {
                safe_count += 1;
                continue;
            }
        }

        println!("Number safe: {safe_count}");
    }
}

fn solve(mut levels: impl Iterator<Item = i64>) -> (bool, usize) {
    let first_level = levels.next().expect("Has at least one value");
    let mut last_level = levels.next().expect("Has to have second value");

    if first_level == last_level || (first_level - last_level).abs() > 3 {
        return (false, 1);
    }
    let is_increasing = first_level < last_level;

    for (idx, level) in levels.enumerate() {
        if is_increasing && (level - last_level > 3 || level <= last_level) {
            return (false, idx + 2);
        }

        if !is_increasing && (level - last_level < -3 || level >= last_level) {
            return (false, idx + 2);
        }

        last_level = level;
    }
    // ignore the idx when returning true
    return (true, 0);
}
