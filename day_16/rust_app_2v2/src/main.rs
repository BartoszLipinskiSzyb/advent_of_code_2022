use std::{fs, collections::HashMap};
use itertools::Itertools;

fn walk(current_node: usize, seconds_left: i32, my_nodes: &Vec<&usize>, visited: &mut Vec<bool>, map: &Vec<Vec<&str>>, distances: &Vec<Vec<i32>>, state_memo: &mut HashMap<(usize, Vec<bool>, i32), i32>) -> i32 {
    // println!("Second {}: {:?}", seconds_left, visited);
    if seconds_left <= 1 {
        return 0;
    }

    if !visited.contains(&false) {
        return 0;
    }

    let state = (current_node, visited.clone(), seconds_left);
    if state_memo.contains_key(&state) {
        return *state_memo.get(&state).unwrap();
    }

    let mut max_val = 0;
    let mut result: i32;
    for (next_i, next_node) in my_nodes.iter().enumerate() {
        let time_after = seconds_left - distances[current_node][**next_node] - 1;
        if visited[next_i] || time_after <= 0 {
            continue;
        }

        visited[next_i] = true;

        let next_valve_value = i32::from_str_radix(map[**next_node][1], 10).unwrap() * time_after;
        // println!("{}", next_valve_value);

        result = walk(**next_node, time_after, my_nodes, visited, map, distances, state_memo) + next_valve_value;

        visited[next_i] = false;

        if result > max_val {
            max_val = result;
        }
    }

    state_memo.insert(state, max_val);

    return max_val;
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

    let map: Vec<_> = data_lines.iter().map(|line| line.split_whitespace().collect::<Vec<_>>()).collect();

    let mut indices: HashMap<&str, i32> = HashMap::new();
    for (i, line) in map.iter().enumerate() {
        indices.insert(line[0], i.try_into().unwrap());
    }
                       
    let mut state_memo: HashMap<(usize, Vec<bool>, i32), i32> = HashMap::new();

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
                            // println!("Node {} tries to go to {} on step {}", node[0], map[id][0], step);
                            distances[j][id] = step + 1;
                            visited[j][id] = true;
                        }
                    }
                }
            }
        }
    }

    let mut working_valves: Vec<usize> = vec![];
    for (i, node) in map.iter().enumerate() {
        if node[1] != "0" {
            working_valves.push(i);
        }
    }
    println!("{:?}", working_valves);

    let start_index = *indices.get("aa").unwrap() as usize;
    let mut max_val = 0;
    let mut result: i32;

    let i = 26;
    let work_split = working_valves.len() / 2;
    for perm in working_valves.iter().combinations(work_split) {
        // println!("{:?}", perm);

        let mut my_visited = vec![false; work_split];
        let my_nodes: Vec<&usize> = perm[0..work_split].to_vec();
        state_memo.clear();
        result = walk(start_index, i, &my_nodes, &mut my_visited, &map, &distances, &mut state_memo);

        my_visited = vec![false; working_valves.len() - work_split];
        let mut new_my_nodes: Vec<&usize> = vec![];
        for valve in &working_valves {
            if !my_nodes.contains(&&(*valve as usize)) {
                new_my_nodes.push(&valve);
            }
        }

        //println!("{:?} n {:?} == 0", my_nodes, new_my_nodes);

        state_memo.clear();
        result += walk(start_index, i, &new_my_nodes, &mut my_visited, &map, &distances, &mut state_memo);

        if result > max_val {
            max_val = result;
        }
    }
    println!("{}: {}", i, max_val);
}
