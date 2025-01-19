use std::{
    collections::{HashMap, HashSet},
    env::current_dir,
    ops::Deref,
};

use helpers::Puzzle;

struct Day17;

#[derive(Debug, Clone)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    code: Vec<Code>,
    out: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Code {
    literal: usize,
    combo: ComboOperand,
    instruction: Instruction,
}

#[derive(Debug, Clone)]
enum ComboOperand {
    Literal(usize),
    A,
    B,
    C,
    Reserved,
}

#[derive(Debug, Clone)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<String> for Code {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        let value_as_usize = value
            .parse()
            .map_err(|err| format!("Couldn't parse as usize: {:?}", err))?;
        let literal = if value_as_usize > 7 {
            Err("Value is greater than 7. Can't be literal".to_string())
        } else {
            Ok(value_as_usize)
        }?;
        let combo = match value_as_usize {
            0..=3 => ComboOperand::Literal(value_as_usize),
            4 => ComboOperand::A,
            5 => ComboOperand::B,
            6 => ComboOperand::C,
            7 => ComboOperand::Reserved,
            _ => panic!("Value shouldn't be this"),
        };
        let instruction = match value_as_usize {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("instruction is invalid"),
        };

        Ok(Self {
            literal,
            combo,
            instruction,
        })
    }
}

fn parse_register(register: String, value: Option<&str>) -> usize {
    value
        .expect("Has to have line")
        .strip_prefix(format!("Register {register}: ").as_str())
        .expect("has to have this prefix")
        .parse()
        .expect("has to be a number")
}

impl From<String> for Computer {
    fn from(contents: String) -> Self {
        let mut lines = contents.lines();
        let a = parse_register("A".to_string(), lines.next());
        let b = parse_register("B".to_string(), lines.next());
        let c = parse_register("C".to_string(), lines.next());
        // skip the empty line
        let _empty = lines.next();
        let code = lines
            .next()
            .expect("has to have one final line")
            .strip_prefix("Program: ")
            .expect("has to have the program prefix")
            .split(",")
            .map(|str| str.to_string().try_into().expect("has to be convertable"))
            .collect::<Vec<_>>();
        Self {
            a,
            b,
            c,
            code,
            out: Vec::new(),
        }
    }
}

impl Computer {
    fn set_regs(&mut self, a: usize, b: usize, c: usize) {
        self.a = a;
        self.b = b;
        self.c = c;
    }

    fn get_combo_value(&self, combo: &ComboOperand) -> usize {
        match combo {
            ComboOperand::Literal(value) => *value,
            ComboOperand::A => self.a,
            ComboOperand::B => self.b,
            ComboOperand::C => self.c,
            ComboOperand::Reserved => panic!("tried to use reserved combo operand"),
        }
    }

    fn compute(&mut self, pointer: usize) -> Option<usize> {
        let current_code = self.code.get(pointer)?;
        let operand = self
            .code
            .get(pointer + 1)
            .expect("if we had a current_code operand should exist too");
        //println!("a: {}, b: {}, c: {}", self.a, self.b, self.c);
        //println!("Performing: {:?} with operand {:?}", current_code, operand);
        match current_code.instruction {
            Instruction::Jnz if self.a != 0 => return Some(operand.literal),
            Instruction::Jnz => {}
            Instruction::Adv => {
                self.a /= 2_usize.pow(
                    self.get_combo_value(&operand.combo)
                        .try_into()
                        .expect("has to fit in u32"),
                )
            }
            Instruction::Bdv => {
                self.b = self.a
                    / 2_usize.pow(
                        self.get_combo_value(&operand.combo)
                            .try_into()
                            .expect("has to fit in u32"),
                    )
            }
            Instruction::Cdv => {
                self.c = self.a
                    / 2_usize.pow(
                        self.get_combo_value(&operand.combo)
                            .try_into()
                            .expect("has to fit in u32"),
                    )
            }
            Instruction::Bxl => self.b ^= operand.literal,
            Instruction::Bst => self.b = self.get_combo_value(&operand.combo) % 8,
            Instruction::Bxc => self.b ^= self.c,
            Instruction::Out => self.out.push(self.get_combo_value(&operand.combo) % 8),
        }

        Some(pointer + 2)
    }

    fn run_program(&mut self) {
        let mut pointer = 0;
        while let Some(next_pointer) = &mut self.compute(pointer) {
            pointer = *next_pointer;
        }
    }

    fn out_string(&self) -> String {
        self.out
            .iter()
            .map(|usize| usize.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn get_program_out_string(mut computer: Computer) -> String {
    //println!("{:?}", computer);
    &mut computer.run_program();
    computer.out_string()
}

fn run_program_with_target(computer: Computer) -> Result<(), usize> {
    let target = computer
        .code
        .iter()
        .map(|code| code.literal)
        .collect::<Vec<_>>();

    let mut a_reg = 1_000_000_000_000_000_000;
    loop {
        let mut test_a_reg = a_reg;
        for _ in 0..8 {
            test_a_reg >>= 3;
        }
        if test_a_reg != 0 {
            let (mut a, mut b, _) = run_code_magic(a_reg, false);
            let mut target_idx = 0;
            let mut so_far = Vec::with_capacity(target.len());
            while target_idx < target.len() && b == target[target_idx] {
                so_far.push(b);
                target_idx += 1;
                (a, b, _) = run_code_magic(a, false);
            }

            if target_idx == target.len() - 1 {
                println!("Found! {a_reg}");
                break;
            }
            if target_idx > 6 {
                println!("target is: {:?}", target);
                println!("#special {a_reg} {target_idx}");
                println!("so_far: {:?}", so_far);
            }
            //println!("Not found :( {a_reg}. Made it to {target_idx}");
        } else {
            println!("skipped {a_reg}");
        }

        a_reg += 1;
    }

    Ok(())
}

fn run_code_magic(a_reg: usize, debug: bool) -> (usize, usize, usize) {
    let mut a = a_reg;
    let mut b = a % 8;
    if debug {
        println!("a: {a}");
        println!("b.1: {b}");
    }
    b ^= 1;
    if debug {
        println!("b.2: {b}");
    }
    let mut c = a >> b;
    if debug {
        println!("c.0: {c}");
    }
    c ^= 6;
    if debug {
        println!("c.1: {c}");
    }
    b ^= c;
    if debug {
        println!("b.3: {b}");
    }
    b %= 8;
    if debug {
        println!("b: {b}");
    }
    a >>= 3;
    (a, b, c)
}

fn puzzle_2_test(computer: Computer) {
    for i in 0..9 {
        let mut a_reg_vec = vec![0; 14];
        let last = a_reg_vec.len() - 1;
        a_reg_vec[0] = 4;
        a_reg_vec[last] = 5;
        let mut a_reg = a_reg_vec
            .into_iter()
            .map(|val| val.to_string())
            .collect::<String>()
            .parse()
            .expect("has to be usize");
        a_reg += 8 * i;

        let mut test_computer = computer.clone();
        test_computer.a = a_reg;
        let computer_result = get_program_out_string(test_computer);
        //println!("computer_result: {computer_result}");

        let mut b = 0;
        let mut count = 1;
        (a_reg, b, _) = run_code_magic(a_reg, false);
        let mut result = Vec::new();
        result.push(b);
        while a_reg > 0 {
            (a_reg, b, _) = run_code_magic(a_reg, false);
            result.push(b);
            count += 1;
        }
        let result_val = result
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(",");
        println!("len: {}", result.len());
        println!("{result_val}");
    }
    return;
}

fn convert_to_bits(num: usize, len: usize) -> Vec<Bit> {
    // Get number of bits needed to represent the number
    let bits_needed = if num == 0 {
        return vec![Bit::Zero; len];
    } else {
        (usize::BITS - num.leading_zeros()) as usize
    };

    let mut bits = Vec::with_capacity(bits_needed);
    let mut n = num;

    // Extract bits from right to left
    while n > 0 {
        let bit = match n & 1 {
            0 => Bit::Zero,
            1 => Bit::One,
            _ => panic!("this is impossible"),
        };
        bits.push(bit);
        n >>= 1;
    }

    // If number was 0, push a single 0
    if bits.is_empty() {
        bits.push(Bit::Zero);
    }

    // Include leading zeros up to length
    while bits.len() < len {
        bits.push(Bit::Zero);
        println!("pushing: len(bits): {} len: {}", bits.len(), len);
    }
    // Reverse to get most significant bits first
    bits.reverse();
    assert!(bits.len() == len);
    bits
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Bit {
    One,
    Zero,
}

impl Deref for Bit {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        match self {
            Bit::One => &1,
            Bit::Zero => &0,
        }
    }
}

struct PrefixLockBitVec {
    current_new_val: usize,
    prefix_bits: Vec<Bit>,
    playground_bits: Vec<Bit>,
}

fn bitvec_to_usize(bits: &Vec<Bit>) -> usize {
    let mut result: usize = 0;

    for bit in bits.iter() {
        result = (result << 1) | **bit;
    }

    result
}

impl Iterator for PrefixLockBitVec {
    type Item = (usize, Vec<Bit>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_new_val > 7 {
            return None;
        }

        println!("current_new_val: {}", self.current_new_val);
        let suffix_bits = convert_to_bits(self.current_new_val, 3);
        self.playground_bits.truncate(self.prefix_bits.len());
        for bit in suffix_bits.iter() {
            self.playground_bits.push(bit.clone());
        }
        assert!(self.playground_bits.len() == self.prefix_bits.len() + 3);
        let current = bitvec_to_usize(&self.playground_bits);

        self.current_new_val += 1;

        Some((current, suffix_bits))
    }
}

impl PrefixLockBitVec {
    fn new(prefix_bits: Vec<Bit>) -> Self {
        let mut playground_bits = Vec::with_capacity(prefix_bits.len());
        playground_bits.clone_from(&prefix_bits);
        Self {
            current_new_val: 0,
            playground_bits,
            prefix_bits,
        }
    }
}

impl Puzzle for Day17 {
    fn puzzle_1(contents: String) {
        let computer: Computer = contents.into();
        println!("computer is: {:?}", computer);
        let out = get_program_out_string(computer);
        println!("out: {out}");
    }

    fn puzzle_2(contents: String) {
        let computer: Computer = contents.into();
        let target = computer
            .code
            .iter()
            .map(|code| code.literal)
            .rev()
            .collect::<Vec<_>>();

        let mut matched_target = false;
        let mut current_bits = Vec::with_capacity(46);
        let mut matched_previously = 0;
        let mut skip_map: HashMap<usize, HashSet<Vec<Bit>>> = HashMap::new();

        while !matched_target {
            println!("current_bits: {:?}", current_bits);
            let iter = PrefixLockBitVec::new(current_bits.clone());
            match find_match(
                matched_previously,
                &target,
                &skip_map,
                iter,
                computer.clone(),
            ) {
                Some((output, mut bits)) => {
                    matched_previously += 1;
                    matched_target = output.len() == target.len();
                    current_bits.append(&mut bits);
                }
                None => {
                    println!(
                        "Couldn't find any in current state, going back one level and tryin again"
                    );
                    let to_skip = current_bits
                        .drain(current_bits.len() - 3..)
                        .collect::<Vec<_>>();
                    assert!(to_skip.len() == 3);
                    // Clear the current level since we're going back up and will start from a
                    // different place
                    skip_map
                        .entry(matched_previously)
                        .and_modify(|set| set.clear());
                    matched_previously -= 1;
                    // Add to the level above
                    skip_map
                        .entry(matched_previously)
                        .or_insert_with(HashSet::new)
                        .insert(to_skip);
                }
            }
        }
    }
}

fn find_match(
    matched_previously: usize,
    target: &Vec<usize>,
    skip: &HashMap<usize, HashSet<Vec<Bit>>>,
    mut iter: PrefixLockBitVec,
    computer: Computer,
) -> Option<(Vec<usize>, Vec<Bit>)> {
    let mut skipped = false;
    while let Some((val, bits)) = iter.next() {
        if let Some(set) = skip.get(&matched_previously) {
            if set.contains(&bits) {
                println!("skipping!");
                println!("matched_previously: {matched_previously}");
                skipped = true;
                continue;
            }
        }
        let mut test_computer = computer.clone();
        println!("new a is: {val} with bits {:?}", bits);
        //println!("right shifted a is: {}", val >> 3);
        test_computer.a = val;
        test_computer.run_program();
        let output = test_computer
            .out
            .clone()
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        println!("output is: {:?}", test_computer.out);
        println!("reversed output is: {:?}", output);

        if output.len() == matched_previously + 1 {
            let mut all_match = true;
            for zip in output.iter().zip(target.iter()) {
                if *zip.0 != *zip.1 {
                    all_match = false;
                }
            }
            if all_match {
                println!("matched!");
                return Some((output, bits));
            }
        }
    }
    None
}

fn main() {
    Day17::run();
}
