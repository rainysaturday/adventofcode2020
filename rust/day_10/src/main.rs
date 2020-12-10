use std::collections::HashMap;

fn main() {
    let mut data = include_str!("../../../input10")
        .split("\n")
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    data.sort();

    let mut current_jolt = 0;
    let mut diff_map: HashMap<u32, u32> = HashMap::new();
    for jolt in data.iter() {
        let diff = jolt - current_jolt;
        diff_map.insert(diff, diff_map.get(&diff).unwrap_or(&0) + 1);

        current_jolt = *jolt;
    }
    diff_map.insert(3, diff_map.get(&3).unwrap_or(&0) + 1); // An additional 3 for the last output
    println!("DiffMap: {:?}", diff_map);
    println!("Part1: 1-diffs * 3-diffs = {}" , (diff_map.get(&1).unwrap() * diff_map.get(&3).unwrap()));

    // Split in groups where the jolt diff is greater than 3
    let mut last = 0;
    let mut perms: Vec<u64> = Vec::new();
    for i in 0..data.len() {
        let last_jolt = if i == 0 { 0 } else { data[i - 1] };
        if data[i] - last_jolt == 3 || i == data.len() - 1 {
            let three_split = if i == data.len() - 1 { &data[last..data.len() ] } else { &data[last..i] };
            println!("Split: {:?}", three_split);
            
            let len = three_split.len() as u64 + if last == 0 && data[0] == 1 { 1 } else { 0 }; // Treat the first group as one item longer because of the 0 if first item is 1
            // Different group lengths have different permutations
            match len {
                1 => perms.push(1),
                2 => perms.push(1),
                3 => perms.push(2),
                4 => perms.push(4),
                5 => perms.push(7),
                a => panic!("Unhandled length: {}", a),
            }
            last = i;
        }
    }
    
    println!("Part2: permutation groups: {:?}, answer: {}", perms, perms.iter().fold(1u64, |a, b| a*b));
}
