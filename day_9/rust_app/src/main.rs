use std::{fs, collections::HashMap};

fn update_position(directions: &HashMap<&str, [i32; 2]>, mut position: Vec<i32>, direction: char) -> Vec<i32>{
    for i in 0..2 {
        position[i] += directions.get(&String::from(direction) as &str).unwrap()[i];
    }
    return position.to_vec();
}

fn is_next_to_other(a: &Vec<i32>, b: &Vec<i32>) -> bool{
    for x in -1..=1 {
        for y in -1..=1 {
            if a[0] + x == b[0] && a[1] + y == b[1] {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let raw_data = fs::read("input").expect("Should be able to read file");
    let input_lines: Vec<_> = raw_data.split(|&c| c == b'\n').collect();
    
    let directions: HashMap<&str, [i32; 2]> = HashMap::from([
        ("U", [0, 1]),
        ("R", [1, 0]),
        ("D", [0, -1]),
        ("L", [-1, 0]),
    ]);


    let mut head_position = vec![0, 0];
    let mut tail_position = vec![0, 0];
    let mut visited_by_tail: Vec<Vec<i32>> = vec![vec![0, 0]];


    for line in input_lines {
        if line.len() == 0 {
            continue;
        }

        let number_of_repetitions: u8;
        let direction_num = line[0];
        let direction = char::from_u32(direction_num as u32).unwrap();

        if line.len() == 3{
            number_of_repetitions = line[2] - ('0' as u8);
        } else {
            number_of_repetitions = (line[3] - ('0' as u8)) + (line[2] - ('0' as u8)) * 10;
        }

        for _ in 0..number_of_repetitions {
            head_position = update_position(&directions, head_position, direction);

            if !is_next_to_other(&head_position, &tail_position) {
                for d in &directions {
                    let candidate_next_position_of_tail = update_position(&directions, head_position.clone(), d.0.chars().next().unwrap());
                    //println!("candidate: {:?}", candidate_next_position_of_tail);
                    if is_next_to_other(&candidate_next_position_of_tail, &tail_position) {
                        tail_position = candidate_next_position_of_tail;
                        if !visited_by_tail.contains(&tail_position) {
                            visited_by_tail.push(tail_position.clone());
                        }
                        break;
                    }

                }
            }

            //println!("{:?} {:?}", head_position, tail_position);
        }
    }

    println!("{:?}", visited_by_tail.len());
}
