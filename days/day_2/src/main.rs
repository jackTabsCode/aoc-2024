#[allow(dead_code)]
static EXAMPLE: &str = include_str!("example.txt");
static INPUT: &str = include_str!("input.txt");

fn part_1() {
    let mut reports_safe = 0;

    for line in INPUT.lines() {
        let mut levels = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .peekable();

        let mut safe = true;

        let mut sign: Option<i32> = None;

        while let Some(level) = levels.next() {
            let next = levels.peek();
            if let Some(next) = next {
                let diff = level - next;
                let diff_sign = diff.signum();

                if let Some(sign) = sign {
                    if diff_sign != sign {
                        safe = false;
                        break;
                    }
                } else {
                    sign = Some(diff_sign);
                }

                if diff.abs() > 3 || diff.abs() < 1 {
                    safe = false;
                    break;
                }
            }
        }

        if safe {
            reports_safe += 1;
        }
    }

    println!("{reports_safe}")
}

fn part_2_report(levels: &[i32]) -> bool {
    let mut peekable = levels.iter().peekable();

    let mut sign: Option<i32> = None;

    while let Some(level) = peekable.next() {
        let next = peekable.peek();
        if let Some(next) = next {
            let diff = level - *next;
            let diff_sign = diff.signum();

            if let Some(sign) = sign {
                if diff_sign != sign {
                    return false;
                }
            } else {
                sign = Some(diff_sign);
            }

            if diff.abs() > 3 || diff.abs() < 1 {
                return false;
            }
        }
    }

    true
}

fn part_2() {
    let mut reports_safe = 0;

    for line in INPUT.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if part_2_report(&levels) {
            reports_safe += 1;
            continue;
        }

        let mut safe = false;

        for (index, _) in levels.iter().enumerate() {
            if safe {
                break;
            }

            let mut new = levels.clone();
            new.remove(index);

            if part_2_report(&new) {
                safe = true;
                break;
            }
        }

        if safe {
            reports_safe += 1;
        }
    }

    println!("{reports_safe}")
}

fn main() {
    part_1();
    part_2();
}
