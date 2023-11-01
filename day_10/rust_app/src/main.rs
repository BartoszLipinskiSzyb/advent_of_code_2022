use std::{fs, str::from_utf8};

const CYCLES_TO_CHECK: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn main() {
    let raw_input = fs::read("input").expect("Should be able to read the file");
    
    let lines: Vec<&str> = raw_input.split(|&c| c == b'\n').map(|utf| from_utf8(utf).unwrap()).collect();
    
    let mut cycle = 1;
    let mut regx = 1;

    let mut result = 0;

    for line in lines {
        println!("Regx: {} Cycle: {}", regx, cycle);
        if line.len() == 0 {
            continue;
        }
        let splitted: Vec<&str> = line.split(|c| c == ' ').collect();

        if CYCLES_TO_CHECK.contains(&cycle) {
            result += regx * cycle;
        }
        
        cycle += 1;

        if splitted[0] == "addx" {

            if CYCLES_TO_CHECK.contains(&cycle) {
                result += regx * cycle;
            }

            cycle += 1;
            regx += i32::from_str_radix(splitted[1], 10).expect("Parameter should be base 10 number");
        }
    }

    println!("\n\n{}", result);
}
