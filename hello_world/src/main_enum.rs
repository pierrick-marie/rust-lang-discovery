enum Expr {
    Null,
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div{dividend: i32, divisor: i32},
    Val(i32),
}

fn print_expr(expr: Expr) {
    match expr {
        Expr::Null => println!("No value"),
        Expr::Add(a, b) => println!("{}", a + b),
        Expr::Sub(a, b) => println!("{}", a - b),
        Expr::Mul(a, b) => println!("{}", a * b),
        Expr::Div{dividend: _a, divisor: 0} => println!("Divisor is zero!"),
        Expr::Div{dividend: a, divisor: b} => println!("{}", a / b),
        Expr::Val(a) => println!("{}", a),
    }
}

fn main() {

    let mut expr = Expr::Null;
    print_expr(expr);

    expr = Expr::Add(2, 3);
    print_expr(expr);

    expr = Expr::Sub(2, 3);
    print_expr(expr);

    expr = Expr::Mul(2, 3);
    print_expr(expr);

    expr = Expr::Div{dividend: 2, divisor: 0};
    print_expr(expr);

    expr = Expr::Div{dividend: 15, divisor: 3};
    print_expr(expr);

    expr = Expr::Val(3);
    print_expr(expr);
}
