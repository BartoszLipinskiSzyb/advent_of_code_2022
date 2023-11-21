use std::{fs, io};

fn print_chamber(chamber: &Vec<Vec<char>>, height_limit: i32) {
    for y in 0..height_limit {
        println!("{:?}", chamber[(height_limit - y - 1) as usize])
    }
}

fn main() {
    let jet_streams = fs::read_to_string("input").unwrap().replace("\n", "");

    let input_rocks = fs::read_to_string("rocks").unwrap();
    let rock_strings: Vec<&str> = input_rocks.split("\r\n\r\n").collect();
    let rocks: Vec<Vec<&str>> = rock_strings.iter().map(|rock| rock.split("\r\n").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    // println!("{:?}", rocks);

    let chamber_width = 7;
    let mut chamber: Vec<Vec<char>> = vec![vec![' '; chamber_width]; 4096];
    let mut highest_rock = 0;
    let mut current_second = 0;

    let mut rock_backup: Vec<(i32, i32)>;

    for rock_id in 0..2022 {
        let rock_shape = &rocks[rock_id % rocks.len()];
        
        for (i, layer) in chamber.iter().enumerate() {
            if !layer.contains(&'#') {
                highest_rock = i;
                // println!("{}", highest_rock);
                break;
            }
        }

        // spawn rock
        let mut curr_rock: Vec<(i32, i32)> = vec![];

        for y in 0..rock_shape.len(){
            for x in 0..rock_shape[y].len(){
                if rock_shape[y].chars().nth(x).unwrap() == '#' {
                    // chamber[(rock_shape.len() as i32 + highest_rock as i32 - y as i32 + 2) as usize][x + 2] = '@';

                    curr_rock.push(((x + 2) as i32, (rock_shape.len() as i32 + highest_rock as i32 - y as i32 + 2)));
                }
            }
        }

        let mut stopped = false;
        while !stopped {
            let is_pushing_left: bool = jet_streams.chars().nth(current_second % jet_streams.len()).unwrap() == '<';
            // println!("{}", is_pushing_left);            
            rock_backup = curr_rock.clone();
            if is_pushing_left {
                for point in &mut curr_rock {
                    if point.0 == 0 {
                        curr_rock = rock_backup.clone();
                        break;
                    }
                    if chamber[point.1 as usize][(point.0 - 1) as usize] == ' ' {
                        point.0 -= 1;
                    } else {
                        // println!("Left block");
                        curr_rock = rock_backup.clone();
                        break;
                    }
                }
            } else {
                for point in &mut curr_rock {
                    if point.0 == 6 {
                        curr_rock = rock_backup.clone();
                        break;
                    }
                    if chamber[point.1 as usize][(point.0 + 1) as usize] == ' ' {
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
                if point.1 == 0 {
                    curr_rock = rock_backup.clone();
                    stopped = true;
                    break;
                }
                if chamber[(point.1 - 1) as usize][point.0 as usize] == ' ' {
                    point.1 -= 1;
                } else {
                    // println!("Left block");
                    curr_rock = rock_backup.clone();
                    stopped = true;
                    break;
                }
            }
            
            // let mut gorol = String::new();
            // io::stdin().read_line(&mut gorol).unwrap();
            // print_chamber(&chamber, 20);
            current_second += 1;
        }

        for point in curr_rock {
            chamber[point.1 as usize][point.0 as usize] = '#';
        }
    }


    for (i, layer) in chamber.iter().enumerate() {
        if !layer.contains(&'#') {
            highest_rock = i;
            println!("{}", highest_rock);
            break;
        }
    }
    println!("Height: {}", highest_rock);
}
