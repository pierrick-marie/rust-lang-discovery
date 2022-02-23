#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn print_point(p: Point) {
    println!("Point : {} {}", p.x, p.y);
}

// fn print_ref_point(p: &Point) {
//     println!("Point : {} {}", p.x, p.y);
// }

fn change_ref_point(p: &mut Point) {
    p.x += 10;
    p.y += 10;
}

fn main() {

    let point = Point {
        x: 32,
        y: 24,
    };
    let p1 = Point {
        x: 1,
        y: 2,
    };
    let mut p2 = p1;

    println!("Point : {} {}!", point.x, point.y);
    println!("Point : {:#?}", point);
    println!("Point p1 : {} {}", p1.x, p1.y);
    println!("Point p2 : {} {}", p2.x, p2.y);

    // print_point(p1);

    print_point(p2);
    // print_point(p2);
    change_ref_point(&mut p2);
    print_point(p2);
}
