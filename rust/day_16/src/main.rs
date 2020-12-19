use std::collections::HashMap;
use regex::Regex;

fn is_valid_field(rules: &HashMap<&str, Vec<(u32, u32)>>, field_value: u32) -> bool {
    for rule in rules.values() {
        if is_in_rule(field_value, rule) {
            return true;
        }
    }
    false
}

fn is_in_rule(value: u32, rule: &Vec<(u32, u32)>) -> bool {
    for range in rule.iter() {
        if value >= range.0 && value <= range.1 {
            return true;
        }
    }
    false
}

fn main() {
    let input = include_str!("../../../input16");
    let data = input.split("\n").collect::<Vec<&str>>();
    let mut my_ticket: Vec<u32> = Vec::new();
    let mut nearby_tickets: Vec<Vec<u32>> = Vec::new();
    let mut rules: HashMap<&str, Vec<(u32, u32)>> = HashMap::new();

    let rules_reg = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let mut pos = 0;
    let mut parsing_nearby = false;
    while pos < data.len() {
        let line = data[pos].trim();
        match line {
            "" => (),
            "your ticket:" => {
                my_ticket = data[pos + 1].split(",").map(|n| n.trim().parse::<u32>().unwrap()).collect();
                pos = pos + 1;
            },
            "nearby tickets:" => parsing_nearby = true,
            l => {
                if parsing_nearby {
                    nearby_tickets.push(l.split(",").map(|n| n.parse::<u32>().unwrap()).collect());
                } else {
                    if let Some(c) = rules_reg.captures(l) {
                        rules.insert(c.get(1).unwrap().as_str(), vec![
                            (c.get(2).unwrap().as_str().parse::<u32>().unwrap(), c.get(3).unwrap().as_str().parse::<u32>().unwrap()), 
                            (c.get(4).unwrap().as_str().parse::<u32>().unwrap(), c.get(5).unwrap().as_str().parse::<u32>().unwrap()), 
                        ]);
                    } else {
                        panic!("Unhandled line: {} at pos {}", l, pos);
                    }
                }
            }
        }

        pos = pos + 1;
    }

    // Error rate
    let error_rate = nearby_tickets.iter().map(|t| t.iter().filter(|f| !is_valid_field(&rules, **f)).sum::<u32>()).sum::<u32>();
    println!("Part1, error_rate {}", error_rate);

    let valid_nearby = nearby_tickets.into_iter().filter(|t| t.iter().all(|f| is_valid_field(&rules, *f))).collect::<Vec<Vec<u32>>>();

    // Assumption, For each lap and for each column, there is only one rule that is valid for all values, and then we can ignore any previous found rules for subsequent laps
    let mut found_rules: HashMap<&str, usize> = HashMap::new();
    while found_rules.len() < rules.len() {
        for column in 0..rules.len() {
            let column_values = valid_nearby.iter().map(|t| t[column]).collect::<Vec<u32>>();
            let valid_rules = rules.iter().filter(|(_, r)| column_values.iter().all(|v| is_in_rule(*v, r) ))
                .filter(|(name, _)| !found_rules.contains_key(*name))
                .map(|(name, rules)| (*name, rules.clone()))
                .collect::<Vec<(&str, Vec<(u32,u32)>)>>();

            if valid_rules.len() == 1 {
                println!("Found rule: {:?} at column {}", valid_rules, column);
                found_rules.insert(valid_rules[0].0, column);
            }
        }
    }

    println!("Part2: mul-sum of departure fields on my ticket: {}", found_rules.iter().filter(|(name, _)| name.starts_with("departure")).map(|(_, i)| my_ticket[*i as usize] as u64).fold(1, |a, b| a * b));
}
