extern crate core;

use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    let lines: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let oc = chars.next().unwrap() as u8 - b'@'; //convert A, B, C to 1, 2, 3.
            chars.next();
            let yc = chars.next().unwrap() as u8 - b'@' - 23; //convert X, Y, Z to 1, 2, 3.
            (oc as i32, yc as i32)
        })
        .collect();
    solve1(&lines);
    solve2(&lines);
}

fn solve1(lines: &Vec<(i32, i32)>) {
    let mut score = 0;
    for (oc, yc) in lines {
        match yc-oc {
            0 => score += yc + 3, //draw
            1 | -2 => score += yc + 6, //win
            2 | -1 => score += yc + 0,
            _ => panic!("Unreachable code."),

        }
    }
    println!("The score is {score}.")
}

fn solve2(lines: &Vec<(i32, i32)>) {
    let mut score = 0;
    for (oc, res) in lines {
        match (oc, res) {
            (oc, 2) => score += oc + (res - 1) * 3,
            (1, 1) => score += 3 + (res - 1) * 3, //3 for scissors
            (1, 3) => score += 2 + (res - 1) * 3, //2 for paper
            (2, 1) => score += 1 + (res - 1) * 3, //1 for rock
            (2, 3) => score += 3 + (res - 1) * 3, //3 for scissors
            (3, 1) => score += 2 + (res - 1) * 3, //2 for paper
            (3, 3) => score += 1 + (res - 1) * 3, //1 for rock
            _ => panic!("Unreachable code."),
        }
    }
    println!("The score is {score}.")
}
