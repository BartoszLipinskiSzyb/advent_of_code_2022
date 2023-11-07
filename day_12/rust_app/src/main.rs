use std::{fs, str::from_utf8, env};

const DIRS: [[i32; 2]; 4] = [
    [1, 0],
    [0, -1],
    [0, 1],
    [-1, 0]
];

fn walk(map: &Vec<String>, curr_height: u8, pos: &Vec<i32>, end_point: &Vec<i32>, distance: i32, distance_map: &mut Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, counter: &mut u128) {

    if pos[0] < 0 || pos[0] >= distance_map[0].len().try_into().unwrap() || pos[1] < 0 || pos[1] >= distance_map.len().try_into().unwrap() {
        return;
    }

    let mut my_height: u8 = map[pos[1] as usize].chars().nth(pos[0] as usize).unwrap() as u8;

    if my_height == 83 {
        my_height = 97;
    }

    if my_height == 69 {
        println!("!!! pos = {:?}, shortest = {}", pos, distance_map[pos[1] as usize][pos[0] as usize]);
        my_height = 122;
    }
    *counter += 1;
    if *counter % 655360 == 0 {
        *counter = 0;
        // println!("pos = {:?}, height = {}, curr_height = {}", pos, my_height, curr_height);
    }
    let diff = (curr_height as i16) - (my_height as i16);
    if diff < -1 || diff >= 1 {
        // println!("  {} != {}", curr_height, my_height);
        return;
    }

    if visited[pos[1] as usize][pos[0] as usize] {
        return;
    }

    visited[pos[1] as usize][pos[0] as usize] = true;

    if distance_map[pos[1] as usize][pos[0] as usize] > distance {
        distance_map[pos[1] as usize][pos[0] as usize] = distance;
    }

    for dir in DIRS {
        let new_pos = vec![pos[0] + dir[0], pos[1] + dir[1]];
        walk(map, my_height, &new_pos, end_point, distance + 1, distance_map, visited, counter);
        drop(new_pos);
    }
    
    visited[pos[1] as usize][pos[0] as usize] = false;

    return;
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let raw_data =  fs::read("test").expect("Error reading input file");
    let height_lines: Vec<_> = from_utf8(&raw_data).expect("Should be utf-8")
        .split_whitespace()
        .map(|line| line.replace(" ", ""))
        .collect();

    // println!("{:?}", height_lines);

    let mut shortest_distance_map: Vec<Vec<i32>> = vec![vec![9999999; height_lines[0].len()]; height_lines.len()];

    // println!("{:?}", shortest_distance_map);

    let mut start_point: Vec<i32> = vec![];
    let mut end_point: Vec<i32> = vec![];

    for (y, line) in height_lines.iter().enumerate() {
        for (x, point) in line.chars().enumerate() {
            if point == 'S' {
                start_point = vec![x.try_into().unwrap(), y.try_into().unwrap()];
            }
            if point == 'E' {
                end_point = vec![x.try_into().unwrap(), y.try_into().unwrap()];
            }
        }
    }

    println!("S = {:?}, E = {:?}", start_point, end_point);

    let mut visited: Vec<Vec<bool>> = vec![vec![false; height_lines[0].len()]; height_lines.len()];
    let mut debug_couter: u128 = 0;
    walk(&height_lines, 'a' as u8, &start_point, &end_point, 0, &mut shortest_distance_map, &mut visited, &mut debug_couter);

    for line in &shortest_distance_map {
        println!("{:?}", line);
    }

    // println!("{:?}", shortest_distance_map);

    println!("\n\n{}", shortest_distance_map[end_point[1] as usize][end_point[0] as usize]);
}
