static INPUT: &str = include_str!("input.txt");

fn part_1() {
    let mut columns: (Vec<u32>, Vec<u32>) = INPUT
        .lines()
        .filter_map(|line| {
            let mut split = line.split_whitespace();
            let first = split.next()?.parse::<u32>().ok()?;
            let second = split.next()?.parse::<u32>().ok()?;

            Some((first, second))
        })
        .collect();

    columns.0.sort();
    columns.1.sort();

    let mut total_dist = 0;

    for row in columns.0.iter().zip(columns.1.iter()) {
        total_dist += row.0.abs_diff(*row.1);
    }

    println!("{total_dist}");
}

fn main() {
    part_1();
}
