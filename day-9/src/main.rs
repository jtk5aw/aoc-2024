use std::iter;

use helpers::Puzzle;

struct Day9;

fn main() {
    Day9::run();
}

fn file_and_gaps(digit_vec: Vec<usize>) -> (usize, Vec<usize>, Vec<usize>) {
    let mut files = Vec::with_capacity(digit_vec.len() / 2);
    let mut gaps = Vec::with_capacity(digit_vec.len() / 2);
    let mut max_final_file_size: usize = 0;

    for (digit_idx, digit) in digit_vec.into_iter().enumerate() {
        max_final_file_size += digit;
        if digit_idx % 2 == 0 {
            files.push(digit);
        } else {
            gaps.push(digit);
        }
    }

    (max_final_file_size, files, gaps)
}

impl Puzzle for Day9 {
    fn puzzle_1(contents: String) {
        let chars = contents.trim().chars().collect::<Vec<_>>();
        let digit_vec = chars
            .iter()
            .map(|digit_char| {
                usize::try_from(digit_char.to_digit(10).expect("Has to be a digit"))
                    .expect("has to be a usize")
            })
            .collect::<Vec<_>>();

        let (max_final_file_size, mut files, gaps) = file_and_gaps(digit_vec);

        let mut final_file = Vec::with_capacity(max_final_file_size);
        let mut front_idx = 0;
        let mut back_idx = files.len() - 1;
        let mut gap_size = 0;

        while back_idx != 0 && front_idx != files.len() {
            if gap_size == 0 && front_idx < files.len() {
                iter::repeat(front_idx)
                    .take(files[front_idx])
                    .for_each(|val| final_file.push(val));
                files[front_idx] = 0;
                gap_size = gaps[front_idx];
                front_idx += 1;
            }

            if gap_size > 0 && files[back_idx] > 0 {
                final_file.push(back_idx);

                files[back_idx] -= 1;
                gap_size -= 1;
            }

            if files[back_idx] == 0 {
                back_idx -= 1;
            }
        }

        let mut checksum = 0;
        for idx in 0..final_file.len() {
            checksum += final_file[idx] * idx;
        }
        println!("The final checksum is: {checksum}");
    }

    fn puzzle_2(contents: String) {
        let chars = contents.trim().chars().collect::<Vec<_>>();
        let digit_vec = chars
            .iter()
            .map(|digit_char| {
                usize::try_from(digit_char.to_digit(10).expect("Has to be a digit"))
                    .expect("has to be a usize")
            })
            .collect::<Vec<_>>();

        let (max_final_file_size, files, mut gaps) = file_and_gaps(digit_vec);
        let mut file_drained = vec![false; files.len()];
        let mut final_file = vec![0; max_final_file_size];
        let mut final_file_idx = 0;
        let mut front_file_idx = 0;

        println!("{:?}", files);
        while front_file_idx < files.len() {
            if file_drained[front_file_idx] {
                final_file_idx += files[front_file_idx];
            } else {
                iter::repeat(front_file_idx)
                    .take(files[front_file_idx])
                    .for_each(|val| {
                        final_file[final_file_idx] = val;
                        final_file_idx += 1;
                    });
            }

            if 0 < gaps.len() {
                let mut gap_len = gaps.remove(0);
                for idx in (front_file_idx + 1..files.len()).rev() {
                    if !file_drained[idx] && files[idx] <= gap_len {
                        iter::repeat(idx).take(files[idx]).for_each(|val| {
                            final_file[final_file_idx] = val;
                            final_file_idx += 1;
                            gap_len -= 1;
                        });
                        file_drained[idx] = true;
                    }
                }
                final_file_idx += gap_len;
            }

            front_file_idx += 1;
        }

        println!("{:?}", final_file);

        let check_sum = final_file
            .iter()
            .enumerate()
            .fold(0 as usize, |acc, (idx, val)| acc + idx * val);

        println!("check sum is: {check_sum}");
    }
}
