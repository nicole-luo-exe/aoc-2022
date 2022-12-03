use std::io::{self, stdin, BufReader};

#[allow(dead_code)]
fn part_one(input: &String) -> u32 {
    input
        .split('\n')
        .map(|l| {
            let r = l
                .split(' ')
                .map(|s| s.trim().parse::<char>().unwrap())
                .collect::<Vec<char>>();
            let i = r[1] as u32 - 'X' as u32;
            let j = r[0] as u32 - 'A' as u32;
            let outcome = if (j + 1) % 3 == i {
                6
            } else if (i + 1) % 3 == j {
                0
            } else {
                3
            };
            i + 1 + outcome
        })
        .fold(0, |sum, x| sum + x)
}

fn part_two(input: &String) -> i32 {
    input
        .split('\n')
        .map(|l| {
            let r = l
                .split(' ')
                .map(|s| s.trim().parse::<char>().unwrap())
                .collect::<Vec<char>>();
            let outcome = r[1] as i32 - 'X' as i32;
            let opp_move = r[0] as i32 - 'A' as i32;
            // lose => play -1 of opponent's move, draw => play 0, win => play 1
            (3 + opp_move + outcome - 1) % 3 + 1 + (outcome * 3)
        })
        .fold(0, |sum, x| sum + x)
}
fn main() {
    let input = io::read_to_string(BufReader::new(stdin())).unwrap();
    println!("Ans: {}", part_two(&input));
}
