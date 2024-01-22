use colored::*;
use std::{io, process::exit};

const OPERATORS: [char; 6] = ['+', '-', '/', '*', 'q', 'h'];

const HELP_MESSAGE: &str = "
    'h' ==== help ---- show this output\n
    'q' ==== quit ---- exits application
 ";

fn main() {
    println!("Welcome to Mawunyo's Calculator!\n");
    println!("This is my first rust project!\n");

    loop {
        // * First number entry
        println!("\n{}", "Please enter first number: ".green());

        let first_number: f64 = number_input();

        //* Operator entry
        println!(
            "{}",
            "Please the operator. \nFor a list of operators please enter 'h': ".green()
        );

        let operator = operator_input();

        //* Second number entry

        println!("{}", "Please enter second number: ".green());

        let second_number = number_input();

        calculate_and_print_result(first_number, operator, second_number);

        if decision_input() {
            continue;
        } else {
            break;
        }
    }
}


fn number_input() -> f64 {
    let mut number_string = String::new();

    io::stdin()
        .read_line(&mut number_string)
        .expect("Please enter a correct input");

    let number_string_bytes = &number_string.clone().into_bytes();

    if number_string_bytes[0] == b'q' {
        exit(0);
    }

    match number_string.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Sorry it seams you entered a wrong value.".red());

            println!("{}", "Please enter a new value: ".bright_blue());

            number_input()
        }
    }
}

fn operator_input() -> char {
    let mut operator = String::new();

    io::stdin()
        .read_line(&mut operator)
        .expect("Please enter a correct input");

    match operator.trim().parse() {
        Ok(operator) => {
            if OPERATORS.contains(&operator) {
                match operator {
                    'h' => {
                        println!("{}", HELP_MESSAGE.blue().bright_magenta());

                        println!("{}", "Please enter operator:".yellow());

                        operator_input()
                    }

                    'q' => exit(0),
                    _ => operator,
                }
            } else {
                println!("{operator} {}", " is not a valid operator".red());

                operator_input()
            }
        }
        Err(_) => {
            println!("{}", "Sorry, it seams you entered a wrong operator.".red());

            operator_input()
        }
    }
}

fn decision_input() -> bool {
    println!("{}", "Do you want to compute other number? y/n: ".bold().magenta());

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Should be and input");

    if input.starts_with('y') {
        true
    } else {
        false
    }
}

fn calculate_and_print_result(first_number: f64, operator: char, second_number: f64) {
    let mut result: f64 = 0.0;

    if operator == OPERATORS[0] {
        result = first_number + second_number;
    }

    if operator == OPERATORS[1] {
        result = first_number - second_number;
    }
    if operator == OPERATORS[2] {
        result = first_number / second_number;
    }
    if operator == OPERATORS[3] {
        result = first_number * second_number;
    }

    println!("The result is: {}", result)
}
