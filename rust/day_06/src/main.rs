use std::collections::HashMap;

fn main() {
    let data = include_str!("../../../input06");
    let groups: Vec<&str> = data.split("\n\n").collect();

    let mut p1: usize = 0;
    let mut p2: usize = 0;
    groups.iter().for_each(|g| {
        let mut map: HashMap<char, usize> = HashMap::new();
        let persons: Vec<&str> = g.split("\n").collect();
        let person_count = persons.len();
        persons.iter().for_each(|person| person.chars().for_each(|q| {
            map.insert(q, if let Some(val) = map.get(&q) { *val + 1 } else { 1 });
        }));
        p1 = p1 + map.len();
        p2 = p2 + map.iter().filter(|(_, value)| **value == person_count).count();
    });
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}
