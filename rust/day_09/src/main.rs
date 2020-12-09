fn is_sum(nums: &Vec<i64>, value: i64, start: usize, size: usize) -> bool {
    for i in start..start+size {
        for u in i..start+size {
            if nums[i] + nums[u] == value {
                return true;
            }
        }
    }
    false
}

fn main() {
    let nums = include_str!("../../../input09").split("\n")
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    let mut invalid_num = 0;
    for i in 25..nums.len() {
        if !is_sum(&nums, nums[i], i - 25, 25) {
            println!("Part1, first invalid number: {}", nums[i]);
            invalid_num = nums[i];
            break;
        }
    }

    for size in 2..nums.len() - 1 {
        for start in size..nums.len() - size - 1 {
            let subrange = &nums[start..start+size];
            if subrange.iter().sum::<i64>() == invalid_num {
                let mut sorted: Vec<i64> = Vec::new();
                sorted.extend(subrange);
                sorted.sort();
                println!("Part2, sum of {}", (sorted[0] + sorted[sorted.len() - 1]));
                return;
            }
        }
    }
}
