use std::{fs, str::FromStr, mem::swap};

fn print_map(map: &Vec<Vec<bool>>){
    for y in 0..180 {
        for x in 420..=580 {
            if map[y as usize][x as usize] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let input_data = fs::read_to_string("input").unwrap();

    let mut map: Vec<Vec<bool>> = vec![vec![false; 1024]; 1024];

    let mut cords: Vec<&str>;
    let mut pos: Vec<i32>;
    let mut last_pos: Vec<i32>;
    let mut max_depth = 0;

    for line in input_data.lines(){
        last_pos = vec![];
        cords = line.split(" -> ").collect();
        for cord in cords {
            pos = cord.split(",").map(|d| FromStr::from_str(d).unwrap()).collect::<Vec<_>>();
            
            
            if last_pos.len() == 2{
                let mut from_y: i32 = pos[1];
                let mut to_y: i32 = last_pos[1];
                let mut from_x: i32 = pos[0];
                let mut to_x: i32= last_pos[0];
                if pos[1] > last_pos[1]{
                    swap(&mut from_y, &mut to_y);
                }
                if pos[0] > last_pos[0]{
                    swap(&mut from_x, &mut to_x);
                }

                if to_y > max_depth{
                    max_depth = to_y;
                }

                for y in from_y..=to_y {
                    for x in from_x..=to_x {
                        map[y as usize][x as usize] = true;
                    }
                }
            }

            last_pos = pos.clone();
        }
    }

    for i in 0..1024{
        map[(max_depth + 2) as usize][i as usize] = true;
    }

    print_map(&map);
    
    let mut sand_pieces = 0;
    loop {
        if map[0][500]{
            break;
        }
        let mut sand_pos: Vec<i32> = vec![500, 0];
        sand_pieces += 1;

        loop {
            if !map[(sand_pos[1]+1) as usize][sand_pos[0] as usize]{
                sand_pos[1] += 1;
                continue;
            }

            if !map[(sand_pos[1]+1) as usize][(sand_pos[0]-1) as usize]{
                sand_pos[1] += 1;
                sand_pos[0] -= 1;
                continue;
            }
            if !map[(sand_pos[1]+1) as usize][(sand_pos[0]+1) as usize]{
                sand_pos[1] += 1;
                sand_pos[0] += 1;
                continue;
            }

            map[sand_pos[1] as usize][sand_pos[0] as usize] = true;
            //io::stdin().read_line(&mut user_input).unwrap();
            //print_map(&map);
            //println!("{}", sand_pieces);
            break;
        }
    }
    println!();
    print_map(&map);
    println!("Result: {}", sand_pieces);
}
