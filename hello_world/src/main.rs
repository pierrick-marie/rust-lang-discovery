#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn origin() -> Point {
        Point {
            x: 0,
            y: 0
        }
    }

    fn dist_from_origin(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        return (sum_of_squares as f64).sqrt();
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
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
    let mut p3 = Point::new(11, 22);
    let p4 = Point::origin();

    println!("Point : {} {}!", point.x, point.y);
    println!("Point : {:#?}", point);
    println!("Point p1 : {} {}", p1.x, p1.y);
    println!("Point p2 : {} {}", p2.x, p2.y);

    // print_point(p1);

    print_point(p2);
    // print_point(p2);
    change_ref_point(&mut p2);
    print_point(p2);

    println!("Origin point : {}", p2.dist_from_origin());
    p2.translate(2, 3);
    print_point(p2);

    print_point(p3);
    p3.translate(1, 1);
    print_point(p3);

    print_point(p4);

}
