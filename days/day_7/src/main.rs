#[allow(dead_code)]
static EXAMPLE: &str = include_str!("example.txt");
static INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
enum Equation {
    Number(u64),
    Add(Box<Equation>, Box<Equation>),
    Multiply(Box<Equation>, Box<Equation>),
    Concat(Box<Equation>, Box<Equation>),
}

impl Equation {
    fn evaluate(&self) -> u64 {
        match self {
            Equation::Number(value) => *value,
            Equation::Add(left, right) => left.evaluate() + right.evaluate(),
            Equation::Multiply(left, right) => left.evaluate() * right.evaluate(),
            Equation::Concat(left, right) => (left.evaluate().to_string()
                + right.evaluate().to_string().as_str())
            .parse::<u64>()
            .unwrap(),
        }
    }
}

#[derive(Debug)]
struct Calibration {
    test_value: u64,
    numbers: Vec<u64>,
}

impl Calibration {
    pub fn generate_equations(&self, concat: bool) -> Vec<Equation> {
        let mut equations = Vec::<Equation>::new();

        fn generate(
            numbers: &[u64],
            current: Equation,
            index: usize,
            equations: &mut Vec<Equation>,
            concat: bool,
        ) {
            if index == numbers.len() - 1 {
                equations.push(current);
                return;
            }

            let add_equation = Equation::Add(
                Box::new(current.clone()),
                Box::new(Equation::Number(numbers[index + 1])),
            );
            generate(numbers, add_equation, index + 1, equations, concat);

            let multiply_equation = Equation::Multiply(
                Box::new(current.clone()),
                Box::new(Equation::Number(numbers[index + 1])),
            );
            generate(numbers, multiply_equation, index + 1, equations, concat);

            if concat {
                let concat_equation = Equation::Concat(
                    Box::new(current.clone()),
                    Box::new(Equation::Number(numbers[index + 1])),
                );
                generate(numbers, concat_equation, index + 1, equations, concat);
            }
        }

        generate(
            &self.numbers,
            Equation::Number(self.numbers[0]),
            0,
            &mut equations,
            concat,
        );

        equations
    }
}

fn parse() -> Vec<Calibration> {
    INPUT
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            let test_value = split.next().unwrap().parse::<u64>().unwrap();
            let numbers = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();

            Calibration {
                test_value,
                numbers,
            }
        })
        .collect()
}

fn part_1() {
    let calibrations = parse();

    let mut total = 0;

    'outer: for calibration in calibrations {
        let equations = calibration.generate_equations(false);

        for equation in equations {
            if equation.evaluate() == calibration.test_value {
                total += calibration.test_value;
                continue 'outer;
            }
        }
    }

    println!("{total}");
}

fn part_2() {
    let calibrations = parse();

    let mut total = 0;

    'outer: for calibration in calibrations {
        let equations = calibration.generate_equations(true);

        for equation in equations {
            if equation.evaluate() == calibration.test_value {
                total += calibration.test_value;
                continue 'outer;
            }
        }
    }

    println!("{total}");
}

fn main() {
    part_1();
    part_2();
}
