use std::fs;

fn true_modulo(x: i64, modulo: i64) -> i64 {
    let mut result = x;

    while result < 0 {
        result += modulo;
    }

    return result % modulo;
}

fn print_chamber(chamber: &Vec<Vec<bool>>, start: i64, height_limit: i64, points: &Vec<(i64, i64)>, chamber_height: i64) {
    for y in start..height_limit {
        let y_mod = true_modulo(height_limit - y - 1, chamber_height) as usize;
        for x in 0..7{
            if chamber[y_mod][x]{
                print!("#");    
            } else {
                if points.contains(&(x as i64, y_mod as i64)) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}

fn main() {
    let jet_streams = fs::read_to_string("input").unwrap().replace("\n", "");

    let input_rocks = fs::read_to_string("rocks").unwrap();
    let rock_strings: Vec<&str> = input_rocks.split("\r\n\r\n").collect();
    let rocks: Vec<Vec<&str>> = rock_strings.iter().map(|rock| rock.split("\r\n").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    // let highest_rock_size = rocks.iter().map(|rock| rock.len()).max().unwrap();

    // println!("{:?}", rocks);

    let chamber_width = 7;
    let chamber_height = 512;
    let mut chamber: Vec<Vec<bool>> = vec![vec![false; chamber_width]; chamber_height];
    chamber[chamber_height - 1] = vec![true; chamber_width];

    let mut highest_rock_test = 0;
    let mut last_highest_rock_test;

    // let rock_limit = jet_streams.len() * rocks.len();

    // let test_startback = 10;
    // let mut current_second = rock_limit - rocks.len() * test_startback;
    let mut current_second = 0;

    let mut rock_backup: Vec<(i64, i64)>;
    let mut rock_shape;
    let mut curr_rock: Vec<(i64, i64)>;
    let mut stopped;
    let mut y_mod;
    let mut is_pushing_left;

    let mut movements: Vec<char> = vec![];

    let rock_limit = 10000000;
    for rock_id in 0..rock_limit {
        rock_shape = &rocks[rock_id % rocks.len()];
       
        // spawn rock
        curr_rock = vec![];

        for y in 0..rock_shape.len(){
            y_mod = true_modulo(rock_shape.len() as i64 + highest_rock_test as i64 - y as i64 + 2, chamber_height as i64);
            for x in 0..chamber_width {
                chamber[y_mod as usize][x] = false;
            }
            for x in 0..rock_shape[y].len(){
                if rock_shape[y].chars().nth(x).unwrap() == '#' {
                    // chamber[(rock_shape.len() as i64 + highest_rock as i64 - y as i64 + 2) as usize][x + 2] = '@';

                    curr_rock.push(((x + 2) as i64, y_mod));
                }
            }
        }

        last_highest_rock_test = highest_rock_test;
        highest_rock_test += 3 + rock_shape.len();


        stopped = false;
        while !stopped {
            is_pushing_left = jet_streams.chars().nth(current_second % jet_streams.len()).unwrap() == '<';
            // println!("{}", is_pushing_left);            
            rock_backup = curr_rock.clone();
            if is_pushing_left {
                for point in &mut curr_rock {
                    if point.0 == 0 {
                        curr_rock = rock_backup.clone();
                        break;
                    }
                    if chamber[point.1 as usize % chamber_height][(point.0 - 1) as usize] == false {
                        point.0 -= 1;
                        movements.push('<');
                    } else {
                        // println!("Left block");
                        curr_rock = rock_backup.clone();
                        break;
                    }
                }
            } else {
                for point in &mut curr_rock {
                    if point.0 == chamber_width as i64 - 1 {
                        curr_rock = rock_backup.clone();
                        break;
                    }
                    if chamber[point.1 as usize % chamber_height][(point.0 + 1) as usize] == false {
                        movements.push('>');
                        point.0 += 1;
                    } else {
                        // println!("Left block");
                        curr_rock = rock_backup.clone();
                        break;
                    }
                }
            }

            // print_chamber(&chamber, 20);
            rock_backup = curr_rock.clone();

            for point in &mut curr_rock {
                if chamber[true_modulo(point.1 - 1, chamber_height as i64) as usize][point.0 as usize] == false {
                    movements.push('v');
                    point.1 = true_modulo(point.1 - 1, chamber_height as i64);
                } else {
                    // println!("Left block");
                    curr_rock = rock_backup.clone();
                    stopped = true;
                    break;
                }
            }

            if !stopped && highest_rock_test > last_highest_rock_test {
                highest_rock_test -= 1;
            }
            //println!("{}", highest_rock_test);

            current_second += 1;
        }

        for point in curr_rock {
            chamber[point.1 as usize][point.0 as usize] = true;
        }
    }

    'outer: for cycle_length in 1..rock_limit{
        for i in 0..cycle_length{
            if movements[i] != movements[i + cycle_length] {
                continue 'outer;
            }
        }
        println!("cycle: {}", cycle_length);
    }

    println!("Height: {}", highest_rock_test);
}
