use regex::*;
use std::collections::HashMap;

struct Contents {
    bags: HashMap<String, usize>,
}

impl Contents {
    fn new() -> Contents {
        Contents {
            bags: HashMap::new()
        }
    }
}

fn contains_bag(rules: &HashMap<String, Contents>, needle: &String, current_bag: &String) -> bool {
    let contents = rules.get(current_bag).unwrap();
    if let Some(_) = contents.bags.get(needle) {
        return true;
    }

    for sub_bag in contents.bags.keys() {
        if contains_bag(rules, needle, sub_bag) {
            return true;
        }
    }

    return false;
}

fn count_children(rules: &HashMap<String, Contents>, current_bag: &String) -> usize {
    let contents = rules.get(current_bag).unwrap();
    let mut sum = 0;
    for sub_bag in contents.bags.keys() {
        let mul = contents.bags.get(sub_bag).unwrap();
        sum = sum + ((count_children(rules, sub_bag) + 1) * mul);
    }

    return sum;
}

fn main() {
    let rules = include_str!("../../../input07").split("\n").collect::<Vec<&str>>();

    let reg = Regex::new(r#"(\d+) (.*) bags?"#).unwrap();
    let mut bag_rules: HashMap<String, Contents> = HashMap::new();
    for rule in rules.iter() {
        let parts = rule.split(" bags contain ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("wrong size: {}", parts.len());
        }
        let name = String::from(parts[0]);
        let mut contents = Contents::new();
        parts[1].split(",").for_each(|dep| {
            if let Some(c) = reg.captures(dep) {
                let count = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let dep_name = String::from(c.get(2).unwrap().as_str());
                contents.bags.insert(dep_name, count);
            }
        });
        bag_rules.insert(name, contents);
    }

    println!("Parsed {} rules", bag_rules.len());
    println!("Part1: {} bags contains at least 1 shiny gold bag", bag_rules.keys().filter(|bag| contains_bag(&bag_rules, &String::from("shiny gold"), bag)).count());
    println!("Part2: {} bags inside 1 shiny gold bag", count_children(&bag_rules, &String::from("shiny gold")));

}
