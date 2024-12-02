use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/02.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn parse_line<'a>(line: String) -> Vec<usize> {
    line.split_ascii_whitespace().map(|n| n.parse().expect("Not a number")).collect()
}

fn is_safe_iter<I: Iterator<Item = usize>>(numbers: I, max_difference: usize) -> bool {
    let mut signed_differences = numbers.tuple_windows().map(|(a, b)| (a.cmp(&b), a != b && a.abs_diff(b) <= max_difference));
    signed_differences.all_equal_value().map(|v| v.1).unwrap_or(false)
}

fn is_safe(numbers: &[usize], max_difference: usize) -> bool {
    is_safe_iter(numbers.iter().copied(), max_difference)
}

fn skipped_iterator<'a>(numbers: &'a [usize], index: usize) -> impl Iterator<Item = usize> + 'a {
    numbers[..index].iter().chain(numbers[index + 1..].iter()).copied()
}

fn is_safe_with_removal(numbers: &[usize], max_difference: usize) -> bool {
    (0 .. numbers.len()).any(|index| is_safe_iter(skipped_iterator(numbers, index), max_difference))
}

fn main() {
    let numbers = read_lines().map(parse_line).collect_vec();
    println!("Part 1: {}", numbers.iter().filter(|line| is_safe(line, 3)).count());
    println!("Part 2: {}", numbers.iter().filter(|line| is_safe(line, 3) || is_safe_with_removal(line, 3)).count());
}
