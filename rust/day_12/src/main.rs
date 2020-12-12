use std::cmp::{max, min};


fn manhattan_distance(x0: i32, y0: i32, x1: i32, y1: i32) -> u32 {
    let x = max(x0, x1) - min(x0, x1);
    let y = max(y0, y1) - min(y0, y1);
    return x as u32 + y as u32;
}

fn main() {
    let data = include_str!("../../../input12").split("\n").collect::<Vec<&str>>();
    let dirs: Vec<(i32, i32)>= vec![
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ];
    let mut current_dir = 1i32;    // 1 is east
    let mut x = 0i32;
    let mut y = 0i32;
    for line in data.iter() {
        let value = line[1..].parse::<i32>().unwrap();
        match line.chars().nth(0).unwrap() {
            'N' => y = y - value,
            'S' => y = y + value,
            'E' => x = x + value,
            'W' => x = x - value,
            'L' => {
                current_dir = current_dir - (value / 90);
                if current_dir < 0 {
                    current_dir = current_dir + 4;
                }
            },
            'R' => current_dir = (current_dir + (value / 90)) % 4,
            'F' => {
                x = x + (dirs[current_dir as usize].0 * value);
                y = y + (dirs[current_dir as usize].1 * value);
            },
            _ => panic!("Unhandled line: {}", line),
        }
    }
    println!("Part1: distance: {}", manhattan_distance(0, 0, x, y));

    
    let mut s_x = 0i32;
    let mut s_y = 0i32;
    let mut w_x = 10i32;
    let mut w_y = -1i32;
    for line in data.iter() {
        let value = line[1..].parse::<i32>().unwrap();
        match line.chars().nth(0).unwrap() {
            'N' => w_y = w_y - value,
            'S' => w_y = w_y + value,
            'E' => w_x = w_x + value,
            'W' => w_x = w_x - value,
            'L' => {
                let rot = value / 90;
                match rot {
                    0 | 4 => (),
                    1 => { 
                        let tmp = w_x;
                        w_x = w_y;
                        w_y = tmp * -1;
                    },
                    2 => { 
                        w_x = w_x * -1;
                        w_y = w_y * -1;
                    },
                    3 => { 
                        let tmp = w_x;
                        w_x = w_y * -1;
                        w_y = tmp;
                    },
                    v => panic!("Bad rotation value {}", v),
                }
            },
            'R' => {
                let rot = value / 90;
                match rot {
                    0 | 4 => (),
                    1 => { 
                        let tmp = w_x;
                        w_x = w_y * -1;
                        w_y = tmp;
                    },
                    2 => { 
                        w_x = w_x * -1;
                        w_y = w_y * -1;
                    },
                    3 => { 
                        let tmp = w_x;
                        w_x = w_y;
                        w_y = tmp * -1;
                    },
                    v => panic!("Bad rotation value {}", v),
                }
            },
            'F' => {
                s_x = s_x + (w_x * value);
                s_y = s_y + (w_y * value);
            },
            _ => panic!("Unhandled line: {}", line),
        }
    }
    println!("Part2: distance: {}", manhattan_distance(0, 0, s_x, s_y));
}
