use std::collections::{HashSet};

type World = HashSet<(i32, i32, i32, i32)>;

fn active_neighbours(world: &World, x: i32, y: i32, z: i32, w: i32, four_d: bool) -> usize {
    let mut w_min = 0;
    let mut w_to = 0;
    if four_d {
        w_min = -1;
        w_to = 1;
    }
    let mut count = 0;
    for check_x in -1i32..=1i32 {
        for check_y in -1i32..=1i32 {
            for check_z in -1i32..=1i32 {
                for check_w in w_min..=w_to {
                    if check_x == 0 && check_y == 0 && check_z == 0 && check_w == 0 {
                        continue;
                    }
                    if world.contains(&(x + check_x, y + check_y, z + check_z, w + check_w)) {
                        count = count + 1;
                    }
                }
            }
        }
    }
    return count;
}

fn mutate(world: &World, four_d: bool) -> World {
    let mut new_world = world.clone();
    let mut w_min = 0;
    let mut w_to = 0;
    if four_d {
        w_min = -1;
        w_to = 1;
    }

    let mut checked = World::new();
    for active_pos in world.iter() {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                        for w in w_min..=w_to {
                        let pos = (active_pos.0 + x, active_pos.1 + y, active_pos.2 + z, active_pos.3 + w);
                        if checked.contains(&pos) {
                            continue;
                        } else {
                            checked.insert(pos);
                        }
                        // println!("Checking pos: {:?}", pos);
                        let n = active_neighbours(&world, pos.0, pos.1, pos.2, pos.3, four_d);
                        let active = world.contains(&pos);
                        if active && !(n == 2 || n == 3) {
                            new_world.remove(&pos);
                        } else if !active && n == 3 {
                            new_world.insert(pos);
                        }
                    }
                }
            }
        }
    }
    
    new_world
}

fn main() {
    let input = include_str!("../../../input17");
    let data = input.split("\n").collect::<Vec<&str>>();

    let mut world = World::new();
    data.iter().enumerate().for_each(|(y, line)| line.chars().enumerate().for_each(|(x, c)| if c == '#' { world.insert((x as i32, y as i32, 0, 0)); } ));
    let original_world = world.clone();

    for _ in 0..6 {
        world = mutate(&world, false);
    }
    println!("Part1: cubes in 3d world: {}", world.len());

    world = original_world.clone();
    for _ in 0..6 {
        world = mutate(&world, true);
    }
    println!("Part2: cubes in 4d world: {}", world.len());
}
