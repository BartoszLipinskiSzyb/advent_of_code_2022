use std::{fs, str::from_utf8};
//            items operator operand test true false
type Monkey = (Vec<i128>, char, i128, i128, i128, i128);

fn update_value(v: i128, operator: char, mut operand: i128) -> i128 {
    let result: i128;

    if operand == -1 {
        operand = v;
    }

    result = match operator {
        '*' => { v * operand as i128 },
        '+' => { v + operand },
        _ => { v }
    };

    return result;
}

fn main() {
    let raw_data = fs::read("input").expect("Should be able to read file");
    let lines: Vec<_> = raw_data.split(|&c| c == b'\n').collect();

    let mut monkeys: Vec<Monkey> = vec![];

    //parse the input
    let mut current_starting_items: Vec<i128> = vec![];
    let mut current_operator: char = ' ';
    let mut current_operand: i128 = -1;
    let mut current_test: i128 = 1;
    let mut current_if_true: i128 = 0;
    let mut current_if_false: i128 = 0;

    let mut counter = 0;
    for line in lines {
        let line_str = from_utf8(line).unwrap();
        
        // starting item
        if counter == 1 {
            current_starting_items.clear();
            let line_splitted: Vec<_> = line_str.split(|c| c == ':').collect();
            let items = line_splitted[1].replace(" ", "");
            current_starting_items = items.split(",").map(|s| i128::from_str_radix(s, 10).expect("Should be base 10 integer")).collect();

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
                    i128::from_str_radix(&operation_str[operation_str.len() - 1].replace(" ", ""), 10).expect("Should be base 10 integer")
                }
            }
        }
        
        // test
        if counter == 3 {
            let line_splitted: Vec<_> = line_str.split(|c| c == ' ').collect();
            current_test = i128::from_str_radix(line_splitted[line_splitted.len() - 1], 10).expect("Should be base 10 integer");
        }

        // if true
        if counter == 4 {
            let line_splitted: Vec<_> = line_str.split(|c| c == ' ').collect();
            current_if_true = i128::from_str_radix(line_splitted[line_splitted.len() - 1], 10).expect("Should be base 10 integer");
        }

        // if false
        if counter == 5 {
            let line_splitted: Vec<_> = line_str.split(|c| c == ' ').collect();
            current_if_false = i128::from_str_radix(line_splitted[line_splitted.len() - 1], 10).expect("Should be base 10 integer");
        
            // add monkey
            monkeys.push((current_starting_items.clone(), current_operator, current_operand, current_test, current_if_true, current_if_false));
        }

        counter += 1;
        counter %= 7;
    }


    let mut divisor = 1;
    for monkey in &monkeys {
        divisor *= monkey.3;
    }

    // let's play
    let mut activity: Vec<i128> = vec![0; monkeys.len()];
    for turn in 0..10000 {
        if turn % 50 == 0 {
            println!("turn: {}", turn);
        }
        for m in 0..monkeys.len() {
            let mut sub = 0;
            for i in 0..monkeys[m as usize].0.len() {
                activity[m as usize] += 1;
                // println!("{:?}[{}]", monkeys[m as usize].0, i);
                let updated_value = update_value(monkeys[m as usize].0[(i - sub) as usize], monkeys[m as usize].1, monkeys[m as usize].2);
                let new_value = updated_value % divisor;
                // println!("Updated value: {}, factors: {:?}", updated_value, new_factors);
                // println!("New value: {}", new_value);

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

    // find 2 most active
    let mut biggest = 0;
    let mut sec_biggest = 0;
    println!("Activity {:?}", activity);

    for act in activity {
        if act >= biggest {
            sec_biggest = biggest;
            biggest = act;
        } else if act > sec_biggest {
            sec_biggest = act;
        }
    }

    let result = biggest as i128 * sec_biggest as i128;

    println!("{}", result);
}
