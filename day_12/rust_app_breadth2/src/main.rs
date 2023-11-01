use std::{fs, str::from_utf8, env, io::stdin};
use std::process::Command;

const DIRS: [[i32; 2]; 4] = [
    [1, 0],
    [0, -1],
    [0, 1],
    [-1, 0]
];

fn walk(map: &Vec<String>, pos: &mut Vec<i32>, visited: &mut Vec<Vec<bool>>, end: &Vec<i32>, distance: i32, curr_height: u8, distance_map: &mut Vec<Vec<i32>>){
    if pos[0] < 0 || pos[0] >= map[0].len().try_into().unwrap() || pos[1] < 0 || pos[1] >= map.len().try_into().unwrap() {
        return;
    }

    if visited[pos[1] as usize][pos[0] as usize] {
        // println!("visited!");
        return;
    }

    let mut my_height: u8 = map[pos[1] as usize].chars().nth(pos[0] as usize).unwrap() as u8;

    if my_height == 83 {
        my_height = 97;
    }

    if my_height == 69 {
        my_height = 122;
    }

    let diff = (curr_height as i16) - (my_height as i16);
    if diff < -1 {
        // // println!("  {} != {}", curr_height, my_height);
        return;
    }

    for (y, line) in visited.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val {
                print!(".");
            } else {
                print!("{}", map[y].chars().nth(x).unwrap());
            }
        }
        println!();
    }

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    //let mut child = Command::new("sleep").arg("0.01").spawn().unwrap();
    //let _result = child.wait().unwrap();

    // println!("pos = {:?}", pos);

    
    if pos[0] == end[0] && pos[1] == end[1]{
        // println!("!!! pos = {:?}, shortest = {}", pos, distance);
    }

    visited[pos[1] as usize][pos[0] as usize] = true;
    distance_map[pos[1] as usize][pos[0] as usize] = distance;
    
    for dir in DIRS {
        pos[0] += dir[0];
        pos[1] += dir[1];

        let new_distance = distance + 1;
        walk(map, pos, visited, end, new_distance, my_height, distance_map);

        pos[0] -= dir[0];
        pos[1] -= dir[1];
    }

    return;
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let raw_data =  fs::read("input").expect("Error reading input file");
    let height_lines: Vec<_> = from_utf8(&raw_data).expect("Should be utf-8")
        .split_whitespace()
        .map(|line| line.replace(" ", ""))
        .collect();

    // // println!("{:?}", height_lines);


    // // println!("{:?}", shortest_distance_map);

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

    let mut distance_map: Vec<Vec<i32>> = vec![vec![-1; height_lines[0].len()]; height_lines.len()];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; height_lines[0].len()]; height_lines.len()];
    
    walk(&height_lines, &mut start_point, &mut visited, &end_point, 0.clone(), 'a' as u8, &mut distance_map);

    for line in distance_map {
        for val in line {
            // print!("{} ", val);
        }
        // println!();
    }
}
