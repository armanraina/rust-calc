use std::io::Write;
use std::{io, num};
use std::str::FromStr;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("read input failure");

        let expression_tree = parse(&input);
        let result = calculate(&expression_tree);
        println!("{}", result);
        io::stdout().flush();
    }
}

#[derive(Debug)]
struct ExpressionTree{
    num1:f64,
    num2:f64,
    operation:Operation
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulus
}
impl FromStr for Operation {
    type Err = ();
    fn from_str(input: &str)-> Result<Operation, Self::Err>{
        match input {
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "^" => Ok(Operation::Exponent),
            "%" => Ok(Operation::Modulus),
            _ => Err(()),
        }
    }
}
fn parse(input:&String)->ExpressionTree
{
    let mut iter = input.split_ascii_whitespace();
    let num1 = iter.next();
    let operation = iter.next();
    let num2 = iter.next();

    let num1 = match num1{
        None => panic!("missing input"),
        Some(num) => num.parse::<f64>(),
    }.unwrap();

    let num2 = match num2 {
        None => panic!("missing input"),
        Some(num) => num.parse::<f64>(),
    }.unwrap();

    let operation = match operation {
        None => panic!("missing input"),
        Some(num) => num.parse::<Operation>(),
    }.unwrap();

    ExpressionTree {
        num1,
        num2,
        operation
    }
}

fn calculate(expression_tree:&ExpressionTree)->f64 {
    let num1 = expression_tree.num1;
    let num2 = expression_tree.num2;

    match expression_tree.operation {
        Operation::Add => num1 + num2,
        Operation::Subtract => num1 - num2,
        Operation::Multiply => num1 * num2,
        Operation::Divide => num1 / num2,
        Operation::Exponent => num1.powf(num2),
        Operation::Modulus => num1 % num2,
    }
}
