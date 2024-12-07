use std::time::Instant;
const MILLIS_PER_SEC: u64 = 1_000;
const INPUT: &'static str = include_str!("../input.txt");

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
    Unchanging,
}

impl From<i64> for Direction {
    fn from(value: i64) -> Self {
        match value {
            value if value < 0 => Direction::Decreasing,
            value if value > 0 => Direction::Increasing,
            value if value == 0 => Direction::Unchanging,
            _ => unreachable!(),
        }
    }
}

fn is_valid_change(i1: &i64, i2: &i64, direction: Direction) -> bool {
    let change = i1 - i2;
    let moving: Direction = change.into();
    if direction != moving {
        return false;
    }
    let change = change.abs();

    change >= 1 && change <= 3
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    input.lines().for_each(|line| {
        let inputs: Vec<i64> = line
            .split_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        if inputs.len() < 2 {
            return;
        }

        let direction: Direction = (inputs[0] - inputs[1]).into();
        if !inputs
            .iter()
            .zip(inputs.iter().skip(1))
            .any(|(i1, i2)| !is_valid_change(i1, i2, direction))
        {
            count += 1;
        }
    });
    count
}

fn is_line_valid(line: Vec<i64>) -> bool {
    for idx in -1..line.len() as isize {
        let mut iter = line
            .iter()
            .enumerate()
            .filter(|(i, _)| *i as isize != idx)
            .peekable();

        let mut iter2 = line
            .iter()
            .enumerate()
            .filter(|(i, _)| *i as isize != idx)
            .skip(1)
            .peekable();

        let direction: Direction = (iter.peek().unwrap().1 - iter2.peek().unwrap().1).into();

        let is_good = !iter
            .zip(iter2)
            .any(|((_, i1), (_, i2))| !is_valid_change(i1, i2, direction));

        if is_good {
            return is_good;
        }
    }
    false
}

fn part2(input: &str) -> usize {
    let mut count = 0;
    input.lines().for_each(|line| {
        let inputs: Vec<i64> = line
            .split_whitespace()
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        if inputs.len() < 2 {
            return;
        }
        if is_line_valid(inputs) {
            count += 1;
        }
    });
    count
}

fn main() {
    let start = Instant::now();
    let sol1 = part1(INPUT);
    let end = Instant::now();
    let dur = end - start;

    let start = Instant::now();
    let sol2 = part2(INPUT);
    let end = Instant::now();
    let dur2 = end - start;

    println!(
        "Part1: {} ({:.3}ms), Part2: {} ({:.3}ms)",
        sol1,
        dur.as_secs_f64() * (MILLIS_PER_SEC as f64), // millis
        sol2,
        dur2.as_secs_f64() * (MILLIS_PER_SEC as f64), // millis
    );
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_inputs() {
        let test_input_1 = ("7 6 4 2 1", 1); // true);
        let test_input_2 = ("1 2 7 8 9", 0); // false);
        let test_input_3 = ("9 7 6 2 1", 0); // false);
        let test_input_4 = ("1 3 2 4 5", 0); // false);
        let test_input_5 = ("8 6 4 4 1", 0); // false);
        let test_input_6 = ("1 3 6 7 9", 1); // true);
        assert_eq!(part1(test_input_1.0), test_input_1.1);
        assert_eq!(part1(test_input_2.0), test_input_2.1);
        assert_eq!(part1(test_input_3.0), test_input_3.1);
        assert_eq!(part1(test_input_4.0), test_input_4.1);
        assert_eq!(part1(test_input_5.0), test_input_5.1);
        assert_eq!(part1(test_input_6.0), test_input_6.1);
    }

    #[test]
    fn test_inputs_part2() {
        let test_input_1 = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(part2(test_input_1), 4);
    }
}
