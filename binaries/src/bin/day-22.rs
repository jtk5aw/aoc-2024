use std::collections::{HashSet, VecDeque};

use helpers::{HeapNode, IndexedBinaryHeap, Puzzle, UpdateKeyResult};

struct Day22;

struct FixedSequence {
    queue: VecDeque<isize>,
    capacity: usize,
}

impl FixedSequence {
    fn new(capacity: usize) -> FixedSequence {
        Self {
            queue: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn add(&mut self, to_add: isize) -> bool {
        self.queue.push_front(to_add);
        if self.queue.len() > self.capacity {
            self.queue.pop_back();
        }
        self.queue.len() == self.capacity
    }

    fn get(&self) -> Option<VecDeque<isize>> {
        if self.queue.len() == self.capacity {
            return Some(self.queue.clone());
        }
        None
    }
}

impl Puzzle for Day22 {
    fn puzzle_1(contents: String) {
        let total: isize = contents
            .lines()
            .map(|line_str| {
                println!("{line_str}");
                line_str.parse::<isize>().expect("has to be a num")
            })
            .map(|num| {
                let mut secret = num;
                for _ in 0..2000 {
                    let mult_64 = secret * 64;
                    secret = mix(secret, mult_64);
                    secret = prune(secret);
                    let div_32 = secret / 32;
                    secret = mix(secret, div_32);
                    secret = prune(secret);
                    let mult_2048 = secret * 2048;
                    secret = mix(secret, mult_2048);
                    secret = prune(secret);
                    println!("secret: {secret}");
                }
                secret
            })
            .sum();
        println!("The sum is: {total}");
    }

    fn puzzle_2(contents: String) {
        // TODO TODO TODO: Keep a heap of the current sum you'd get for given sequences
        // update this as you go
        // need to keep in mind that when going through sequences for a given monkey's secret
        // number you will only sell on the first appearance of a sequecne
        let mut heap = IndexedBinaryHeap::<VecDeque<isize>>::max();
        let test: isize = contents
            .lines()
            .enumerate()
            .map(|(idx, line_str)| {
                //println!("{line_str}");
                (idx, line_str.parse::<isize>().expect("has to be a num"))
            })
            .map(|(idx, num)| {
                //println!("{idx} num: {num}");
                let mut prev_secret = num;
                let mut new_secret = num;
                let mut fixed_sequence = FixedSequence::new(4);
                // Once a sequence is seen it will be sold on first view
                let mut already_seen = HashSet::new();
                for _ in 0..2000 {
                    // Calc new secret
                    let mult_64 = new_secret * 64;
                    new_secret = mix(new_secret, mult_64);
                    new_secret = prune(new_secret);
                    let div_32 = new_secret / 32;
                    new_secret = mix(new_secret, div_32);
                    new_secret = prune(new_secret);
                    let mult_2048 = new_secret * 2048;
                    new_secret = mix(new_secret, mult_2048);
                    new_secret = prune(new_secret);

                    // Check current sequence
                    let new_sell_value = new_secret % 10;
                    let diff = new_sell_value - (prev_secret % 10);
                    //println!("{new_secret}: {new_sell_value} {:?}", diff);
                    if fixed_sequence.add(diff) && already_seen.insert(fixed_sequence.queue.clone())
                    {
                        if let Some(value) = heap.get_value(&fixed_sequence.queue) {
                            let update_result = heap.attempt_update_key(
                                &fixed_sequence.queue,
                                value.priority() + new_sell_value as usize,
                            );
                            assert!(matches!(
                                update_result,
                                UpdateKeyResult::SuccessfullyUpdated
                                    | UpdateKeyResult::NoUpdateEqual
                            ));
                        } else {
                            heap.push(HeapNode::with_priority(
                                fixed_sequence.queue.clone(),
                                new_sell_value as usize,
                            ));
                        }
                    }

                    // Iteration cleanup
                    already_seen.insert(fixed_sequence.queue.clone());
                    prev_secret = new_secret;
                }
                new_secret
            })
            .sum();

        //heap.print_binary_tree();
        for i in 1..=10 {
            println!("top {i} value: {:?}", heap.pop());
        }
        println!("the sum was: {test}");
    }
}

fn prune(num: isize) -> isize {
    num % 16777216
}

fn mix(num: isize, mixin: isize) -> isize {
    num ^ mixin
}

fn main() {
    Day22::run();
}
