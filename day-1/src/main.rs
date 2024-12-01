use std::{collections::HashMap, time::Instant};
const INPUT: &'static str = include_str!("../input.txt");
const MILLIS_PER_SEC: u64 = 1_000;

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
        dur2.as_secs_f64() * (MILLIS_PER_SEC as f64) // millis
    );
}

#[allow(non_snake_case)]
fn part2(input: &str) -> usize {
    let mut frequecy_map = HashMap::new();
    let mut list1 = Vec::new();

    input.lines().for_each(|l| {
        let mut numbers = l.split_whitespace();
        list1.push(numbers.next().unwrap().parse::<isize>().unwrap());

        let n2 = numbers.next().unwrap().parse::<isize>().unwrap();
        match frequecy_map.get_mut(&n2) {
            Some(freq) => *(freq) += 1,
            None => {
                frequecy_map.insert(n2, 1);
            }
        };
    });

    let sum = list1.iter().fold(0, |acc, n1| {
        let score = n1 * frequecy_map.get(&n1).unwrap_or(&0);
        acc + score
    });
    sum as usize
}

#[allow(non_snake_case)]
fn part1(input: &str) -> usize {
    let (mut list1, mut list2) = (Vec::new(), Vec::new());
    input.lines().for_each(|l| {
        let mut numbers = l.split_whitespace();
        list1.push(numbers.next().unwrap().parse::<isize>().unwrap());
        list2.push(numbers.next().unwrap().parse::<isize>().unwrap());
    });

    list1.sort();
    list2.sort();

    let sum = list1.iter().zip(list2.iter()).fold(0, |acc, (i1, i2)| {
        let dist = i1 - i2;
        acc + if dist < 0 { dist * -1 } else { dist }
    });
    sum as usize
}
