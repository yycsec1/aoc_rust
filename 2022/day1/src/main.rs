use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./src/input.txt").unwrap();
    part1(&input);
    part2(&input);

}

fn part1(input: &str) {
    let elf_loads: Vec<u32> = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|loads| loads.split("\n").map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();
    println!("The result is {}", elf_loads[0]);
}

fn part2(input: &str) {
let mut elf_loads: Vec<u32> = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|loads| loads.split("\n").map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();
    elf_loads.sort_by(|a, b| b.cmp(a));
    println!("The result is {}", elf_loads.iter().take(3).sum::<u32>());
}
