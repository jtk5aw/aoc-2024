use helpers::Puzzle;

struct Day25;

#[derive(Debug)]
struct Counters {
    counts: [u32; 5],
    kind: KeyOrLock,
}

#[derive(Debug)]
enum KeyOrLock {
    Key,
    Lock,
}

impl Counters {
    fn fits(key: &Counters, lock: &Counters) -> bool {
        if !matches!(key.kind, KeyOrLock::Key) || !matches!(lock.kind, KeyOrLock::Lock) {
            panic!("bad input");
        }
        for idx in 0..5 {
            if key.counts[idx] + lock.counts[idx] > 5 {
                return false;
            }
        }
        true
    }
}

impl Puzzle for Day25 {
    fn puzzle_1(contents: String) {
        let mut iter = contents.lines();
        let mut keys = Vec::new();
        let mut locks = Vec::new();

        while let Some(first_line) = iter.next() {
            let first_char = first_line.chars().next().unwrap();
            let kind = match first_char {
                '#' => KeyOrLock::Lock,
                '.' => KeyOrLock::Key,
                _ => panic!("This shouldn't ever happen"),
            };

            let counts =
                iter.by_ref()
                    .take(5)
                    .map(|line| line.chars())
                    .fold([0; 5], |mut acc, mut line| {
                        acc[0] += if line.next().unwrap() == '#' { 1 } else { 0 };
                        acc[1] += if line.next().unwrap() == '#' { 1 } else { 0 };
                        acc[2] += if line.next().unwrap() == '#' { 1 } else { 0 };
                        acc[3] += if line.next().unwrap() == '#' { 1 } else { 0 };
                        acc[4] += if line.next().unwrap() == '#' { 1 } else { 0 };
                        assert!(line.next().is_none());
                        acc
                    });

            match kind {
                KeyOrLock::Lock => locks.push(Counters { counts, kind }),
                KeyOrLock::Key => keys.push(Counters { counts, kind }),
            }

            // Skip the final line of the structure and then the blank line
            iter.next();
            iter.next();
        }

        println!("keys: {:?}", keys);
        println!("locks: {:?}", locks);

        let sum: usize = locks
            .iter()
            .map(|lock| keys.iter().filter(|key| Counters::fits(key, lock)).count())
            .sum();

        println!("The number of matches is {sum}");
    }

    fn puzzle_2(contents: String) {
        todo!()
    }
}

fn main() {
    Day25::run()
}
