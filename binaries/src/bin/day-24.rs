use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use helpers::Puzzle;

struct Day24;

#[derive(Debug)]
struct Wire {
    name: String,
    feeds_into: Vec<Rc<RefCell<Gate>>>,
}

impl Wire {
    fn trigger(self, value: u8) -> Vec<(String, u8)> {
        self.feeds_into
            .into_iter()
            .flat_map(|gate_ref| {
                let mut gate = gate_ref.borrow_mut();
                gate.transition(value)
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
struct Gate {
    state: ComputeState,
    kind: GateKind,
    output_wire_name: String,
}

impl Gate {
    fn transition(&mut self, value: u8) -> Option<(String, u8)> {
        self.state = self.state.clone().transition(value);
        if let ComputeState::Both(first_val, second_val) = self.state {
            let computed_value = self.kind.compute(first_val, second_val);
            return Some((self.output_wire_name.to_string(), computed_value));
        }
        None
    }
}

#[derive(Debug, Clone)]
enum ComputeState {
    None,
    One(u8),
    Both(u8, u8),
}

impl ComputeState {
    fn transition(self, new_value: u8) -> Self {
        match self {
            ComputeState::None => Self::One(new_value),
            ComputeState::One(old_value) => Self::Both(old_value, new_value),
            ComputeState::Both(_, _) => panic!("already computed"),
        }
    }
}

#[derive(Debug)]
enum GateKind {
    And,
    Or,
    Xor,
}

impl GateKind {
    fn compute(&self, first_val: u8, second_val: u8) -> u8 {
        match self {
            GateKind::And => first_val & second_val,
            GateKind::Or => first_val | second_val,
            GateKind::Xor => first_val ^ second_val,
        }
    }
}

impl TryFrom<&str> for GateKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "XOR" => Ok(GateKind::Xor),
            "AND" => Ok(GateKind::And),
            "OR" => Ok(GateKind::Or),
            _ => Err(()),
        }
    }
}

impl Puzzle for Day24 {
    fn puzzle_1(contents: String) {
        let mut line_iter = contents.lines();

        // Get starting values and initial wires
        let mut original_wires = HashMap::new();
        let mut starting_values = VecDeque::new();
        while let Some(next_line) = line_iter.next() {
            if next_line.is_empty() {
                break;
            }
            let (name, remainder) = next_line.split_once(":").expect("has to have colon");
            let value = remainder
                .strip_prefix(" ")
                .expect("has to start with blank")
                .parse::<u8>()
                .expect("has to be a byte");
            starting_values.push_back((name.to_string(), value));
            original_wires.insert(
                name.to_string(),
                Wire {
                    name: name.to_string(),
                    feeds_into: Vec::new(),
                },
            );
        }

        let mut final_wires = line_iter
            .map(|curr_line| curr_line.split_once(" -> ").expect("has to have \" -> \""))
            .fold(original_wires, |mut wires, (input_def, output_def)| {
                let input_split = input_def.split_whitespace().collect::<Vec<_>>();
                if input_split.len() != 3 {
                    panic!("ran wrong");
                }
                let first_wire_name = input_split[0];
                let second_wire_name = input_split[2];
                let kind = GateKind::try_from(input_split[1]).expect("has to be an action");
                let output_wire = Wire {
                    name: output_def.to_string(),
                    feeds_into: Vec::new(),
                };

                assert!(wires.insert(output_def.to_string(), output_wire).is_none());

                let new_gate = Gate {
                    state: ComputeState::None,
                    kind,
                    output_wire_name: output_def.to_string(),
                };
                let gate_ref = Rc::new(RefCell::new(new_gate));
                // TODO TODO TODO: you're not guaranteed ot see the output wire
                // before it is used as an input. So that means these next lines will fail
                // Try to understand if that'll be an issue. I don't think it will be I just need
                // to add a or_insert_with
                println!("current_wires: {:?}", wires);
                println!(
                    "first_wire_name: {}, second_wire_name: {}",
                    first_wire_name, second_wire_name
                );
                wires
                    .get_mut(first_wire_name)
                    .expect("has to have already been inserted")
                    .feeds_into
                    .push(gate_ref.clone());

                wires
                    .get_mut(second_wire_name)
                    .expect("has to have already been inserted")
                    .feeds_into
                    .push(gate_ref.clone());

                wires
            });

        let mut to_process = starting_values;
        let mut result = HashMap::new();
        while let Some((name, byte)) = to_process.pop_front() {
            result.insert(name.to_string(), byte);
            let wire = final_wires
                .remove(&name)
                .expect("should have been able to pull the wire");
            wire.trigger(byte)
                .into_iter()
                .for_each(|new_to_process| to_process.push_back(new_to_process));
        }

        let mut keys = result.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            println!(
                "key:  {}, value: {}",
                key,
                result.get(key).expect("has to exist")
            );
        }
    }

    fn puzzle_2(contents: String) {
        todo!()
    }
}

fn main() {
    Day24::run()
}
