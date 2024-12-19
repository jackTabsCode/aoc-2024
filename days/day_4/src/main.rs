#[allow(dead_code)]
static EXAMPLE: &str = include_str!("example.txt");
static INPUT: &str = include_str!("input.txt");

fn build_matrix() -> Vec<Vec<char>> {
    INPUT.lines().map(|line| line.chars().collect()).collect()
}

static XMAS: &str = "XMAS";
static PART_1_DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn get_ch_at_pos(mat: &[Vec<char>], row: usize, col: usize) -> Option<char> {
    mat.get(row)?.get(col).copied()
}

fn part_1() {
    let mat = build_matrix();
    let xmas_chars: Vec<char> = XMAS.chars().collect();

    let mut hits = 0;

    for (row, cols) in mat.iter().enumerate() {
        for (col, _) in cols.iter().enumerate() {
            for (x, y) in PART_1_DIRECTIONS {
                let mut found = true;

                for (i, ch) in xmas_chars.iter().enumerate() {
                    let check_row = row as isize + i as isize * x;
                    let check_col = col as isize + i as isize * y;

                    if get_ch_at_pos(&mat, check_row as usize, check_col as usize) != Some(*ch) {
                        found = false;
                        break;
                    }
                }

                if found {
                    hits += 1;
                }
            }
        }
    }

    println!("{hits}");
}

static PART_2_DIRECTIONS: [(isize, isize); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
static MAS: [char; 3] = ['M', 'A', 'S'];

fn part_2() {
    let mat = build_matrix();

    let mut hits = 0;

    for (row, cols) in mat.iter().enumerate() {
        for (col, _) in cols.iter().enumerate() {
            let mut diagonals = 0;

            for (x, y) in PART_2_DIRECTIONS {
                let mut found = true;

                for (i, mas_ch) in MAS.iter().enumerate() {
                    let check_row = row as isize + (i as isize - 1) * x;
                    let check_col = col as isize + (i as isize - 1) * y;

                    if get_ch_at_pos(&mat, check_row as usize, check_col as usize) != Some(*mas_ch)
                    {
                        found = false;
                        break;
                    }
                }

                if found {
                    diagonals += 1;
                }
            }

            if diagonals >= 2 {
                hits += 1;
            }
        }
    }

    println!("{hits}");
}

fn main() {
    part_1();
    part_2();
}
