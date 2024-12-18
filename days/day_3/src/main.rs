#[allow(dead_code)]
static EXAMPLE: &str = include_str!("example.txt");
static INPUT: &str = include_str!("input.txt");

fn parse_op(op: &str) -> Option<(u32, u32)> {
    let nums: Vec<&str> = op.split(',').collect();

    if nums.len() == 2 {
        if let (Ok(x), Ok(y)) = (nums[0].parse::<u32>(), nums[1].parse::<u32>()) {
            return Some((x, y));
        }
    }

    None
}

fn part_1() {
    let mut total = 0;
    let mut start = 0;

    while let Some(index) = INPUT[start..].find("mul(") {
        let new_index = start + index;
        let after_mul = &INPUT[new_index + 4..];

        if let Some(end_index) = after_mul.find(')') {
            let op = &after_mul[..end_index];

            if let Some((x, y)) = parse_op(op) {
                total += x * y;
            }
        }

        start = new_index + 4;
    }

    println!("{total}")
}

fn main() {
    part_1();
}
