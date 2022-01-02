extern crate core;

use std::fs::File;
use std::io::{BufReader, BufRead};

enum OverLapOrder {
    FirstToSecond,
    SecondToFirst,
    Bigger
}

struct OverLapStat {
    length: i32,
    order: OverLapOrder
}

fn get_overlap(first: &String, second: &String) -> OverLapStat {
    let mut counter = if first.len() < second.len() {first.len()} else {second.len()} as i32;
    if first.contains(second) || second.contains(first) {
        return OverLapStat { length: counter, order: OverLapOrder::Bigger }
    }

    while counter > 0 {
        let prefix_candidate: String = second.chars().skip(second.len() - counter as usize).collect();
        if first.starts_with(&prefix_candidate) {
            return OverLapStat { length: counter, order: OverLapOrder::SecondToFirst };
        }
        let suffix_candidate: String = second.chars().take(counter as usize).collect();
        if first.ends_with(&suffix_candidate) {
            return OverLapStat { length: counter, order: OverLapOrder::FirstToSecond };
        }
        counter = counter - 1;
    }
    OverLapStat { length: 0, order: OverLapOrder::FirstToSecond }
}

fn concat(first: String, second: String, length: usize) -> String {
    let mut text_result = first;
    let sub: String = second.chars().skip(length).collect();
    text_result.push_str(&sub[..]);
    text_result
}

fn remove_maximum_overlap(fragments: &mut Vec<String>) {
    let mut first_idx = 0;
    let mut second_idx = 1;
    let mut max_over_lap = OverLapStat{ length: i32::MIN, order: OverLapOrder::FirstToSecond };

    for i in 0..fragments.len() - 1 {
        for j in i + 1..fragments.len() {
            let first = fragments.get(i);
            let second = fragments.get(j);
            let over_lap = get_overlap(first.unwrap(), second.unwrap());
            if over_lap.length > max_over_lap.length {
                first_idx = i;
                second_idx = j;
                max_over_lap = over_lap;
            }
        }
    }

    let second = fragments.remove(second_idx);
    let first = fragments.remove(first_idx);
    let merge_result = match max_over_lap.order {
        OverLapOrder::Bigger => {
            if first.len() > second.len() {first} else {second}
        },
        OverLapOrder::FirstToSecond => {
            concat(first, second, max_over_lap.length as usize)
        },
        OverLapOrder::SecondToFirst => {
            concat(second, first, max_over_lap.length as usize)
        }
    };

    fragments.insert(first_idx, merge_result);
}

fn reassemble(line: &String) -> String {
    let mut fragments: Vec<String> = line.split(';')
        .map(|fragment| String::from(fragment))
        .collect();
    while fragments.len() > 1 {
        remove_maximum_overlap(&mut fragments);
    }
    fragments.remove(0)
}

fn main() {
    let f = File::open("D:/Downloads/rust projects/practice/src/input.txt").unwrap();
    BufReader::new(&f).lines()
        .map(|line| line.unwrap())
        .map(|line| reassemble(&line))
        .for_each(|line| println!("{}", line));
}
