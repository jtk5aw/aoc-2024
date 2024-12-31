use helpers::Puzzle;

fn main() {
    Day13::run();
}

struct Day13;

const BUTTON_A_START: &str = "Button A:";
const BUTTON_B_START: &str = "Button B:";
const PRIZE_START: &str = "Prize:";

#[derive(Debug, Clone)]
enum Line {
    ButtonA,
    ButtonB,
    Prize,
}

#[derive(Debug)]
struct QuestionFailure {
    line: String,
    kind: Line,
}

fn check_line(line_str: &str, kind: Line) -> Result<(i128, i128), QuestionFailure> {
    let prefix = match kind {
        Line::ButtonA => BUTTON_A_START,
        Line::ButtonB => BUTTON_B_START,
        Line::Prize => PRIZE_START,
    };
    if !line_str.starts_with(prefix) {
        return Err(QuestionFailure {
            line: line_str.to_string(),
            kind,
        });
    }
    let str_vals = line_str[prefix.len()..]
        .trim()
        .split_once(", ")
        .ok_or_else(|| QuestionFailure {
            line: line_str.to_string(),
            kind: kind.clone(),
        })?;

    //println!("str_vals: {:?}", str_vals);

    match (str_vals.0[2..].parse(), str_vals.1[2..].parse()) {
        (Ok(val_1), Ok(val_2)) => Ok((val_1, val_2)),
        _ => Err(QuestionFailure {
            line: line_str.to_string(),
            kind,
        }),
    }
}

#[derive(Debug)]
struct Question {
    puzzle_a: (i128, i128),
    puzzle_b: (i128, i128),
    prize: (i128, i128),
}

impl TryFrom<[&str; 3]> for Question {
    type Error = QuestionFailure;

    fn try_from(value: [&str; 3]) -> Result<Self, Self::Error> {
        let puzzle_a = check_line(value[0], Line::ButtonA)?;
        let puzzle_b = check_line(value[1], Line::ButtonB)?;
        let prize = check_line(value[2], Line::Prize)?;
        Ok(Question {
            puzzle_a,
            puzzle_b,
            prize,
        })
    }
}

impl Question {
    fn x1(&self) -> i128 {
        self.puzzle_a.0
    }
    fn y1(&self) -> i128 {
        self.puzzle_a.1
    }
    fn x2(&self) -> i128 {
        self.puzzle_b.0
    }
    fn y2(&self) -> i128 {
        self.puzzle_b.1
    }
    fn xp(&self) -> i128 {
        self.prize.0
    }
    fn yp(&self) -> i128 {
        self.prize.1
    }

    fn calc_answer(&self) -> Result<(i128, i128), String> {
        let n2_numerator = self.y1() * self.xp() - self.x1() * self.yp();
        let n2_denominator = self.x2() * self.y1() - self.x1() * self.y2();
        println!("n2_numerator: {n2_numerator}, n2_denominator: {n2_denominator}");
        if n2_numerator % n2_denominator != 0 {
            return Err("No solution because n2 can't be evenly divided".to_string());
        }
        let n2 = n2_numerator / n2_denominator;
        // TODO: can this happen?
        if n2 < 0 {
            return Err("No solutino because n2 is negative".to_string());
        }
        println!("n2: {n2}");

        let n1_numerator = self.yp() - n2 * self.y2();
        let n1_denominator = self.y1();
        println!("n1_numerator: {n1_numerator}, n1_denominator: {n1_denominator}",);
        if n1_numerator % n1_denominator != 0 {
            return Err("No solution because n1 can't be evenly divided".to_string());
        }
        let n1 = n1_numerator / self.y1();
        // TODO: Can this happen?
        if n1 < 0 {
            return Err("No solutino because n1 is negative".to_string());
        }
        println!("n1: {n1}");

        Ok((n1, n2))
    }
}

fn get_questions(contents: String) -> Result<Vec<Question>, QuestionFailure> {
    let mut line_iter = contents.lines().peekable();
    // TODO: I think this is an overestimate? Not sure though
    let mut result = Vec::with_capacity(contents.len() / 3);
    while line_iter.peek().is_some() {
        let lines = [
            line_iter.next().expect("First line has to exist"),
            line_iter.next().expect("Second line has to exist"),
            line_iter.next().expect("Third line has to exist"),
        ];
        // Skips the blank line
        line_iter.next();
        let question: Question = lines
            .try_into()
            .unwrap_or_else(|err| panic!("Failed to parse puzzle: {:?}", err));
        //println!("Question: {:?}", question);
        result.push(question);
    }
    Ok(result)
}

impl Puzzle for Day13 {
    fn puzzle_1(contents: String) {
        let mut sum = 0;
        get_questions(contents)
            .unwrap_or_else(|err| panic!("Failed to generate a question because: {:?}", err))
            .into_iter()
            .for_each(|question| match question.calc_answer() {
                Ok((n1, n2)) => {
                    println!("A: {n1}, B: {n2}");
                    sum += 3 * n1 + n2;
                }
                Err(err) => println!("No solution: {}", err),
            });

        println!("Tokens for the max win: {sum}");
    }

    fn puzzle_2(contents: String) {
        let mut sum = 0;
        get_questions(contents)
            .unwrap_or_else(|err| panic!("Failed to generate a question because: {:?}", err))
            .iter_mut()
            .map(|question| {
                question.prize.0 += 10_000_000_000_000;
                question.prize.1 += 10_000_000_000_000;
                question
            })
            .for_each(|question| match question.calc_answer() {
                Ok((n1, n2)) => {
                    println!("A: {n1}, B: {n2}");
                    sum += 3 * n1 + n2;
                }
                Err(err) => println!("No solution: {}", err),
            });
        println!("Tokens for the max win: {sum}");
    }
}
