use std::cmp::max;
use std::collections::BinaryHeap;
use std::io;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn part_one() {
    let reader = BufReader::new(io::stdin());
    let mut res = 0u64;
    let mut cals = 0u64;
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.len() == 0 {
            res = max(res, cals);
            cals = 0;
        } else {
            cals += line.parse::<u64>().unwrap();
        }
    }
    println!("Elf carries at most ({}) cals.", res)
}

fn main() {
    //part two
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut heap = BinaryHeap::<i32>::new();
    input
        .split("\n\n")
        .map(|elf_str| {
            elf_str
                .split("\n")
                .fold(0, |sum, x| sum + x.parse::<i32>().unwrap())
        })
        .for_each(|cals| {
            if heap.len() < 3 {
                heap.push(cals * -1);
            } else if cals > (-1 * heap.peek().unwrap()) {
                heap.pop();
                heap.push(cals * -1);
            }
        });
    println!(
        "{} cals.",
        -1 * heap.into_iter().fold(0, |sum, val| sum + val)
    )
}
