use std::{fs, collections::HashMap, mem::size_of_val};

fn walk(current_valve: i32, eleph_valve: i32, seconds_left: i32, indices: &HashMap<&str, i32>, map: &Vec<Vec<&str>>, is_opened: &mut Vec<bool>, state_memo: &mut HashMap<(i32, i32, Vec<bool>, i32), i32>, distances: &Vec<Vec<i32>>, mut my_penalty: i32, mut el_penalty: i32) -> i32 {
    if seconds_left <= 1 {
        return 0;
    }

    let state = (current_valve, eleph_valve, is_opened.clone(), seconds_left);
    if state_memo.contains_key(&state){
        return *state_memo.get(&state).unwrap();
    }

    let mut some_left_to_open = false;
    for (i, opened) in is_opened.iter().enumerate() {
        if !opened && map[i][1] != "0" {
            some_left_to_open = true;
            break;
        }
    }
    if !some_left_to_open {
        return 0;
    }

    //let my_id = *indices.get(current_valve).unwrap();
    //let el_id = *indices.get(eleph_valve).unwrap();

    // println!("opened: {:?}", is_opened);
    // println!("flowrates {:?}", map.iter().map(|elem| elem[1]).collect::<Vec<_>>());

    let mut max_val = 0;
    let mut result: i32;
    for mut my_step_id in 1..distances[0].len() {
        let mut my_remember_to_close: bool = false;
        let my_node = &map[my_step_id as usize];
        if my_penalty != 0 {
            my_step_id = current_valve as usize;
            if !is_opened[my_step_id] {
                my_remember_to_close = true;
                is_opened[my_step_id] = true;
                //println!("me opening {}", my_step_id);
            }
        }
        else if is_opened[my_step_id] || my_node[1] == "0" {
            continue;
        }
        for mut el_step_id in 1..distances[0].len() {
            let mut el_remember_to_close = false;
            let el_node = &map[el_step_id as usize];
            if el_penalty != 0 {
                el_step_id = eleph_valve as usize;
                if !is_opened[el_step_id]{
                    el_remember_to_close = true;
                    is_opened[el_step_id] = true;
                    //println!("el opening {}", el_step_id);
                }
            }
            else if is_opened[el_step_id] || el_node[1] == "0" {
                continue;
            }

            let my_value: i32;
            let el_value: i32;

            my_penalty = if my_penalty == 0 { 
                let pen = distances[current_valve as usize][my_step_id];
                my_value = (seconds_left - pen).max(0) * i32::from_str_radix(my_node[1], 10).unwrap();
                pen
            } else {
                my_value = 0;
                (my_penalty - 1).max(0)
            };

            el_penalty = if el_penalty == 0 { 
                let pen = distances[eleph_valve as usize][el_step_id];
                el_value = (seconds_left - pen).max(0) * i32::from_str_radix(el_node[1], 10).unwrap();
                pen
            } else { 
                el_value = 0;
                (el_penalty - 1).max(0) 
            };

            result = walk(my_step_id as i32, el_step_id as i32, seconds_left - 1, indices, map, is_opened, state_memo, distances, my_penalty, el_penalty) + my_value + el_value;
            
            if el_remember_to_close {
                is_opened[el_step_id] = false;
            }

            if result > max_val {
                max_val = result;
            }
        }
        if my_remember_to_close{
            is_opened[my_step_id] = false;
        }
    }

    state_memo.insert(state, max_val);

    return max_val;
}

fn main() {
    let input_data = fs::read_to_string("test").unwrap();
    let data_lines: Vec<String> = input_data.lines().map(|line| line.to_lowercase()
            .replace("valves ", "")
            .replace("valve ", "")
            .replace("has flow rate=", "")
            .replace("; tunnels lead to", "")
            .replace("; tunnel leads to", "")
            .replace(",", "")).collect();

    let map: Vec<_> = data_lines.iter().map(|line| line.split_whitespace().collect::<Vec<_>>()).collect();

    let mut indices: HashMap<&str, i32> = HashMap::new();
    for (i, line) in map.iter().enumerate() {
        indices.insert(line[0], i.try_into().unwrap());
    }
                        //       curr eleph opened   secs value
    let mut state_memo: HashMap<(i32, i32, Vec<bool>, i32), i32> = HashMap::new();
    let mut is_opened: Vec<bool> = vec![false; map.len()];


    let mut distances: Vec<Vec<i32>> = vec![vec![1024; map.len()]; map.len()];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map.len()]; map.len()];

    for (j, _start_node) in map.iter().enumerate(){
        distances[j][j] = 0;
        visited[j][j] = true;
        for step in 0..50{
            for (i, node) in map.iter().enumerate() {
                if distances[j][i] == step {
                    for neigh in 2..node.len() {
                        let id = *indices.get(node[neigh]).unwrap() as usize;
                        if !visited[j][id]{
                            println!("Node {} tries to go to {} on step {}", node[0], map[id][0], step);
                            distances[j][id] = step + 1;
                            visited[j][id] = true;
                        }
                    }
                }
            }
        }
    }

    //for i in 0..=26{
    //    println!("{}: {}", i, walk(*indices.get("aa").unwrap(), *indices.get("aa").unwrap(), i, &indices, &map, &mut is_opened, &mut state_memo));
    //}
    println!("{}", walk(*indices.get("aa").unwrap(), *indices.get("aa").unwrap(), 26, &indices, &map, &mut is_opened, &mut state_memo, &distances, 0, 0));
    println!("states saved: {}", state_memo.len());
    println!("states size: {}", size_of_val(&state_memo));
}
