mod calculator;

use calculator::Calculator;

fn main() {
    let tokens = Calculator::parse("2 * 2 + 48 / 4");
    println!("{:?}", tokens);

    let expr = Calculator::expression(tokens.unwrap());
    println!("{:?}", expr);

    let value = Calculator::evaluate(expr);
    println!("{}", value.unwrap());
}
