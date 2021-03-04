extern crate clap;
const DC_VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Dependencies
use std::io;
use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, App};

// Takes expression and stack vector, then
// Evaluate expression and push result to the stack
fn evaluate_expression(expr: Vec<&str>, stack: &mut Vec<f64>) {
    // For each item, match available operations
    for token in expr {
        match token {
            // Math options
            "+" => { // ADDITION
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    continue;
                }
                // Pops two elements, sum them and push the result back to the stack
                let sum = stack.pop().unwrap() + stack.pop().unwrap();
                stack.push(sum);
            },

            "-" => { // SUBTRACTION
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    continue;
                }
                // Pops two elements, subtract them and push the result back to the stack
                let sub = -(stack.pop().unwrap() - stack.pop().unwrap());
                stack.push(sub);
            },

            "*" => { // MULTIPLICATION
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    continue;
                }
                // Pops two elements, multiply them and push the result back to the stack
                let mul = stack.pop().unwrap() * stack.pop().unwrap();
                stack.push(mul);
            },

            "/" => { // DIVISION
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    continue;
                }
                // Pops two elements
                let divisor = stack.pop().unwrap();
                // Check if divisor is equal to zero
                if divisor == 0.0 {
                    println!("ERR: unable to divide by zero.");
                    continue;
                } else {
                    let dividend = stack.pop().unwrap();
                    stack.push(dividend / divisor);
                }
            },

            "%" => { // MODULO
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    continue;
                }
                // Pops two elements
                let second_term = stack.pop().unwrap();
                // Check if remainder is equal to zero
                if second_term == 0.0 {
                    println!("ERR: unable to divide by zero.");
                    continue;
                } else {
                    let first_term = stack.pop().unwrap();
                    stack.push(first_term % second_term);
                }
            },

            "^" => { // EXPONENTIAL
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    continue;
                }
                // Pops two elements
                let exponent = stack.pop().unwrap();
                let base = stack.pop().unwrap();
                // Compute the exponent and push the result to the stack
                stack.push(base.powf(exponent));
            },

            "v" => { // SQUARE ROOT
                // Check if stack has enough items
                if stack.len() < 1 {
                    println!("ERR: stack is empty or either has too few elements.");
                    continue;
                }
                // Pops one element
                let argument = stack.pop().unwrap();
                stack.push(argument.sqrt());
            },
            
            // Stack options

            "c" => { // CLEAR THE STACK
                stack.clear();
            },

            "d" => { // DUPLICATE LAST ELEMENT OF THE STACK
                // Check whether stack is empty
                if stack.is_empty() {
                    println!("ERR: stack is empty");
                    continue;
                }
                let top_el = stack.last().cloned();
                stack.push(top_el.unwrap());
            },

            "p" => { // PRINT TOP ELEMENT OF THE STACK
                let head = stack.last().unwrap();
                if head.fract() == 0.0 { // If number is integer
                    println!("{0}", head); // Print without any rounding
                } else {
                    println!("{0:.4}", head); // Print with radix = 4
                }
            },

            "f" => { // PRINT ENTIRE STACK
                for item in stack.clone() {
                    println!("{0}", item); // Print without rounding
                }
            },

            // Default case
            number => { // Handle all other options
                // Check if whether is a number
                match number.parse::<f64>() {
                    Ok(_) => stack.push(number.parse::<f64>().unwrap()),
                    Err(_) => println!("ERR: unrecognized option."),
                }
            }
        }
    }
}

fn main(){
    let mut user_input = String::new(); // Raw user input
    let mut stack_vec: Vec<f64> = Vec::new(); // Emulate a stack for Reverse Polish Notation
    let mut cli_args: bool = false; // Check whether user called program trough non-interactive mode

    // Handle command line arguments
    let matches = App::new("dc - RPN Desk calculator")
                        .version("0.1.0")
                        .author("Marco C. <ceticamarco@gmail.com>")
                        .about("UNIX's dc clone")
                        .arg(Arg::with_name("expression")
                            .short("e")
                            .long("expression")
                            .required(false)
                            .takes_value(true)
                            .multiple(true)
                            .help("evaluate expression")
                        ).arg(Arg::with_name("file")
                            .short("f")
                            .long("file")
                            .required(false)
                            .takes_value(true)
                            .multiple(true)
                            .help("evaluate contents of file")
                        ).arg(Arg::with_name("version")
                        .short("V")
                        .long("version")
                        .required(false)
                        .help("output version information and exit")
                        ).get_matches();

    // Handle command line parameters if required
    if matches.is_present("expression") { // Inline expression option
        let expressions: Vec<&str> = matches.values_of("expression").unwrap().collect(); // Retrieve parameter
        for expression in expressions { // Handle multiple expressions at once
            // Tokenize expression
            let tokens: Vec<&str> = expression.trim().split(' ').collect();
            // Evaluate expression
            evaluate_expression(tokens, &mut stack_vec);
        }
        // There's nothing left to do
        cli_args = true;
    }

    if matches.is_present("file") { // Evaluate file option
        let files_path: Vec<&str> = matches.values_of("file").unwrap().collect(); // Retrieve parameter
        for file_path in files_path { // Handle multiple files at once
            // Open file
            let mut file_stream = match File::open(&file_path) {
                Ok(file_stream) => file_stream,
                Err(err) => panic!("Unable to open file \"{0}\": \"{1}\"", &file_path, err),
            };
            // Read file's content
            let mut file_buffer = String::new();
            match file_stream.read_to_string(&mut file_buffer) {
                Ok(_) => (),
                Err(err) => panic!("Unable to read from file \"{0}\": \"{1}\"", file_path, err),
            }

            // Tokenize buffer
            file_buffer.retain(|c| {c != '\n'});
            let tokens: Vec<&str> = file_buffer.split(' ').collect();
            // Evaluate buffer
            evaluate_expression(tokens, &mut stack_vec);
        }
        // There's nothing left to do
        cli_args = true;
    }

    if matches.is_present("version") {
        println!(
            "dc - RPN Desk Calculator {version}\n\
            This tool is NOT part of the GNU coreutils.\n\
            Developed by Marco C.(<ceticamarco@gmail.com>) (c) 2021", 
            version=DC_VERSION);

        cli_args = true;
    }

    /* If cli_args is set, it means that user has already
     * used the tool through cli arguments.
     * We cannot do this using std::process::exit(0)
     * since it would prevent multiple instance of the same
     * parameter(for instance multiples -f or multiples -e)  */
     if cli_args == true {
         std::process::exit(0);
     }

    // Otherwise, just read from standard input
    loop {
        // Read from stdin
        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read from stdin.");

        // Remove newline and remove whitespace
        let tokens: Vec<&str> = user_input.trim().split(' ').collect();

        // Evaluate expression
        evaluate_expression(tokens, &mut stack_vec); // We borrow stack, since we need to reuse it

        // Clear user input string
        user_input.clear();
    }
}