#[allow(dead_code)]
static EXAMPLE: &str = include_str!("example.txt");
static INPUT: &str = include_str!("input.txt");

fn build_matrix() -> Vec<Vec<char>> {
    INPUT.lines().map(|line| line.chars().collect()).collect()
}

static XMAS: &str = "XMAS";
static DIRECTIONS: [(isize, isize); 8] = [
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
            for (x, y) in DIRECTIONS {
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

fn main() {
    part_1();
}
