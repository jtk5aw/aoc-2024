use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use helpers::Puzzle;

struct Day24;

#[derive(Debug)]
struct Wire {
    name: String,
    feeds_into: Vec<Rc<RefCell<Gate>>>,
    fed_from: Vec<Rc<RefCell<Gate>>>,
}

impl Wire {
    fn deep_clone(&self, cloned_gates: &mut HashMap<String, Rc<RefCell<Gate>>>) -> Self {
        Self {
            name: self.name.clone(),
            feeds_into: self
                .feeds_into
                .iter()
                .map(|gate_ref| {
                    let gate = gate_ref.borrow();
                    let new_gate_ref = cloned_gates
                        .entry(gate.output_wire_name.to_string())
                        // .and_modify(|_| println!("cache hit!"))
                        .or_insert_with(|| Rc::new(RefCell::new(gate_ref.borrow().clone())));
                    new_gate_ref.clone()
                })
                .collect::<Vec<_>>(),
            fed_from: self
                .fed_from
                .iter()
                .map(|gate_ref| {
                    let gate = gate_ref.borrow();
                    let new_gate_ref = cloned_gates
                        .entry(gate.output_wire_name.to_string())
                        //.and_modify(|_| println!("cache hit!"))
                        .or_insert_with(|| Rc::new(RefCell::new(gate_ref.borrow().clone())));
                    new_gate_ref.clone()
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl Wire {
    fn trigger(self, value: u8) -> Vec<(String, u8, Gate)> {
        self.feeds_into
            .into_iter()
            .flat_map(|gate_ref| {
                let mut gate = gate_ref.borrow_mut();
                gate.transition(value)
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Gate {
    state: ComputeState,
    kind: GateKind,
    output_wire_name: String,
}

impl Gate {
    fn transition(&mut self, value: u8) -> Option<(String, u8, Gate)> {
        self.state = self.state.clone().transition(value);
        if let ComputeState::Both(first_val, second_val) = self.state {
            let computed_value = self.kind.compute(first_val, second_val);
            return Some((
                self.output_wire_name.to_string(),
                computed_value,
                self.clone(),
            ));
        }
        None
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GateKind {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Circuit {
    wires: HashMap<String, Wire>,
    gates: Vec<Rc<RefCell<Gate>>>,
}

impl Circuit {
    fn swap_gates(&mut self, first_gate: Gate, second_gate: Gate) {
        let mut first_gate_idx = None;
        let mut second_gate_idx = None;
        // for gate in self.gates.iter() {
        //     println!("gate: {:?}", gate);
        // }
        for idx in 0..self.gates.len() {
            let gate = self.gates[idx].borrow();
            assert!(!(*gate == second_gate && *gate == first_gate));
            if *gate == first_gate {
                first_gate_idx = Some(idx);
            }
            if *gate == second_gate {
                second_gate_idx = Some(idx);
            }
        }
        // println!(
        //     "first_gate: {:?}, second_gate: {:?}",
        //     first_gate, second_gate
        // );
        // println!("{:?}, {:?}", first_gate_idx, second_gate_idx);
        match (first_gate_idx, second_gate_idx) {
            (Some(first_idx), Some(second_idx)) => {
                let mut first_to_swap = self.gates[first_idx].borrow_mut();
                let mut second_to_swap = self.gates[second_idx].borrow_mut();

                first_to_swap.output_wire_name = second_gate.output_wire_name.clone();
                second_to_swap.output_wire_name = first_gate.output_wire_name.clone();
            }
            _ => panic!("failed to swap both gates"),
        }
    }
}

impl Clone for Circuit {
    fn clone(&self) -> Self {
        let mut cloned_gates = HashMap::new();
        let wires = self
            .wires
            .iter()
            .map(|(key, val)| (key.clone(), val.deep_clone(&mut cloned_gates)))
            .collect::<HashMap<String, Wire>>();
        let gates = cloned_gates.into_values().collect::<Vec<_>>();
        assert!(gates.len() == self.gates.len());
        Self { wires, gates }
    }
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

fn build_circuit(contents: String) -> (VecDeque<(String, u8)>, Circuit) {
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

    let (wires, gates) = line_iter
        .map(|curr_line| curr_line.split_once(" -> ").expect("has to have \" -> \""))
        .fold(
            (HashMap::new(), Vec::new()),
            |(mut wires, mut gates), (input_def, output_def)| {
                // Input parsing
                let input_split = input_def.split_whitespace().collect::<Vec<_>>();
                if input_split.len() != 3 {
                    panic!("ran wrong");
                }
                let first_wire_name = input_split[0];
                let second_wire_name = input_split[2];
                let kind = GateKind::try_from(input_split[1]).expect("has to be an action");

                // Create the gate
                let new_gate = Gate {
                    state: ComputeState::None,
                    kind,
                    output_wire_name: output_def.to_string(),
                };
                let gate_ref = Rc::new(RefCell::new(new_gate));
                gates.push(gate_ref.clone());
                println!("found gate!");

                // Create the output wire
                let output_wire = Wire {
                    name: output_def.to_string(),
                    feeds_into: Vec::new(),
                    fed_from: vec![gate_ref.clone()],
                };

                if let None = wires.get(output_def) {
                    wires.insert(output_def.to_string(), output_wire);
                }

                // Create the input wires
                wires
                    .entry(first_wire_name.to_string())
                    .and_modify(|wire| wire.feeds_into.push(gate_ref.clone()))
                    .or_insert_with(|| Wire {
                        name: first_wire_name.to_string(),
                        feeds_into: vec![gate_ref.clone()],
                        fed_from: vec![],
                    });
                wires
                    .entry(second_wire_name.to_string())
                    .and_modify(|wire| wire.feeds_into.push(gate_ref.clone()))
                    .or_insert_with(|| Wire {
                        name: second_wire_name.to_string(),
                        feeds_into: vec![gate_ref.clone()],
                        fed_from: vec![],
                    });

                (wires, gates)
            },
        );

    let circuit = Circuit { wires, gates };

    (starting_values, circuit)
}

struct RunResult {
    values: HashMap<String, u8>,
    paths: HashMap<String, Path>,
}

#[derive(Debug)]
enum Path {
    None,
    One(Vec<Gate>),
    Both(Vec<Gate>, Vec<Gate>),
}

impl Path {
    fn add(&mut self, new_path: Vec<Gate>) {
        *self = match self {
            Path::None => Self::One(new_path),
            Path::One(first_path) => Self::Both(first_path.clone(), new_path),
            Path::Both(_, _) => panic!("should never be more than two paths"),
        }
    }

    fn set_of_gates(&self) -> HashSet<Gate> {
        match self {
            Path::One(vec) => HashSet::from_iter(vec.clone().into_iter()),
            _ => panic!("shouldn't call this method for gates in this state (all output wires should only be reached once)"),
        }
    }
}

fn run_circuit(starting_values: VecDeque<(String, u8)>, circuit: Circuit) -> RunResult {
    let mut wires = circuit.wires;
    let mut to_process = starting_values
        .into_iter()
        .map(|(name, starting_byte)| (name, starting_byte, Vec::new()))
        .collect::<VecDeque<_>>();
    let mut values = HashMap::new();
    let mut paths = HashMap::new();
    while let Some((name, byte, path)) = to_process.pop_front() {
        values.insert(name.to_string(), byte);
        let wire = wires
            .remove(&name)
            .expect("should have been able to pull the wire");
        wire.trigger(byte)
            .into_iter()
            .for_each(|(new_wire_name, new_val, gate)| {
                let mut new_path = path.clone();
                new_path.push(gate);
                let paths_entry = paths
                    .entry(new_wire_name.to_string())
                    .or_insert_with(|| Path::None);
                paths_entry.add(new_path.clone());
                to_process.push_back((new_wire_name, new_val, new_path))
            });
    }
    RunResult { values, paths }
}

impl Puzzle for Day24 {
    fn puzzle_1(contents: String) {
        let (starting_values, final_circuit) = build_circuit(contents);

        let run_result = run_circuit(starting_values, final_circuit);
        let result_values = run_result.values;

        let mut keys = result_values.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys.iter() {
            println!(
                "key:  {}, value: {}",
                key,
                result_values.get(*key).expect("has to exist")
            );
        }

        let (x, _) = convert_to_usize('x', &result_values);
        let (y, _) = convert_to_usize('y', &result_values);
        let (num, _) = convert_to_usize('z', &result_values);
        println!("x + y = z: {x} + {y} = {num}");
    }

    fn puzzle_2(contents: String) {
        let (starting_points, initial_circuit) = build_circuit(contents);

        let initial_run_result = run_circuit(starting_points.clone(), initial_circuit.clone());
        let initial_result_values = initial_run_result.values;
        let (x, _) = convert_to_usize('x', &initial_result_values);
        let (y, _) = convert_to_usize('y', &initial_result_values);
        let (num, num_bit_vec) = convert_to_usize('z', &initial_result_values);

        let mut keys = initial_result_values.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys.iter() {
            println!(
                "key:  {}, value: {}",
                key,
                initial_result_values.get(*key).expect("has to exist")
            );
        }

        let expected_num = x + y;
        println!("expected_num: {expected_num}, num: {num}");
        let expected_bit_vec = convert_to_bits(expected_num, num_bit_vec.len());

        println!("should be x + y = z: {x} + {y} = {}", x + y);
        println!("is actually x + y = z: {x} + {y} = {num}");

        let (idx_string, expected_string, actual_string, wrong_output_wires) =
            diff_bit_vecs(expected_bit_vec.clone(), num_bit_vec.into());
        println!("{idx_string}");
        println!("{expected_string}");
        println!("{actual_string}");

        let mut sets_to_chose_from = Vec::new();
        let mut total_num_gates = 0;
        for wire_name in wrong_output_wires.clone() {
            let paths = initial_run_result
                .paths
                .get(&wire_name)
                .expect("has to exist");
            println!("paths: {:?}", paths);
            let set_of_gates = paths.set_of_gates();
            total_num_gates += set_of_gates.len();
            sets_to_chose_from.push(set_of_gates);
        }

        println!(
            "sets_to_chose_from: {}, total_num_gates: {total_num_gates}, ",
            sets_to_chose_from.len(),
        );

        let potential_swaps = generate_potential_swaps(sets_to_chose_from);
        println!("{:?}", potential_swaps);
        println!("found {} potential_swaps", potential_swaps.len());

        let mut min_wrong = wrong_output_wires.len();
        println!("min wrong: {min_wrong}");
        // TODO TODO TODO: Right now I'm only doing one swap at a time I need to do 4 at a time
        // but this mega for loop just won't work (duh)
        // So what needs to be done is trim the set of potential swaps.
        // a Swap could only possibly solve the problem if there is a change in every single
        // potential path. There are 12 wrong paths to start. That means that some of the swaps
        // HAVE to have an effect on multiple output bits. Need to only consider swaps that meet
        // these criteria
        let mut idx = 0;
        for first_swap_idx in 0..potential_swaps.len() {
            let first_swap = potential_swaps[first_swap_idx].clone();
            for second_swap_idx in first_swap_idx..potential_swaps.len() {
                let second_swap = potential_swaps[first_swap_idx].clone();
                for third_swap_idx in second_swap_idx..potential_swaps.len() {
                    let third_swap = potential_swaps[first_swap_idx].clone();
                    for fourth_swap_idx in third_swap_idx..potential_swaps.len() {
                        panic!("testing other stuff");
                        let fourth_swap = potential_swaps[first_swap_idx].clone();
                        println!("attempt {idx}");
                        let mut new_circuit = initial_circuit.clone();
                        new_circuit.swap_gates(first_swap.0.clone(), first_swap.1.clone());
                        new_circuit.swap_gates(second_swap.0.clone(), second_swap.1.clone());
                        new_circuit.swap_gates(third_swap.0.clone(), third_swap.1.clone());
                        new_circuit.swap_gates(fourth_swap.0.clone(), fourth_swap.1.clone());
                        let run_result = run_circuit(starting_points.clone(), new_circuit);
                        let (new_num, new_bit_vec) = convert_to_usize('z', &run_result.values);
                        println!("produced: {new_num}");
                        let (_, _, _, wrong_output_wires) =
                            diff_bit_vecs(expected_bit_vec.clone(), new_bit_vec.into());
                        if wrong_output_wires.len() < min_wrong {
                            println!("found new min!");
                            println!("num_wrong: {}", wrong_output_wires.len());
                            min_wrong = wrong_output_wires.len()
                        }

                        if new_num == expected_num {
                            println!("found!!!!!");
                            break;
                        }
                        idx += 1;
                    }
                }
            }
        }
    }
}

fn generate_potential_swaps(sets_to_chose_from: Vec<HashSet<Gate>>) -> Vec<(Gate, Gate)> {
    let mut result = Vec::new();
    for idx_1 in 0..sets_to_chose_from.len() {
        for idx_2 in idx_1..sets_to_chose_from.len() {
            let set_1 = sets_to_chose_from[idx_1].clone();
            let set_2 = sets_to_chose_from[idx_2].clone();

            let intersection = set_1.intersection(&set_2).collect::<HashSet<_>>();

            for set_1_val in set_1.iter() {
                for set_2_val in set_2.iter() {
                    if !intersection.contains(&set_1_val) && !intersection.contains(&set_2_val) {
                        let mut new_set_1_val = set_1_val.clone();
                        new_set_1_val.state = ComputeState::None;
                        let mut new_set_2_val = set_2_val.clone();
                        new_set_2_val.state = ComputeState::None;
                        result.push((new_set_1_val, new_set_2_val));
                    }
                }
            }
        }
    }
    result
}

fn diff_bit_vecs(
    expected_bit_vec: Vec<u8>,
    actual_bit_vec: Vec<u8>,
) -> (String, String, String, Vec<String>) {
    let mut idx_string = String::with_capacity(expected_bit_vec.len() * 2);
    let mut expected_string = String::with_capacity(expected_bit_vec.len() * 2);
    let mut actual_string = String::with_capacity(expected_bit_vec.len() * 2);
    let mut wrong_output_wires = Vec::new();
    for (idx, (expected_bit, actual_bit)) in expected_bit_vec
        .iter()
        .zip(actual_bit_vec.iter())
        .enumerate()
    {
        if expected_bit != actual_bit {
            let wrong_wire_name = format!("z{:0>2}", actual_bit_vec.len() - idx);
            //println!("the bit at {idx} ({wrong_wire_name}) doesn't match",);
            wrong_output_wires.push(wrong_wire_name);
        }
        idx_string += format!("{: >2}", idx).as_str();
        expected_string += format!("{: >2}", expected_bit).as_str();
        actual_string += format!("{: >2}", actual_bit).as_str();
    }
    (
        idx_string,
        expected_string,
        actual_string,
        wrong_output_wires,
    )
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
    println!("bits.len(): {}, len: {len}", bits.len());
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
