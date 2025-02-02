use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    convert,
    rc::Rc,
};

use helpers::Puzzle;

struct Day24;

#[derive(Clone, Debug)]
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

fn build_wires(contents: String) -> (VecDeque<(String, u8)>, HashMap<String, Wire>) {
    let mut line_iter = contents.lines();

    // Get starting values and initial wires
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
    }

    let wires = line_iter
        .map(|curr_line| curr_line.split_once(" -> ").expect("has to have \" -> \""))
        .fold(HashMap::new(), |mut wires, (input_def, output_def)| {
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

            if let None = wires.get(output_def) {
                wires.insert(output_def.to_string(), output_wire);
            }

            let new_gate = Gate {
                state: ComputeState::None,
                kind,
                output_wire_name: output_def.to_string(),
            };
            let gate_ref = Rc::new(RefCell::new(new_gate));
            println!("found gate!");
            // println!("current_wires: {:?}", wires);
            // println!(
            //     "first_wire_name: {}, second_wire_name: {}",
            //     first_wire_name, second_wire_name
            // );
            wires
                .entry(first_wire_name.to_string())
                .and_modify(|wire| wire.feeds_into.push(gate_ref.clone()))
                .or_insert_with(|| Wire {
                    name: first_wire_name.to_string(),
                    feeds_into: vec![gate_ref.clone()],
                });
            wires
                .entry(second_wire_name.to_string())
                .and_modify(|wire| wire.feeds_into.push(gate_ref.clone()))
                .or_insert_with(|| Wire {
                    name: second_wire_name.to_string(),
                    feeds_into: vec![gate_ref.clone()],
                });

            wires
        });

    (starting_values, wires)
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

fn run_circuit(
    starting_values: VecDeque<(String, u8)>,
    mut wires: HashMap<String, Wire>,
) -> HashMap<String, u8> {
    let mut to_process = starting_values;
    let mut result = HashMap::new();
    while let Some((name, byte)) = to_process.pop_front() {
        result.insert(name.to_string(), byte);
        let wire = wires
            .remove(&name)
            .expect("should have been able to pull the wire");
        wire.trigger(byte)
            .into_iter()
            .for_each(|new_to_process| to_process.push_back(new_to_process));
    }
    result
}

impl Puzzle for Day24 {
    fn puzzle_1(contents: String) {
        let (starting_values, final_wires) = build_wires(contents);

        let result = run_circuit(starting_values, final_wires);

        let mut keys = result.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys.iter() {
            println!(
                "key:  {}, value: {}",
                key,
                result.get(*key).expect("has to exist")
            );
        }

        let (x, _) = convert_to_usize('x', &result);
        let (y, _) = convert_to_usize('y', &result);
        let (num, _) = convert_to_usize('z', &result);
        println!("x + y = z: {x} + {y} = {num}");
    }

    fn puzzle_2(contents: String) {
        let (starting_points, initial_wires) = build_wires(contents);

        let initial_result = run_circuit(starting_points.clone(), initial_wires.clone());
        let (x, _) = convert_to_usize('x', &initial_result);
        let (y, _) = convert_to_usize('y', &initial_result);
        let (num, num_bit_vec) = convert_to_usize('z', &initial_result);

        let expected_num = x + y;
        let expected_bit_vec = convert_to_bits(expected_num, num_bit_vec.len());

        // TODO TODO TODO: need to convert the expected num to a bitvec then find the bits that
        // aren't correct between the two and then find the gates that will fix those bits and
        // start flipping
        // honestly that might be kinda dumb because flipping one gate might involve flipping two z
        // values but it at least gives a starting point I guess

        println!("should be x + y = z: {x} + {y} = {}", x + y);
        println!("is actually x + y = z: {x} + {y} = {num}");

        let mut idx_string = String::with_capacity(expected_bit_vec.len() * 2);
        let mut expected_string = String::with_capacity(expected_bit_vec.len() * 2);
        let mut actual_string = String::with_capacity(expected_bit_vec.len() * 2);
        for (idx, (expected_bit, actual_bit)) in
            expected_bit_vec.iter().zip(num_bit_vec.iter()).enumerate()
        {
            if expected_bit != actual_bit {
                println!("the bit at {idx} doesn't match");
            }
            idx_string += format!("{: >3}", idx).as_str();
            expected_string += format!("{: >3}", expected_bit).as_str();
            actual_string += format!("{: >3}", actual_bit).as_str();
        }
        println!("{idx_string}");
        println!("{expected_string}");
        println!("{actual_string}");
    }
}

// copied from day-17
// Claude basically wrote this function
fn convert_to_bits(num: usize, len: usize) -> Vec<u8> {
    // Get number of bits needed to represent the number
    let bits_needed = if num == 0 {
        return vec![0; len];
    } else {
        (usize::BITS - num.leading_zeros()) as usize
    };

    let mut bits = Vec::with_capacity(bits_needed);
    let mut n = num;

    // Extract bits from right to left
    while n > 0 {
        let bit = match n & 1 {
            0 => 0,
            1 => 1,
            _ => panic!("this is impossible"),
        };
        bits.push(bit);
        n >>= 1;
    }

    // If number was 0, push a single 0
    if bits.is_empty() {
        bits.push(0);
    }

    // Include leading zeros up to length
    while bits.len() < len {
        bits.push(1);
    }
    // Reverse to get most significant bits first
    bits.reverse();
    assert!(bits.len() == len);
    bits
}

fn convert_to_usize(leading_char: char, result: &HashMap<String, u8>) -> (usize, VecDeque<u8>) {
    let mut keys = result.keys().collect::<Vec<_>>();
    keys.sort();
    let mut bit_vec = VecDeque::new();
    let mut num = 0;
    for key in keys {
        if key.starts_with(leading_char) {
            let byte = result.get(key).expect("has to exist");
            num +=
                *byte as usize * 2_usize.pow(bit_vec.len().try_into().expect("has to fit in u32"));
            bit_vec.push_front(*result.get(key).expect("has to exist"));
        }
    }
    (num, bit_vec)
}

fn main() {
    Day24::run()
}
