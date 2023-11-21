use std::{fs, collections::HashMap};

fn main() {
    let input_data = fs::read_to_string("test").unwrap();
    let data_lines: Vec<String> = input_data.lines().map(|line| line.to_lowercase()
            .replace("valves ", "")
            .replace("valve ", "")
            .replace("has flow rate=", "")
            .replace("; tunnels lead to", "")
            .replace("; tunnel leads to", "")
            .replace(",", "")).collect();
    //println!("{:?}", data_lines);

    let map: Vec<_> = data_lines.iter().map(|line| line.split_whitespace().collect::<Vec<_>>()).collect();
    //println!("{:?}", map);

    let mut indices: HashMap<&str, i32> = HashMap::new();
    for (i, line) in map.iter().enumerate() {
        indices.insert(line[0], i.try_into().unwrap());
    }

    let mut is_opened: Vec<bool> = vec![false; map.len()];

    // TODO: zaimplementować breadth first search, tak aby dystans od każdego do każdego node'a był znany
    let mut distances: Vec<Vec<i32>> = vec![vec![1024; map.len()]; map.len()];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map.len()]; map.len()];

    for (j, start_node) in map.iter().enumerate(){
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
    println!("{:?}", indices);
    println!("Distances: {:?}", distances);

    let mut sum = 0;
    let mut seconds_left = 30;
    let mut current_node = *indices.get("aa").unwrap() as usize;
    let mut next_node: usize = 0;
    while seconds_left > 0 {
        let mut max_reward = -1;
        for (i, valve) in map.iter().enumerate(){
            if !is_opened[i] {
                let result = (seconds_left - distances[current_node][i] - 1) * i32::from_str_radix(valve[1], 10).unwrap();
                println!("At seconds_left={}, valve {} has reward {} with flow rate={}", seconds_left, valve[0], result, valve[1]);
                if result > max_reward {
                    max_reward = result;
                    next_node = i as usize;
                }
            }
        }
        println!("Opening valve {} with p/m={}", map[next_node][0], map[next_node][1]);
        sum += max_reward;
        is_opened[next_node] = true;
        seconds_left -= distances[current_node][next_node] + 1;
        current_node = next_node;
    }

    println!("result = {}", sum);
}
