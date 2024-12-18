use std::collections::HashMap;

#[allow(dead_code)]
static EXAMPLE: &str = include_str!("example.txt");
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

fn part_2() {
    let columns: (Vec<u32>, Vec<u32>) = INPUT
        .lines()
        .filter_map(|line| {
            let mut split = line.split_whitespace();
            let first = split.next()?.parse::<u32>().ok()?;
            let second = split.next()?.parse::<u32>().ok()?;

            Some((first, second))
        })
        .collect();

    let mut second_list_set = HashMap::new();
    for n in columns.1 {
        second_list_set.insert(n, second_list_set.get(&n).map(|c| c + 1).unwrap_or(1));
    }

    let mut score = 0;
    for n in columns.0 {
        score += second_list_set.get(&n).map(|c| c * n).unwrap_or(0);
    }

    println!("{score}")
}

fn main() {
    part_1();
    part_2();
}
