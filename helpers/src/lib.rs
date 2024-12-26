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
