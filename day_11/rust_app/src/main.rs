use std::{fs, str::from_utf8};
//            items operator operand test true false
type Monkey = (Vec<i32>, char, i32, i32, i32, i32);

fn update_value(v: i32, operator: char, mut operand: i32) -> i32 {
    let result: i32;

    if operand == -1 {
        operand = v;
    }

    result = match operator {
        '*' => { v * operand },
        '+' => { v + operand },
        _ => { v }
    };

    return result / 3;
}

fn main() {
    let raw_data = fs::read("test").expect("Should be able to read file");
    let lines: Vec<_> = raw_data.split(|&c| c == b'\n').collect();

    let mut monkeys: Vec<Monkey> = vec![];

    //parse the input
    let mut current_starting_items: Vec<i32> = vec![];
    let mut current_operator: char = ' ';
    let mut current_operand: i32 = -1;
    let mut current_test: i32 = 1;
    let mut current_if_true: i32 = 0;
    let mut current_if_false: i32 = 0;

    let mut counter = 0;
    for line in lines {
        let line_str = from_utf8(line).unwrap();
        
        // starting item
        if counter == 1 {
            current_starting_items.clear();
            let line_splitted: Vec<_> = line_str.split(|c| c == ':').collect();
            let items = line_splitted[1].replace(" ", "");
            current_starting_items = items.split(",").map(|s| i32::from_str_radix(s, 10).expect("Should be base 10 integer")).collect();

            // println!("{:?}", starting_items);
        }

        // operation
        if counter == 2 {
            let operation_str: Vec<_> = line_str.split(|c| c == ' ').collect();
            // println!("{:?}", operation_str);
            current_operator = operation_str[operation_str.len() - 2].chars().next().unwrap();
            current_operand = {
                if operation_str[operation_str.len() - 1] == "old"{
                    -1
                } else {
                    i32::from_str_radix(&operation_str[operation_str.len() - 1].replace(" ", ""), 10).expect("Should be base 10 integer")
                }
            }
        }
        
        // test
        if counter == 3 {
            let line_splitted: Vec<_> = line_str.split(|c| c == ' ').collect();
            current_test = i32::from_str_radix(line_splitted[line_splitted.len() - 1], 10).expect("Should be base 10 integer");
        }

        // if true
        if counter == 4 {
            let line_splitted: Vec<_> = line_str.split(|c| c == ' ').collect();
            current_if_true = i32::from_str_radix(line_splitted[line_splitted.len() - 1], 10).expect("Should be base 10 integer");
        }

        // if false
        if counter == 5 {
            let line_splitted: Vec<_> = line_str.split(|c| c == ' ').collect();
            current_if_false = i32::from_str_radix(line_splitted[line_splitted.len() - 1], 10).expect("Should be base 10 integer");
        
            // add monkey
            monkeys.push((current_starting_items.clone(), current_operator, current_operand, current_test, current_if_true, current_if_false));
        }

        counter += 1;
        counter %= 7;
    }


    // let's play
    let mut activity: Vec<i32> = vec![0; monkeys.len()];
    for _turn in 0..20 {
        for m in 0..monkeys.len() {
            let mut sub = 0;
            for i in 0..monkeys[m as usize].0.len() {
                activity[m as usize] += 1;
                // println!("{:?}[{}]", monkeys[m as usize].0, i);
                let new_value = update_value(monkeys[m as usize].0[(i - sub) as usize], monkeys[m as usize].1, monkeys[m as usize].2);
                if new_value % monkeys[m as usize].3 == 0 {
                    let m_val = m as usize;
                    let to_m = monkeys[m_val].4 as usize;
                    monkeys[to_m].0.push(new_value);
                } else {
                    let m_val = m as usize;
                    let to_m = monkeys[m_val].5 as usize;
                    monkeys[to_m].0.push(new_value);
                }
                monkeys[m].0.remove(i - sub);
                sub += 1;
            }
        }
    }

    let mut biggest = 0;
    let mut sec_biggest = 0;

    for act in activity {
        if act >= biggest {
            sec_biggest = biggest;
            biggest = act;
        } else if act > sec_biggest {
            sec_biggest = act;
        }
    }
    println!("{}", biggest * sec_biggest)
}
