use std::collections::HashMap;

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
        println!("pointer: {pointer}");
        println!("a: {}, b: {}, c: {}", self.a, self.b, self.c);
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

    fn out_string(&self) -> String {
        self.out
            .iter()
            .map(|usize| usize.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn run_program(mut computer: Computer) -> String {
    println!("computer is: {:?}", computer);
    let mut pointer = 0;
    while let Some(next_pointer) = &mut computer.compute(pointer) {
        pointer = *next_pointer;
    }
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
            let (mut a, mut b, _) = run_code_magic(a_reg);
            let mut target_idx = 0;
            let mut so_far = Vec::with_capacity(target.len());
            while target_idx < target.len() && b == target[target_idx] {
                so_far.push(b);
                target_idx += 1;
                (a, b, _) = run_code_magic(a);
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

fn run_code_magic(a_reg: usize) -> (usize, usize, usize) {
    let mut a = a_reg;
    let mut b = a % 8;
    b ^= 1;
    let mut c = a >> b;
    c ^= 6;
    b ^= c;
    b %= 8;
    a >>= 3;
    (a, b, c)
}

impl Puzzle for Day17 {
    fn puzzle_1(contents: String) {
        let computer: Computer = contents.into();
        println!("computer is: {:?}", computer);
        let out = run_program(computer);
        println!("out: {out}");
    }

    fn puzzle_2(contents: String) {
        let computer: Computer = contents.into();
        println!("computer is: {:?}", computer);
        let mut a_reg = 0;
        let mut a_reg_test = 253580150000000;
        let mut b = 0;
        let mut count = 1;
        (a_reg_test, b, _) = run_code_magic(a_reg_test);
        print!("{b}");
        while a_reg_test > 0 {
            print!(",");
            (a_reg_test, b, _) = run_code_magic(a_reg_test);
            print!("{b}");
            count += 1;
        }
        println!("");
        println!("{count}");
        return;

        loop {
            let mut new_computer = computer.clone();
            new_computer.set_regs(a_reg, 0, 0);
            match run_program_with_target(new_computer) {
                Ok(()) => println!("Found! {a_reg}"),
                Err(failed_at) if failed_at > 0 => {
                    println!("#special Not Found :( {failed_at} a_reg: {a_reg}");
                    break;
                }
                Err(failed_at) => println!("Not found :( {failed_at} a_reg: {a_reg}"),
            }
            println!("Not found :( {a_reg}");
            a_reg += 1;
        }
    }
}

fn main() {
    Day17::run();
}
