use std::io;
use calc::{calculate, Operation};
use dialoguer::{theme::ColorfulTheme, Select};

const MODES:[&str; 4] = ["Addition", "Subtraction", "Multiplication", "Division"];
fn main() {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an operation to perform")
        .default(0)
        .items(&MODES[..])
        .interact()
        .unwrap();
        println!("selected : {}", selection);

    let operation = match MODES[selection] {
        "Addition"        => Operation::Addition,
        "Subtraction"     => Operation::Subtraction,
        "Multiplication"  => Operation::Multiplication,
        "Division"        => Operation::Division,
        _ => {   panic!("Please Enter Value!, Try Again");    }
    };

    let first_number = read_input("Enter the first value:");
    let second_number = read_input("Enter the second value:");
    let calc_value = calculate(operation, first_number, second_number);

    match calc_value {
        Some(number) => println!("= {}", number),
        None => println!("Invalid Operation!"),
    }
}

fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed, Please try again");
    let parsed_number: f64 = input.trim().parse().expect("Failed, Please Enter Valid Number");
    parsed_number
}