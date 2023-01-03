use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    solve1(&input);
    solve2(&input);
}

fn solve1(input: &str) {
    let result: u32 = input
        .lines()
        .flat_map(|line| {
            let (h1, h2) = line.split_at(line.len()/2);
            let s1 = h1.chars().collect::<HashSet<char>>();
            let s2 = h2.chars().collect::<HashSet<char>>();
            s1.intersection(&s2).map(|c|
                if c.is_ascii_lowercase() {
                    c.to_owned() as u32 - 96u32
                } else {
                    c.to_owned() as u32 - 64u32 + 26u32
                }
            ).collect::<Vec<u32>>()
        })
        .sum();
    println!("The result is {result}.");
}

fn solve2(input: &str) {
    let lines: Vec<&str> = input
        .lines()
        .collect();
    let result: u32 = lines
        .chunks(3)
        .flat_map(|chunks| {
            let s1 = chunks[0].chars().collect::<HashSet<char>>();
            let s2 = chunks[1].chars().collect::<HashSet<char>>();
            let s3 = chunks[2].chars().collect::<HashSet<char>>();
            let s4 = s1.intersection(&s2).map(|c| c.to_owned()).collect::<HashSet<char>>();
            s3.intersection(&s4).map(|c|
                if c.is_ascii_lowercase() {
                    c.to_owned() as u32 - 96u32
                } else {
                    c.to_owned() as u32 - 64u32 + 26u32
                }
            ).collect::<Vec<u32>>()
        })
        .sum();
    println!("The result is {result}.");
}
