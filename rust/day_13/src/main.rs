fn main() {
    let data = include_str!("../../../input13").split("\n").collect::<Vec<&str>>();
    let timestamp = data[0].parse::<u32>().unwrap();
    let busses = data[1].split(",").filter(|id| *id != "x").map(|id| id.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut times: Vec<(u32, u32)> = Vec::new();
    busses.iter().for_each(|id| {
        let earliest_departure = ((timestamp / id) + (if timestamp % id == 0 { 0 } else { 1 }))*id;
        times.push((*id, earliest_departure));
    });

    times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let earliest_departure = times[0];
    println!("Part1, Earliest departure: {:?} => {}", earliest_departure, (earliest_departure.0 * (earliest_departure.1 - timestamp)));

    let ids = data[1].split(",").enumerate().filter(|(_, v)| *v != "x").map(|(i, id)| (i as u64, id.parse::<u64>().unwrap())).collect::<Vec<(u64, u64)>>(); // (t+x minutes, buss_id)
    println!("Ids: {:?}", ids);

    let mut tot_period = 1;
    let mut tot_offset = 0;
    for (buss_offset, buss_period) in ids.iter() {
        let new_period = buss_period * tot_period;
        for i in 0..*buss_period {
            let new_time = tot_period * i + tot_offset;
            if (new_time + buss_offset) % buss_period == 0 {
                tot_period = new_period;
                tot_offset = new_time;
                println!("Found tot_period {} and tot_offset {} for id {}", tot_period, tot_offset, buss_period);
                break;
            }
            if i == buss_period - 1 {
                panic!("Didn't find any period for id {}", buss_period);
            }
        }
    }
    println!("Part2: tot_period {} and tot_offset {}", tot_period, tot_offset);
}
