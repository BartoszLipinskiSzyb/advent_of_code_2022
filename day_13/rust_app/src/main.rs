fn silnia(x: i128) -> i128 {
    if x <= 1 {
        1
    } else {
        x * silnia(x - 1)
    }
}

fn main() {
    for i in  0..100 {
        println!("{}: {}", i, silnia(i));
    }
}
