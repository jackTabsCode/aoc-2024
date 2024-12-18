use regex::Regex;

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

fn part_2() {
    let mut total = 0;
    let mut enabled = true;

    let re = Regex::new(
        r"(?x)
    (do\(\)) # turns on multiplication
    |
    (don't\(\)) # turns off multiplication
    |
    (mul\((\d+),(\d+)\)) # multiply two numbers",
    )
    .unwrap();
    for cap in re.captures_iter(INPUT) {
        if cap.get(1).is_some() {
            enabled = true
        }

        if cap.get(2).is_some() {
            enabled = false
        }

        if enabled && cap.get(3).is_some() {
            if let (Some(x), Some(y)) = (cap.get(4), cap.get(5)) {
                let x = x.as_str().parse::<u32>().unwrap();
                let y = y.as_str().parse::<u32>().unwrap();

                total += x * y;
            }
        }
    }

    println!("{total}")
}

fn main() {
    part_1();
    part_2();
}
