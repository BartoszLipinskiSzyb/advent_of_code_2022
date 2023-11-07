use std::{fs, process};

fn manhattan_distance(a: &Vec<i128>, b: &Vec<i128>) -> i128 {
    return (a[0] - b[0]).abs() + (a[1] - b[1]).abs();
}

fn is_in_range(x: i128, y: i128, sensors_range: &Vec<(Vec<i128>, i128)>) -> bool {
    let pos = vec![x, y];
    for sensor in sensors_range {
        if manhattan_distance(&pos, &sensor.0) <= sensor.1 {
            return true;
        }
    }
    return false;
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut sensors_range: Vec<(Vec<i128>, i128)> = vec![];
    let mut beacons: Vec<Vec<i128>> = vec![];

    for line in input.lines() {
        let value_line = &line.replace("Sensor at ", "").replace(": closest beacon is at ", ":").replace("x=", "").replace("y=", "");
        let splitted_line: Vec<_> = value_line.split(":").collect();
        let mut p: Vec<Vec<i128>> = vec![vec![0, 0]; 2];
        for (i, point) in splitted_line.iter().enumerate() {
            p[i] = point.split(", ").map(|val| i128::from_str_radix(val, 10).unwrap()).collect();
        }
        println!("Distance from {:?} to {:?} = {}", p[0], p[1], manhattan_distance(&p[0], &p[1]));
        sensors_range.push((vec![p[0][0], p[0][1]], manhattan_distance(&p[0], &p[1])));
        let beacon = vec![p[1][0], p[1][1]];
        if !beacons.contains(&beacon){
            beacons.push(beacon);
        }
    }
    println!("beacon amount: {}", beacons.len());

    for sensor in &sensors_range {
        // iterate over border of sensor range - start with left side
        let pos: Vec<i128> = vec![sensor.0[0] - sensor.1 - 1, sensor.0[1]];
        let mut ix = 0;
        let mut iy = 0;

        while pos[0] + ix <= sensor.0[0] + sensor.1 + 1 {
            if !is_in_range(pos[0] + ix, pos[1] + iy, &sensors_range){
                if pos[0] + ix >= 0 && pos[0] + ix <= 4000000 && pos[1] + iy >= 0 && pos[1] + iy <= 4000000{
                    println!("{} {} {}", pos[0] + ix, pos[1] + iy, (pos[0]+ix)*4000000+(pos[1]+iy));
                    process::exit(0);
                }
            }

            if !is_in_range(pos[0] + ix, pos[1] - iy, &sensors_range){
                if pos[0] + ix >= 0 && pos[0] + ix <= 4000000 && pos[1] - iy >= 0 && pos[1] - iy <= 4000000{
                    println!("{} {} {}", pos[0] + ix, pos[1] - iy, (pos[0]+ix)*4000000+(pos[1]-iy));
                    process::exit(0);
                }
            }

            ix += 1;
            if pos[0] + ix >= sensor.0[0]{
                iy -= 1;
            } else {
                iy += 1;
            }
        }   
    }
}
