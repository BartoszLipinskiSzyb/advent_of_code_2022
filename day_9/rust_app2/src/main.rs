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

    let mut rope_elements: Vec<Vec<i32>> = vec![vec![0, 0]; 10];

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
            rope_elements[0] = update_position(&directions, rope_elements[0].clone(), direction);

            for element in 0..9 {
                if !is_next_to_other(&rope_elements[element], &rope_elements[element + 1]) {
                    let mut iteration = 0;
                    for d in &directions {
                        let candidate_next_position_of_tail = update_position(&directions, rope_elements[element].clone(), d.0.chars().next().unwrap());
                        //println!("candidate: {:?}", candidate_next_position_of_tail);
                        if is_next_to_other(&candidate_next_position_of_tail, &rope_elements[element + 1]) {
                            rope_elements[element + 1] = candidate_next_position_of_tail;
                            break;
                        }
                        iteration += 1;

                        if iteration == 4 {
                            for d in vec![vec![1, 1], vec![1, -1], vec![-1, 1], vec![-1, -1]].iter() {
                                let candidate = vec![rope_elements[element][0] + d[0], rope_elements[element][1] + d[1]];
                                if is_next_to_other(&candidate, &rope_elements[element + 1]){
                                    rope_elements[element + 1] = candidate;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            
            if !visited_by_tail.contains(&rope_elements[9]) {
                visited_by_tail.push(rope_elements[9].clone());
            }
            
            //println!("{:?}", rope_elements);

            //println!("{:?} {:?}", head_position, tail_position);
        }
    }

    println!("{:?}", visited_by_tail.len());
}
