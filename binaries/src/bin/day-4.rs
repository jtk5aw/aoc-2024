use helpers::{read_grid, Puzzle};

fn main() {
    Day4::run();
}

struct Day4;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

impl Puzzle for Day4 {
    fn puzzle_1(contents: String) {
        let grid = read_grid(contents);
        let mut xmas_count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] == 'X' {
                    // search right
                    if c + 3 < grid[r].len()
                        && grid[r][c + 1] == XMAS[1]
                        && grid[r][c + 2] == XMAS[2]
                        && grid[r][c + 3] == XMAS[3]
                    {
                        xmas_count += 1;
                    }
                    // search left
                    if c >= 3
                        && grid[r][c - 1] == XMAS[1]
                        && grid[r][c - 2] == XMAS[2]
                        && grid[r][c - 3] == XMAS[3]
                    {
                        xmas_count += 1;
                    }
                    // search up
                    if r >= 3
                        && grid[r - 1][c] == XMAS[1]
                        && grid[r - 2][c] == XMAS[2]
                        && grid[r - 3][c] == XMAS[3]
                    {
                        xmas_count += 1;
                    }
                    // search down
                    if r + 3 < grid.len()
                        && grid[r + 1][c] == XMAS[1]
                        && grid[r + 2][c] == XMAS[2]
                        && grid[r + 3][c] == XMAS[3]
                    {
                        xmas_count += 1;
                    }
                    // search up-left
                    if r >= 3
                        && c >= 3
                        && grid[r - 1][c - 1] == XMAS[1]
                        && grid[r - 2][c - 2] == XMAS[2]
                        && grid[r - 3][c - 3] == XMAS[3]
                    {
                        xmas_count += 1;
                    }
                    // search up-right
                    if r >= 3
                        && c + 3 < grid[r].len()
                        && grid[r - 1][c + 1] == XMAS[1]
                        && grid[r - 2][c + 2] == XMAS[2]
                        && grid[r - 3][c + 3] == XMAS[3]
                    {
                        xmas_count += 1;
                    }
                    // search down-left
                    if r + 3 < grid.len()
                        && c >= 3
                        && grid[r + 1][c - 1] == XMAS[1]
                        && grid[r + 2][c - 2] == XMAS[2]
                        && grid[r + 3][c - 3] == XMAS[3]
                    {
                        xmas_count += 1;
                    }
                    // search down-right
                    if r + 3 < grid.len()
                        && c + 3 < grid[r].len()
                        && grid[r + 1][c + 1] == XMAS[1]
                        && grid[r + 2][c + 2] == XMAS[2]
                        && grid[r + 3][c + 3] == XMAS[3]
                    {
                        xmas_count += 1;
                    }
                }
            }
        }

        println!("xmas cound: {xmas_count}");
    }

    fn puzzle_2(contents: String) {
        let grid = read_grid(contents);
        let mut xmas_count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] == 'A' {
                    if r >= 1 && c >= 1 && r + 1 < grid.len() && c + 1 < grid[r].len() {
                        // NOTE: Almost certainly overcomplicated it. I assumed they didn't have to
                        // be diagonal across (which I think is still unclear from the rules) and
                        // could be like the following. Probalby an easier way to just check
                        // diagonals
                        // M.S
                        // .A.
                        // S.M
                        let corners_chars = [
                            grid[r - 1][c - 1],
                            grid[r + 1][c - 1],
                            grid[r - 1][c + 1],
                            grid[r + 1][c + 1],
                        ];
                        let (s_count, m_count) =
                            corners_chars
                                .iter()
                                .fold((0, 0), |(s_count, m_count), char_val| match char_val {
                                    'M' => (s_count, m_count + 1),
                                    'S' => (s_count + 1, m_count),
                                    _ => (s_count, m_count),
                                });

                        if s_count == 2 && m_count == 2 && grid[r - 1][c - 1] != grid[r + 1][c + 1]
                        {
                            xmas_count += 1;
                        }
                    }
                }
            }
        }

        println!("xmas count: {xmas_count}");
    }
}
