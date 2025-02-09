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
    fed_from: Option<Rc<RefCell<Gate>>>,
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
                        .or_insert_with(|| Rc::new(RefCell::new(gate_ref.borrow().clone())));
                    new_gate_ref.clone()
                })
                .collect::<Vec<_>>(),
            fed_from: self.fed_from.as_ref().map(|curr_ref| {
                let borrow = curr_ref.borrow();
                let entry = cloned_gates
                    .entry(borrow.output_wire_name.clone())
                    .or_insert_with(|| Rc::new(RefCell::new(borrow.clone())));
                entry.clone()
            }),
        }
    }
}

impl Wire {
    fn trigger(&self, value: u8) -> Vec<(String, u8, Gate)> {
        self.feeds_into
            .iter()
            .flat_map(|gate_ref| {
                let mut gate = gate_ref.borrow_mut();
                gate.transition(self.name.clone(), value)
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
    fn transition(&mut self, wire_name: String, value: u8) -> Option<(String, u8, Gate)> {
        self.state = self.state.clone().transition(wire_name, value);
        if let ComputeState::Both(first_val, second_val) = &self.state {
            let computed_value = self.kind.compute(first_val.value, second_val.value);
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
    One(ComputeValue),
    Both(ComputeValue, ComputeValue),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct ComputeValue {
    wire_name: String,
    value: u8,
}

impl ComputeState {
    fn transition(self, wire_name: String, value: u8) -> Self {
        match self {
            ComputeState::None => Self::One(ComputeValue { wire_name, value }),
            ComputeState::One(old_value) => {
                Self::Both(old_value, ComputeValue { wire_name, value })
            }
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
        // Get idx's to swap for the regular gate
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
        println!(
            "first_gate: {:?}, second_gate: {:?}",
            first_gate, second_gate
        );
        println!("{:?}, {:?}", first_gate_idx, second_gate_idx);
        match (first_gate_idx, second_gate_idx) {
            (Some(first_idx), Some(second_idx)) => {
                let first_gate_ref = self.gates[first_idx].clone();
                let second_gate_ref = self.gates[second_idx].clone();

                let mut first_to_swap = first_gate_ref.borrow_mut();
                let mut second_to_swap = second_gate_ref.borrow_mut();

                first_to_swap.output_wire_name = second_gate.output_wire_name.clone();
                second_to_swap.output_wire_name = first_gate.output_wire_name.clone();

                self.wires
                    .entry(first_to_swap.output_wire_name.to_string())
                    .and_modify(|wire| wire.fed_from = Some(first_gate_ref.clone()));
                self.wires
                    .entry(second_to_swap.output_wire_name.to_string())
                    .and_modify(|wire| wire.fed_from = Some(second_gate_ref.clone()));
            }
            _ => panic!("failed to swap both gates"),
        };
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

                // Create the output wire
                wires
                    .entry(output_def.to_string())
                    .and_modify(|wire: &mut Wire| wire.fed_from = Some(gate_ref.clone()))
                    .or_insert_with(|| Wire {
                        name: output_def.to_string(),
                        feeds_into: Vec::new(),
                        fed_from: Some(gate_ref.clone()),
                    });

                // Create the input wires
                wires
                    .entry(first_wire_name.to_string())
                    .and_modify(|wire| wire.feeds_into.push(gate_ref.clone()))
                    .or_insert_with(|| Wire {
                        name: first_wire_name.to_string(),
                        feeds_into: vec![gate_ref.clone()],
                        fed_from: None,
                    });
                wires
                    .entry(second_wire_name.to_string())
                    .and_modify(|wire| wire.feeds_into.push(gate_ref.clone()))
                    .or_insert_with(|| Wire {
                        name: second_wire_name.to_string(),
                        feeds_into: vec![gate_ref.clone()],
                        fed_from: None,
                    });

                (wires, gates)
            },
        );

    let circuit = Circuit { wires, gates };

    (starting_values, circuit)
}

struct RunResult {
    values: HashMap<String, u8>,
}

fn run_circuit(starting_values: VecDeque<(String, u8)>, circuit: &Circuit) -> RunResult {
    let wires = &circuit.wires;
    let mut to_process = starting_values;
    let mut values = HashMap::new();
    while let Some((name, byte)) = to_process.pop_front() {
        values.insert(name.to_string(), byte);
        let wire = wires
            .get(&name)
            .expect("should have been able to pull the wire");
        wire.trigger(byte)
            .into_iter()
            .for_each(|(new_wire_name, new_val, _gate)| {
                to_process.push_back((new_wire_name, new_val))
            });
    }
    RunResult { values }
}

impl Puzzle for Day24 {
    fn puzzle_1(contents: String) {
        let (starting_values, final_circuit) = build_circuit(contents);

        let run_result = run_circuit(starting_values, &final_circuit);
        let result_values = run_result.values;

        let (x, _) = convert_to_usize('x', &result_values);
        let (y, _) = convert_to_usize('y', &result_values);
        let (num, _) = convert_to_usize('z', &result_values);
        println!("x + y = z: {x} + {y} = {num}");
    }

    fn puzzle_2(contents: String) {
        let (starting_points, initial_circuit) = build_circuit(contents);

        let mut first_circuit = initial_circuit.clone();
        // Most of these swaps were found by using the get_circuit_paths function on the output
        // bits that were incorrect. Would go through one by one and find where the structure
        // differed from a single bit adder (this wiki page: https://en.wikipedia.org/wiki/Adder_(electronics)
        // in the full-adder section)
        // TL;DR the program won't find the solution to any input. This only works for MY input
        let swaps = vec![
            (
                (GateKind::Xor, "rpv".to_string()),
                (GateKind::And, "z11".to_string()),
            ),
            (
                (GateKind::Xor, "rpb".to_string()),
                (GateKind::And, "ctg".to_string()),
            ),
            (
                (GateKind::Xor, "dmh".to_string()),
                (GateKind::And, "z31".to_string()),
            ),
            (
                (GateKind::Xor, "dvq".to_string()),
                (GateKind::Or, "z38".to_string()),
            ),
        ];
        swaps.iter().for_each(|(first_params, second_params)| {
            first_circuit.swap_gates(
                Gate {
                    state: ComputeState::None,
                    kind: first_params.0.clone(),
                    output_wire_name: first_params.1.clone(),
                },
                Gate {
                    state: ComputeState::None,
                    kind: second_params.0.clone(),
                    output_wire_name: second_params.1.clone(),
                },
            )
        });

        let circuit_with_swaps = first_circuit.clone();

        let first_run_result = run_circuit(starting_points.clone(), &first_circuit);
        let initial_result_values = first_run_result.values;
        let (x, x_bit_vec) = convert_to_usize('x', &initial_result_values);
        let (y, _) = convert_to_usize('y', &initial_result_values);
        let (num, num_bit_vec) = convert_to_usize('z', &initial_result_values);

        let expected_num = x + y;
        println!("expected_num: {expected_num}, num: {num}");
        let expected_bit_vec = convert_to_bits(expected_num, num_bit_vec.len());

        println!("should be x + y = z: {x} + {y} = {}", x + y);
        println!("is actually x + y = z: {x} + {y} = {num}");

        let (idx_string, expected_string, actual_string, _wrong_output_wires) =
            diff_bit_vecs(expected_bit_vec.clone(), num_bit_vec.into());
        println!("{idx_string}");
        println!("{expected_string}");
        println!("{actual_string}");

        let circuit_paths = get_circuit_paths(
            vec!["z03".to_string()],
            //wrong_output_wires[3..4].to_vec(),
            &first_circuit,
        );
        let mut iter = circuit_paths.into_iter().enumerate();

        while let Some((idx, current_level)) = iter.next() {
            println!("===== LEVEL {idx} ======");
            for wire_gates in current_level {
                println!("== NEW WIRE ==");
                for gate in wire_gates {
                    print_gate(&gate.borrow());
                }
            }
        }

        let mut result = swaps
            .into_iter()
            .flat_map(|((_, name_1), (_, name_2))| vec![name_1, name_2])
            .collect::<Vec<_>>();
        result.sort();
        let final_str = result.join(",");
        println!("the answer is {final_str}");

        let bit_vec_len = x_bit_vec.len();

        let mut new_x = 1;
        let mut new_y = 1;
        while new_x < x {
            let x_starting_points = convert_num_to_starting_points('x', new_x, bit_vec_len);
            while new_y < y {
                let y_starting_points = convert_num_to_starting_points('y', new_y, bit_vec_len);
                compare(
                    circuit_with_swaps.clone(),
                    new_x,
                    new_y,
                    x_starting_points.clone(),
                    y_starting_points,
                );
                new_y <<= 1;
            }
            new_y = 1;
            new_x <<= 1;
        }

        // This takes way way way too long
        // for x_num in 0..=x {
        //     let x_starting_points = convert_num_to_starting_points('x', x_num, bit_vec_len);
        //     for y_num in 0..=y {
        //         let y_starting_points = convert_num_to_starting_points('y', y_num, bit_vec_len);
        //         compare(
        //             circuit_with_swaps.clone(),
        //             x_num,
        //             y_num,
        //             x_starting_points.clone(),
        //             y_starting_points,
        //         );
        //     }
        // }
    }
}

fn convert_num_to_starting_points(leading_char: char, num: usize, len: usize) -> Vec<(String, u8)> {
    let bit_vec = convert_to_bits(num, len);
    //println!("{:?}", x_bit_vec);
    bit_vec
        .into_iter()
        .enumerate()
        .map(|(idx, bit_val)| (format!("{leading_char}{:0>2}", len - idx - 1), bit_val))
        .collect::<Vec<_>>()
}

fn compare(
    circuit: Circuit,
    x_num: usize,
    y_num: usize,
    x_starting_points: Vec<(String, u8)>,
    y_starting_points: Vec<(String, u8)>,
) {
    let mut current_starting_points =
        VecDeque::with_capacity(x_starting_points.len() + y_starting_points.len());
    current_starting_points.extend(x_starting_points.clone());
    current_starting_points.extend(y_starting_points.clone());

    let run_result = run_circuit(current_starting_points, &circuit);

    let expected_num = x_num + y_num;
    let (x, x_num_bit_vec) = convert_to_usize('x', &run_result.values);
    let (y, y_num_bit_vec) = convert_to_usize('y', &run_result.values);
    assert!(x == x_num);
    assert!(y == y_num);
    let (num, num_bit_vec) = convert_to_usize('z', &run_result.values);
    let expected_bit_vec = convert_to_bits(expected_num, num_bit_vec.len());
    println!("(actual) {num} == (expected) {expected_num}");
    if expected_num != num {
        println!("x : expected {x_num} actual {x}");
        println!("{:?}", x_num_bit_vec);
        println!("y : expected {y_num} actual {y}");
        println!("{:?}", y_num_bit_vec);
        let (idx_string, expected_string, actual_string, wrong_output_wires) =
            diff_bit_vecs(expected_bit_vec.clone(), num_bit_vec.into());
        println!("{idx_string}");
        println!("{expected_string}");
        println!("{actual_string}");

        // Added the z38 here because it was helpful for debugging this final miss
        let mut to_check = wrong_output_wires[1..2].to_vec();
        to_check.push("z38".to_string());
        let circuit_paths = get_circuit_paths(to_check, &circuit);
        let mut iter = circuit_paths.into_iter().enumerate();

        while let Some((idx, current_level)) = iter.next() {
            // Generally if there's been an issue it has to be lower than this
            if idx == 6 {
                break;
            }
            println!("===== LEVEL {idx} ======");
            for wire_gates in current_level {
                println!("== NEW WIRE ==");
                for gate in wire_gates {
                    print_gate(&gate.borrow());
                }
            }
        }

        println!("mismatch!");
        panic!();
    } else {
        println!("fine!");
    }
}

fn get_circuit_paths(wrong_output_wires: Vec<String>, circuit: &Circuit) -> CircuitPaths {
    let mut bad_gates = HashSet::new();

    let paths = wrong_output_wires
        .into_iter()
        .map(|root_wire| {
            let mut result = Vec::new();
            let first_level = circuit.wires.get(&root_wire).unwrap();
            result.push(vec![first_level.fed_from.as_ref().unwrap().clone()]);
            loop {
                let previous_level = result.last().expect("has to have a last");
                let next_level = previous_level
                    .iter()
                    .flat_map(|gate_ref| {
                        let gate = gate_ref.borrow();
                        match &gate.state {
                            ComputeState::Both(compute_value, compute_value1) => {
                                let first_wire =
                                    circuit.wires.get(&compute_value.wire_name).unwrap();
                                let second_wire =
                                    circuit.wires.get(&compute_value1.wire_name).unwrap();
                                match (first_wire.fed_from.as_ref(), second_wire.fed_from.as_ref())
                                {
                                    (Some(first_gate_ref), Some(second_gate_ref)) => {
                                        let first_gate = first_gate_ref.borrow();
                                        let second_gate = second_gate_ref.borrow();
                                        if bad_transition(&first_gate, &gate) {
                                            bad_gates.insert((first_gate.clone(), gate.clone()));
                                        }
                                        if bad_transition(&second_gate, &gate) {
                                            bad_gates.insert((second_gate.clone(), gate.clone()));
                                        }
                                        Some(vec![first_gate_ref.clone(), second_gate_ref.clone()])
                                    }
                                    (None, None) => {
                                        if matches!(gate.kind, GateKind::Or) {
                                            panic!("This is just a placeholder");
                                        }
                                        None
                                    }
                                    _ => panic!("This shouldn't happen"),
                                }
                            }
                            _ => panic!("This shouldn't happen"),
                        }
                    })
                    .collect::<Vec<_>>();
                if next_level.is_empty() {
                    break;
                }
                let flattened = next_level.into_iter().flatten().collect::<Vec<_>>();
                result.push(flattened);
            }
            result
        })
        .collect::<Vec<_>>();

    println!("BAD_GATES: len({})", bad_gates.len());
    for gate in bad_gates {
        println!(
            "start: {:?} {}, end: {:?} {}",
            gate.0.kind, gate.0.output_wire_name, gate.1.kind, gate.1.output_wire_name
        );
        println!("{:?}", gate);
    }

    CircuitPaths::new(paths)
}

fn bad_transition(start_gate: &Gate, end_gate: &Gate) -> bool {
    match (&start_gate.kind, &end_gate.kind) {
        (GateKind::And, GateKind::Xor) => true,
        (GateKind::Or, GateKind::Or) => true,
        (GateKind::Xor, GateKind::Or) => true,
        _ => false,
    }
}

struct CircuitPaths {
    paths: Vec<Vec<Vec<Rc<RefCell<Gate>>>>>,
}

impl CircuitPaths {
    fn new(paths: Vec<Vec<Vec<Rc<RefCell<Gate>>>>>) -> Self {
        Self { paths }
    }
}

struct CircuitPathsIterator<'a> {
    idx: usize,
    paths: &'a Vec<Vec<Vec<Rc<RefCell<Gate>>>>>,
}

impl<'a> IntoIterator for &'a CircuitPaths {
    type Item = Vec<&'a Vec<Rc<RefCell<Gate>>>>;
    type IntoIter = CircuitPathsIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CircuitPathsIterator {
            idx: 0,
            paths: &self.paths,
        }
    }
}

impl<'a> Iterator for CircuitPathsIterator<'a> {
    type Item = Vec<&'a Vec<Rc<RefCell<Gate>>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining_with_value = self
            .paths
            .iter()
            .flat_map(|wire_path| wire_path.get(self.idx))
            .collect::<Vec<_>>();

        self.idx += 1;

        if remaining_with_value.is_empty() {
            None
        } else {
            Some(remaining_with_value)
        }
    }
}

fn print_gate(gate: &Gate) {
    match &gate.state {
        ComputeState::Both(compute_value, compute_value1) => println!(
            "{} ({}) {:?} {} ({}) -> {}",
            compute_value.value,
            compute_value.wire_name,
            gate.kind,
            compute_value1.value,
            compute_value1.wire_name,
            gate.output_wire_name
        ),
        _ => panic!("has to be in a both state"),
    }
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
            println!("the bit at {idx} ({wrong_wire_name}) doesn't match",);
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
        bits.push(0);
    }
    // Reverse to get most significant bits first
    bits.reverse();
    //println!("bits.len(): {}, len: {len}", bits.len());
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
