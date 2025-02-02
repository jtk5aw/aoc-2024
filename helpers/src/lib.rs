use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::{env, fs, path::Path};

pub fn read_grid(contents: String) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub trait Puzzle {
    fn puzzle_1(contents: String);
    fn puzzle_2(contents: String);

    fn run() {
        let args: Vec<String> = env::args().collect();

        let puzzle_num: &i64 = &args[1].parse().unwrap();
        let output = std::process::Command::new(env!("CARGO"))
            .arg("locate-project")
            .arg("--workspace")
            .arg("--message-format=plain")
            .output()
            .unwrap()
            .stdout;
        let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
        let file_path = cargo_path
            .parent()
            .unwrap()
            .to_path_buf()
            .join("inputs")
            .join(&args[2]);

        println!("reading: {:?}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        if *puzzle_num == 1 as i64 {
            Self::puzzle_1(contents);
        } else if *puzzle_num == 2 as i64 {
            Self::puzzle_2(contents);
        } else {
            println!("bad puzzle num");
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
enum HeapKind {
    Min,
    Max,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Default)]
pub struct HeapNode<T> {
    priority: usize,
    value: T,
}

impl<T> HeapNode<T> {
    pub fn new(value: T) -> Self {
        HeapNode {
            priority: usize::MAX,
            value,
        }
    }

    pub fn with_priority(value: T, priority: usize) -> Self {
        HeapNode { priority, value }
    }

    pub fn priority(&self) -> usize {
        self.priority
    }
}

#[derive(Clone)]
pub struct IndexedBinaryHeap<T> {
    values: Vec<HeapNode<T>>,
    indeces: HashMap<T, usize>,
    kind: HeapKind,
}

impl<T> IndexedBinaryHeap<T>
where
    T: Debug + Default + Clone + Eq + Hash,
{
    pub fn min() -> Self {
        Self::new(HeapKind::Min)
    }

    pub fn max() -> Self {
        Self::new(HeapKind::Max)
    }

    fn new(kind: HeapKind) -> Self {
        let mut values = Vec::new();
        values.push(HeapNode::new(T::default()));
        let indeces = HashMap::new();
        Self {
            values,
            indeces,
            kind,
        }
    }

    fn first_is_of_kind(&self, first_priority: usize, second_priority: usize) -> bool {
        match self.kind {
            HeapKind::Min => first_priority < second_priority,
            HeapKind::Max => first_priority > second_priority,
        }
    }

    pub fn get_value(&self, key: &T) -> Option<&HeapNode<T>> {
        self.indeces.get(key).map(|idx| self.values.get(*idx))?
    }

    fn heapify_down(&mut self, index: usize) {
        let left_index = 2 * index;
        let right_index = 2 * index + 1;
        if let Some(curr_node) = self.values.get(index) {
            match (self.values.get(left_index), self.values.get(right_index)) {
                (None, None) => {}
                (Some(left_node), None) => {
                    self.swap_both(
                        (&left_node.value.clone(), left_index),
                        (&curr_node.value.clone(), index),
                    );
                }
                (Some(left_node), Some(right_node)) => {
                    if self.first_is_of_kind(left_node.priority, right_node.priority) {
                        self.swap_both(
                            (&left_node.value.clone(), left_index),
                            (&curr_node.value.clone(), index),
                        );
                        self.heapify_down(left_index);
                    } else {
                        self.swap_both(
                            (&right_node.value.clone(), right_index),
                            (&curr_node.value.clone(), index),
                        );
                        self.heapify_down(right_index);
                    }
                }
                (None, Some(_)) => panic!("bad heap shape"),
            };
        }
    }

    pub fn pop(&mut self) -> Option<HeapNode<T>> {
        if self.values.len() < 2 {
            return None;
        }
        let original_len = self.values.len();
        let result = self.swap_start_and_end();
        self.heapify_down(1);

        let val = self.indeces.remove(&result.value);
        assert!(val.is_some());

        assert_eq!(self.values.len(), original_len - 1);
        Some(result)
    }

    // Only need to set one index because the other value is removed
    fn swap_start_and_end(&mut self) -> HeapNode<T> {
        let result = self.values.swap_remove(1);
        if let Some(first_node) = self.values.get(1) {
            let index_to_update = self
                .indeces
                .get_mut(&first_node.value)
                .expect("should have key at this point");
            *index_to_update = 1;
        }
        result
    }

    // Update both indeces cause they're both real values
    fn swap_both(&mut self, first: (&T, usize), second: (&T, usize)) {
        let first_index_to_update = self
            .indeces
            .get_mut(first.0)
            .expect("should have key at this point");
        *first_index_to_update = second.1;
        let second_index_to_update = self
            .indeces
            .get_mut(second.0)
            .expect("should have key at thsi point");
        *second_index_to_update = first.1;
        self.values.swap(first.1, second.1);
    }

    fn heapify_up(&mut self, index: usize) {
        if index == 1 {
            return;
        }
        let parent_index = index / 2;
        let current_node = self
            .values
            .get(index)
            .expect("has to be value at current index");
        match self.values.get(parent_index) {
            Some(parent_node) => {
                if self.first_is_of_kind(current_node.priority, parent_node.priority) {
                    self.swap_both(
                        (&parent_node.value.clone(), parent_index),
                        (&current_node.value.clone(), index),
                    );
                    self.heapify_up(parent_index);
                }
            }
            None => panic!("shouldn't have invalid parent when heapifying up"),
        }
    }

    pub fn push(&mut self, node: HeapNode<T>) {
        self.values.push(node.clone());
        if self
            .indeces
            .insert(node.value, self.values.len() - 1)
            .is_some()
        {
            panic!("already inserted this key");
        }
        self.heapify_up(self.values.len() - 1);
    }

    pub fn attempt_update_key(&mut self, key: &T, new_priority: usize) -> UpdateKeyResult {
        if let Some(index) = self.indeces.get(key) {
            let priority_to_update = self
                .values
                .get(*index)
                .expect("provided index must be correct");
            assert!(priority_to_update.value == *key);
            if self.first_is_of_kind(new_priority, priority_to_update.priority) {
                let mutable = self.values.get_mut(*index).expect("got above");
                mutable.priority = new_priority;
                self.heapify_up(*index);
                return UpdateKeyResult::SuccessfullyUpdated;
            } else if new_priority == priority_to_update.priority {
                return UpdateKeyResult::NoUpdateEqual;
            }
            return UpdateKeyResult::NoUpdateWrongKind;
        }
        return UpdateKeyResult::NoUpdateKeyDoesNotExist;
    }

    pub fn print_binary_tree(&self) {
        println!("===START===");
        for (idx, value) in self.values.iter().enumerate() {
            println!(
                "IDX: {idx}, priority: {} value: {:?}",
                value.priority, value.value
            );
        }
        let keys = self.indeces.keys().collect::<Vec<_>>();
        for key in keys {
            println!(
                "key: {:?}, index: {:?}",
                key,
                self.indeces.get(key).expect("has to exist")
            );
        }
        println!("====END====");
    }
}

pub enum UpdateKeyResult {
    NoUpdateKeyDoesNotExist,
    NoUpdateWrongKind,
    NoUpdateEqual,
    SuccessfullyUpdated,
}

#[cfg(test)]
mod test_heap {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::{HeapNode, IndexedBinaryHeap, UpdateKeyResult};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default, Hash)]
    struct TestStruct {
        name: String,
    }

    fn with_priority(priority: usize) -> HeapNode<TestStruct> {
        let count = COUNTER.fetch_add(1, Ordering::SeqCst);
        HeapNode::with_priority(
            TestStruct {
                name: format!("counter: {count}"),
            },
            priority,
        )
    }

    #[test]
    fn test_heap_basic() {
        let mut heap = IndexedBinaryHeap::min();
        heap.push(with_priority(5));
        heap.print_binary_tree();
        heap.push(with_priority(3));
        heap.push(with_priority(4));
        heap.print_binary_tree();
        heap.push(with_priority(2));
        heap.print_binary_tree();
        heap.push(with_priority(15));
        heap.push(with_priority(15));
        heap.push(with_priority(10));
        heap.push(with_priority(10));
        heap.push(with_priority(10));
        heap.print_binary_tree();
        heap.push(with_priority(4));
        heap.print_binary_tree();
        let two = heap.pop().unwrap();
        assert_eq!(2, two.priority);
        heap.print_binary_tree();
        let three = heap.pop().unwrap();
        assert_eq!(3, three.priority);
        let four = heap.pop().unwrap();
        assert_eq!(4, four.priority);
        let four = heap.pop().unwrap();
        assert_eq!(4, four.priority);
        heap.print_binary_tree();
    }

    #[test]
    fn test_heap_decrement_key() {
        let mut heap = IndexedBinaryHeap::min();
        heap.push(with_priority(5));
        heap.push(with_priority(3));
        heap.push(with_priority(4));
        heap.push(with_priority(2));
        heap.push(with_priority(15));
        heap.push(with_priority(15));
        heap.push(with_priority(10));
        heap.push(with_priority(10));
        let node_to_track = with_priority(15);
        heap.push(node_to_track.clone());
        heap.print_binary_tree();
        println!("node_to_track: {:?}", node_to_track);
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 20),
            UpdateKeyResult::NoUpdateWrongKind
        ));
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 13),
            UpdateKeyResult::SuccessfullyUpdated
        ));
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 4),
            UpdateKeyResult::SuccessfullyUpdated
        ));
        heap.print_binary_tree();
        let value = heap.pop().expect("has to exist");
        assert_eq!(2_usize, value.priority);
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 2),
            UpdateKeyResult::SuccessfullyUpdated
        ));
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 20),
            UpdateKeyResult::NoUpdateWrongKind
        ));
        heap.print_binary_tree();
        let value = heap.pop().expect("has to exist");
        assert_eq!(2_usize, value.priority);
        assert_eq!(node_to_track.value, value.value);
    }

    #[test]
    fn test_heap_increment_key() {
        let mut heap = IndexedBinaryHeap::max();
        heap.push(with_priority(5));
        heap.push(with_priority(3));
        heap.push(with_priority(4));
        heap.push(with_priority(2));
        heap.push(with_priority(15));
        heap.push(with_priority(15));
        heap.push(with_priority(10));
        heap.push(with_priority(10));
        let node_to_track = with_priority(1);
        heap.push(node_to_track.clone());
        heap.print_binary_tree();
        println!("node_to_track: {:?}", node_to_track);
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 0),
            UpdateKeyResult::NoUpdateWrongKind
        ));
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 10),
            UpdateKeyResult::SuccessfullyUpdated
        ));
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 13),
            UpdateKeyResult::SuccessfullyUpdated
        ));
        heap.print_binary_tree();
        let value = heap.pop().expect("has to exist");
        assert_eq!(15_usize, value.priority);
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 20),
            UpdateKeyResult::SuccessfullyUpdated
        ));
        heap.print_binary_tree();
        assert!(matches!(
            heap.attempt_update_key(&node_to_track.value, 2),
            UpdateKeyResult::NoUpdateWrongKind
        ));
        heap.print_binary_tree();
        let value = heap.pop().expect("has to exist");
        assert_eq!(20_usize, value.priority);
        assert_eq!(node_to_track.value, value.value);
        heap.print_binary_tree();
    }
}
