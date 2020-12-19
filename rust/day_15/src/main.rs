use std::collections::HashMap;

fn add(history: &mut HashMap<u64, Vec<usize>>, said_value: u64, turn_nr: usize) {
    if let Some(hist) = history.get_mut(&said_value) {
        hist.push(turn_nr);
    } else {
        history.insert(said_value, vec![turn_nr]);
    }
}

fn main() {
    let input = include_str!("../../../input15");
    let data = input.split(",").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut history: HashMap<u64, Vec<usize>> = HashMap::new();
    let mut last_said = 0;
    for i in 0..data.len() {
        add(&mut history, data[i], i+1);
        last_said = data[i];
    }

    for turn in data.len()+1..=30000000 {
        let mut to_say: u64 = 0;
        if let Some(hist) = history.get(&last_said) {
            if hist.len() > 1 {
                to_say = (hist[hist.len()-1] - hist[hist.len()-2]) as u64;
            }
        }

        add(&mut history, to_say, turn);
        last_said = to_say;

        if turn == 2020 {
            println!("Part1, Turn {} said {}", turn, to_say);
        }
        if turn == 30000000 {
            println!("Part2, Turn {} said {}", turn, to_say);
        }
    }
}
