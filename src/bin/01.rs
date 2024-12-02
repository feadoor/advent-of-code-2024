use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/01.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn extract_numbers(line: String) -> (usize, usize) {
    let mut numbers = line.split_ascii_whitespace().map(|n| n.parse::<usize>().expect("Not a number"));
    (numbers.next().expect("Not enough numbers"), numbers.next().expect("Not enough numbers"))
}

fn parse_lists<I: Iterator<Item = String>>(lines: I) -> (Vec<usize>, Vec<usize>) {
    lines.map(extract_numbers).unzip()
}

fn distance_between(left: &mut [usize], right: &mut [usize]) -> usize {
    left.sort(); right.sort();
    left.iter().zip(right.iter()).map(|(&l, &r)| l.abs_diff(r)).sum()
}

fn to_counts(list: &[usize]) -> HashMap<usize, usize> {
    list.iter().fold(HashMap::new(), |mut acc, &n| {
        *acc.entry(n).or_insert(0) += 1; acc
    })
}

fn similarity_score(left: &[usize], right: &[usize]) -> usize {
    let (left_counts, right_counts) = (to_counts(left), to_counts(right));
    left_counts.iter().map(|(n, &cnt)| n * cnt * right_counts.get(n).unwrap_or(&0)).sum()
}

fn main() {
    let (mut left, mut right) = parse_lists(read_lines());
    println!("Part 1: {}", distance_between(&mut left, &mut right));
    println!("Part 2: {}", similarity_score(&left, &right));
}
