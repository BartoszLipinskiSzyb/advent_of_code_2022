use std::{fs, str::from_utf8};

fn draw_pixel(canvas: &mut Vec<bool>, &regx: &i32, &cycle: &i32){
    let crt_position: i32 = (cycle - 1) % 40;    

    println!("crt: {} regx: {} cycle: {}", crt_position, regx, cycle);

    for i in (regx-1)..=(regx+1) {
        if i == crt_position {
            canvas.push(true);
            return ();
        }
    }

    canvas.push(false);
    return ();
}

fn main() {
    let raw_input = fs::read("input").expect("Should be able to read the file");
    
    let lines: Vec<&str> = raw_input.split(|&c| c == b'\n').map(|utf| from_utf8(utf).unwrap()).collect();
    
    let mut cycle = 0;
    let mut regx = 1;

    let mut rendered_image: Vec<bool> = vec![];

    for line in lines {
        // println!("Regx: {} Cycle: {}", regx, cycle);
        if line.len() == 0 {
            continue;
        }
        let splitted: Vec<&str> = line.split(|c| c == ' ').collect();

        cycle += 1;
        draw_pixel(&mut rendered_image, &regx, &cycle);

        if splitted[0] == "addx" {
            cycle += 1;
            draw_pixel(&mut rendered_image, &regx, &cycle);
            regx += i32::from_str_radix(splitted[1], 10).expect("Parameter should be base 10 number");
        }
    }

    for i in 0..rendered_image.len() {
        if i % 40 == 0 {
            println!();    
        }
        if rendered_image[i] {
            print!("#");
        } else {
            print!(".");
        }
    }

    println!("Len: {}", rendered_image.len());
}
