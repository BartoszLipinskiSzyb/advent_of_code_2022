use std::{fs, str::from_utf8, env, io::stdin};

const DIRS: [[i32; 2]; 4] = [
    [1, 0],
    [0, -1],
    [0, 1],
    [-1, 0]
];

fn walk(map: &Vec<String>, dist_map: &mut Vec<Vec<i32>>, pos: &mut Vec<i32>, end: &Vec<i32>, step: i32) -> bool{
//    for (y, line) in dist_map.iter().enumerate() {
//        for (x, val) in line.iter().enumerate() {
//            if *val != -1 {
//                print!(".");
//            } else {
//                print!("{}", map[y].chars().nth(x).unwrap());
//            }
//        }
//        println!();
//    }

//    // czekanie na enter
//    let mut buffer = String::new();
//    stdin().read_line(&mut buffer).unwrap();

    
    //if pos[0] == end[0] && pos[1] == end[1]{
    //    println!("!!! pos = {:?}, shortest = {}", pos, step);
    //    return true;
    //}

    
    let mut curr_height: u8 = map[pos[1] as usize].chars().nth(pos[0] as usize).unwrap() as u8;
    if curr_height == 83 {
        curr_height = 97;
    }

    if curr_height == 69 {
        curr_height = 122;
    }

    for dir in DIRS {

        pos[0] += dir[0];
        pos[1] += dir[1];

        
        // czy w granicach
        if pos[0] < 0 || pos[0] >= map[0].len().try_into().unwrap() || pos[1] < 0 || pos[1] >= map.len().try_into().unwrap() {
            println!("Sprawdzanie {:?} poza granicami", pos);

            pos[0] -= dir[0];
            pos[1] -= dir[1];
            continue;
        }

        // jeśli odwiedzono
        if dist_map[pos[1] as usize][pos[0] as usize] != -1 {
            println!("Sprawdzanie {:?} odwiedzono", pos);

            pos[0] -= dir[0];
            pos[1] -= dir[1];
            continue;
        }

        let mut my_height: u8 = map[pos[1] as usize].chars().nth(pos[0] as usize).unwrap() as u8;

        // przypadki S i E
        if my_height == 83 {
            my_height = 97;
        }

        if my_height == 69 {
            my_height = 122;
        }

        let diff = (curr_height as i16) - (my_height as i16);
        if diff > 1 {
            println!("Sprawdzanie {:?} za wysoko", pos);

            pos[0] -= dir[0];
            pos[1] -= dir[1];
            continue;
        }

        dist_map[pos[1] as usize][pos[0] as usize] = step + 1;
        println!("Sprawdzanie {:?} ok", pos);


        pos[0] -= dir[0];
        pos[1] -= dir[1];
    }

    return false;
}

fn mark(dist_map: &mut Vec<Vec<i32>>, map: &Vec<String>, step: i32, end: &Vec<i32>) -> bool {
    for (y, line) in dist_map.clone().iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val == step {
                let mut pos: Vec<i32> = vec![x.try_into().unwrap(), y.try_into().unwrap()];
                if walk(map, dist_map, &mut pos, end, step) {
                    return true;
                }
            }
        }
    }

    return false;
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
    distance_map[end_point[1] as usize][end_point[0] as usize] = 0;
    
    for step in 0..10000 {
        println!("step = {}", step);
        mark(&mut distance_map, &height_lines, step, &end_point);
    }
    

    let mut min = 99999999;
    for (y, line) in distance_map.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val != -1 {
                if height_lines[y].chars().nth(x).unwrap() == 'a' && val < &min {
                    min = *val;
                }
                print!(".");
            } else {
                print!("{}", height_lines[y].chars().nth(x).unwrap());
            }
        }
        println!();
    }

    println!("min = {}", min);
}
