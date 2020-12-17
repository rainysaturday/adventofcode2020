use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("../../../input14");
    let data = input.split("\n").collect::<Vec<&str>>();

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mem2: HashMap<u64, u64> = HashMap::new();
    let mem_reg = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    let mut keep_mask: u64 = 0;
    let mut set_mask: u64 = 0;
    let mut xs: usize = 0;

    for line in data.iter() {
        if line.starts_with("mask = ") {
            keep_mask = 0;
            set_mask = 0;
            line.chars().skip(7).for_each(|c| {
                set_mask = set_mask << 1;
                keep_mask = keep_mask << 1;
                match c {
                    '1' => set_mask = set_mask | 1,
                    'X' => keep_mask = keep_mask | 1,
                    _ => ()
                }
            });
            xs = line.chars().filter(|x| x == &'X').count();
        } else if let Some(c) = mem_reg.captures(line) {
            let addr = c.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let value = c.get(2).unwrap().as_str().parse::<u64>().unwrap();
            mem.insert(addr, (value & keep_mask) | set_mask);

            // Part 2: naive implementation, use memory like it is free!
            let addr_set_mask = set_mask;
            let flux_mask = keep_mask;
            for i in 0..2u64.pow(xs as u32) {
                // Remove flux-bits of address and set flux-bits
                let mut new_addr = addr & !flux_mask;
                let mut i_offset = 0;
                for offset in 0..36 {
                    if (flux_mask >> offset) & 1 == 1 {
                        new_addr = new_addr | (((i >> i_offset) & 1) << offset);
                        i_offset = i_offset + 1;
                    }
                }
                // Set set-bits
                new_addr = new_addr | addr_set_mask;
                mem2.insert(new_addr, value);
            }
        } else {
            panic!("Unknown line: {}", line);
        }
    }
    println!("Part1: sum of all mem_bytes: {}", mem.values().sum::<u64>());
    println!("Part2: sum of all mem_bytes: {}, {} unique locations", mem2.values().sum::<u64>(), mem2.len());
}
