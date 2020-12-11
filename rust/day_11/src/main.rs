fn adjacent(map: &Vec<Vec<char>>, x: usize, y: usize, needle: char, unlimited: bool) -> usize {
    let adj: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    adj.iter().map(|(off_x, off_y)| {
        for length in 1..=if unlimited {10000} else {1} {
            let new_x = x as i32 + (off_x * length);
            let new_y = y as i32 + (off_y * length);
            if new_x < 0 || new_x >= width || new_y < 0 || new_y >= height {
                return 0;
            }
            if map[new_y as usize][new_x as usize] == needle {
                return 1;
            }
            if map[new_y as usize][new_x as usize] != '.' {
                break;
            }
        }
        return 0;
    }).sum()
}

fn mutate(map: &Vec<Vec<char>>, mutate_map: &mut Vec<Vec<char>>, deep_sight: bool, leaving_limit: usize) {
    let width = map[0].len();
    let height = map.len();
    for x in 0..width {
        for y in 0..height {
            match map[y][x] {
                'L' => {
                    if adjacent(&map, x, y, '#', deep_sight) == 0 {
                        mutate_map[y][x] = '#';
                    }
                },
                '#' => {
                    if adjacent(&map, x, y, '#', deep_sight) >= leaving_limit {
                        mutate_map[y][x] = 'L';
                    }
                },
                _ => ()
            }
        }
    }
}

fn print(map: &Vec<Vec<char>>) -> String {
    let mut string = String::from("");
    for line in map.iter() {
        string.extend(line);
        string.push('\n');
    }
    string
}

fn main() {
    let data = include_str!("../../../input11");
    let orig_map = data.split("\n").map(|row| row.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut map = orig_map.clone();
    for iteration in 0.. {
        let mut copy = map.clone();
        mutate(&map, &mut copy, false, 4);
        let orig = print(&map);
        let current = print(&copy);
        if orig == current {
            println!("No more mutations after {}", iteration);
            println!("Part1: seats occupied currently: {}", current.chars().filter(|c| *c == '#').count());
            break;
        }

        map = copy;
    }

    map = orig_map.clone();
    for iteration in 0.. {
        let mut copy = map.clone();
        mutate(&map, &mut copy, true, 5);
        let orig = print(&map);
        let current = print(&copy);
        if orig == current {
            println!("No more mutations after {}", iteration);
            println!("Part2: seats occupied currently: {}", current.chars().filter(|c| *c == '#').count());
            break;
        }

        map = copy;
    }
    
}
