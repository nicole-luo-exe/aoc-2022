#![feature(iter_array_chunks)]
use std::{
    collections::{HashMap, HashSet},
    io::{self, stdin, BufReader},
};

fn get_val(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

#[allow(dead_code)]
fn part_one(input: &String) -> u32 {
    input
        .split('\n')
        .map(|s| {
            let n = s.len();
            let mut first_half = s[0..n / 2].chars().collect::<HashSet<char>>();
            let res = s[n / 2..n]
                .chars()
                .filter_map(|c| {
                    if first_half.contains(&c) {
                        first_half.remove(&c); // impure functional lol
                        Some(get_val(c))
                    } else {
                        None
                    }
                })
                .fold(0, |sum, x| sum + x);
            res
        })
        .fold(0, |sum, x| sum + x)
}

fn part_two(input: &String) -> u32 {
    input
        .split('\n')
        .into_iter()
        .array_chunks::<3>()
        .map(|[x, y, z]| {
            let mut chars = x
                .chars()
                .into_iter()
                .map(|c| (c, 1))
                .collect::<HashMap<char, u8>>();
            for elf in [y, z] {
                for c in elf.chars().collect::<HashSet<char>>() {
                    if chars.contains_key(&c) {
                        *chars.get_mut(&c).unwrap() += 1;
                    }
                    if chars.get(&c) == Some(&3) {
                        return get_val(c);
                    }
                }
            }
            0
        })
        .fold(0, |sum, x| sum + x)
}

fn main() {
    let input = io::read_to_string(BufReader::new(stdin())).unwrap();
    println!("Ans: {}", part_two(&input));
}
