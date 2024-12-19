#[allow(dead_code)]
static EXAMPLE: &str = include_str!("example.txt");
static INPUT: &str = include_str!("input.txt");

// Using some structs this time for fun

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

#[derive(Debug)]
struct Update {
    pages: Vec<u32>,
}

impl Update {
    pub fn rule_passes(&self, rule: &Rule) -> bool {
        if !self.pages.contains(&rule.before) || !self.pages.contains(&rule.after) {
            return true;
        }

        let mut before_seen = false;

        for num in &self.pages {
            if *num == rule.after && !before_seen {
                return false;
            }

            if *num == rule.before {
                before_seen = true;
            }
        }

        true
    }

    pub fn middle_number(&self) -> u32 {
        *self.pages.get(self.pages.len() / 2).unwrap()
    }
}

#[derive(Debug)]
struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

fn parse() -> Input {
    let mut split = INPUT.split("\n\n");
    let rules: Vec<Rule> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split('|');
            let (before, after) = (
                split.next().unwrap().parse::<u32>().unwrap(),
                split.next().unwrap().parse::<u32>().unwrap(),
            );
            Rule { before, after }
        })
        .collect();

    let updates: Vec<Update> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let pages: Vec<u32> = line
                .split(',')
                .map(|str| str.parse::<u32>().unwrap())
                .collect();

            Update { pages }
        })
        .collect();

    Input { rules, updates }
}

fn part_1() {
    let input = parse();

    let mut total = 0;

    'outer: for update in &input.updates {
        for rule in &input.rules {
            if !update.rule_passes(rule) {
                continue 'outer;
            }
        }

        total += update.middle_number();
    }

    println!("{total}");
}

fn main() {
    part_1();
}
