use std::{fs, collections::HashMap, mem::size_of_val};

fn walk(current_valve: &str, seconds_left: i32, indices: &HashMap<&str, i32>, map: &Vec<Vec<&str>>, initialization: bool, is_opened: &mut Vec<bool>, path: &mut Vec<i32>, state_memo: &mut HashMap<(i32, Vec<bool>, i32), i32>) -> i32 {
    //println!("{}", seconds_left);
    if seconds_left <= 1 {
        return 0;
    }

    let my_id = *indices.get(current_valve).unwrap() as usize;

    let state = &(my_id as i32, is_opened.to_owned(), seconds_left);
    if state_memo.contains_key(state) {
        return *state_memo.get(state).unwrap();
    }

    let data_line = &map[my_id];

    let mut max_value = -1;
    let mut result: i32;
    for i in 2..data_line.len() {
        //path.push(my_id.try_into().unwrap());
        //if path.len() >= 15{
        //    println!("{:?}", path);
        //}
        if !is_opened[my_id] && data_line[1] != "0"{
            is_opened[my_id] = true;
            let my_opening_value = (i32::from_str_radix(data_line[1], 10).unwrap() * (seconds_left - 1)).max(0);
            result = walk(data_line[i as usize], seconds_left - 2, indices, map, initialization, is_opened, path, state_memo) + my_opening_value;
            if result > max_value{
                max_value = result;
            }
            is_opened[my_id] = false;
        }
        result = walk(data_line[i as usize], seconds_left - 1, indices, map, initialization, is_opened, path, state_memo);
        if result > max_value{
            max_value = result;
        }
        //path.pop();
    }

    state_memo.insert(state.clone(), max_value);

    return max_value;
}

fn main() {
    let input_data = fs::read_to_string("input").unwrap();
    let data_lines: Vec<String> = input_data.lines().map(|line| line.to_lowercase()
            .replace("valves ", "")
            .replace("valve ", "")
            .replace("has flow rate=", "")
            .replace("; tunnels lead to", "")
            .replace("; tunnel leads to", "")
            .replace(",", "")).collect();
    //println!("{:?}", data_lines);

    let map: Vec<_> = data_lines.iter().map(|line| line.split_whitespace().collect::<Vec<_>>()).collect();
    // println!("{:?}", map);

    let mut indices: HashMap<&str, i32> = HashMap::new();
    for (i, line) in map.iter().enumerate() {
        indices.insert(line[0], i.try_into().unwrap());
    }
                        //   curr opened     secs value
    let mut state_memo: HashMap<(i32, Vec<bool>, i32), i32> = HashMap::new();
    let mut is_opened: Vec<bool> = vec![false; map.len()];
    let mut path: Vec<i32> = vec![];

    // println!("{:?}", indices);

    println!("{}", walk(map[*indices.get("aa").unwrap() as usize][0], 30, &indices, &map, false, &mut is_opened, &mut path, &mut state_memo));
    println!("states saved: {}", state_memo.len());
    println!("states size: {}", size_of_val(&state_memo));
}
