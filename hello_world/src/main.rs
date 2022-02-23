fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let name = "world";

    let mut number = 4;
    number += 2;

    println!("Hello, {} {}!", max(2, number), name);
}
