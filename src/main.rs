use std::io;

// TODO: Command line parser support
// TODO: Limit/setup decimal digits

fn main(){
    let mut user_input = String::new(); // Raw user input
    let mut stack_vec: Vec<f64> = Vec::new(); // Emulate a stack for Reverse Polish Notation

    loop {
        // Read from stdin
        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read from stdin.");

        // Remove newline and remove whitespace
        let tokens: Vec<&str> = user_input.trim().split(' ').collect();

        // For each item, match available operations
        for token in tokens {
            match token {
                // Math options
                "+" => { // ADDITION
                    // Check if stack has enough items
                    if stack_vec.len() < 2 {
                        println!("ERR: stack is empty or either has too few elements.");
                        continue;
                    }
                    // Pops two elements, sum them and push the result back to the stack
                    let sum = stack_vec.pop().unwrap() + stack_vec.pop().unwrap();
                    stack_vec.push(sum);
                },

                "-" => { // SUBTRACTION
                    // Check if stack has enough items
                    if stack_vec.len() < 2 {
                        println!("ERR: stack is empty or either has too few elements.");
                        continue;
                    }
                    // Pops two elements, subtract them and push the result back to the stack
                    let sub = -(stack_vec.pop().unwrap() - stack_vec.pop().unwrap());
                    stack_vec.push(sub);
                },

                "*" => { // MULTIPLICATION
                    // Check if stack has enough items
                    if stack_vec.len() < 2 {
                        println!("ERR: stack is empty or either has too few elements.");
                        continue;
                    }
                    // Pops two elements, multiply them and push the result back to the stack
                    let mul = stack_vec.pop().unwrap() * stack_vec.pop().unwrap();
                    stack_vec.push(mul);
                },

                "/" => { // DIVISION
                    // Check if stack has enough items
                    if stack_vec.len() < 2 {
                        println!("ERR: stack is empty or either has too few elements.");
                        continue;
                    }
                    // Pops two elements
                    let divisor = stack_vec.pop().unwrap();
                    // Check if divisor is equal to zero
                    if divisor == 0.0 {
                        println!("ERR: unable to divide by zero.");
                        continue;
                    } else {
                        let dividend = stack_vec.pop().unwrap();
                        stack_vec.push(dividend / divisor);
                    }
                },

                "%" => { // MODULO
                    // Check if stack has enough items
                    if stack_vec.len() < 2 {
                        println!("ERR: stack is empty or either has too few elements.");
                        continue;
                    }
                    // Pops two elements
                    let second_term = stack_vec.pop().unwrap();
                    // Check if remainder is equal to zero
                    if second_term == 0.0 {
                        println!("ERR: unable to divide by zero.");
                        continue;
                    } else {
                        let first_term = stack_vec.pop().unwrap();
                        stack_vec.push(first_term % second_term);
                    }
                },

                "^" => { // EXPONENTIAL
                    // Check if stack has enough items
                    if stack_vec.len() < 2 {
                        println!("ERR: stack is empty or either has too few elements.");
                        continue;
                    }
                    // Pops two elements
                    let exponent = stack_vec.pop().unwrap();
                    let base = stack_vec.pop().unwrap();
                    // Compute the exponent and push the result to the stack
                    stack_vec.push(base.powf(exponent));
                },

                "v" => { // SQUARE ROOT
                    // Check if stack has enough items
                    if stack_vec.len() < 1 {
                        println!("ERR: stack is empty or either has too few elements.");
                        continue;
                    }
                    // Pops one element
                    let argument = stack_vec.pop().unwrap();
                    stack_vec.push(argument.sqrt());
                },
                
                // Stack options

                "c" => { // CLEAR THE STACK
                    stack_vec.clear();
                },

                "d" => { // DUPLICATE LAST ELEMENT OF THE STACK
                    let top_el = stack_vec.last().cloned();
                    stack_vec.push(top_el.unwrap());
                },

                "p" => { // PRINT TOP ELEMENT OF THE STACK
                    println!("{0}", stack_vec.last().unwrap());
                },

                "f" => { // PRINT ENTIRE STACK
                    for item in &stack_vec {
                        println!("{0}", item);
                    }
                },

                // Default case
                number => { // Handle all other options
                    // Check if whether is a number
                    match number.parse::<f64>() {
                        Ok(_) => stack_vec.push(number.parse::<f64>().unwrap()),
                        Err(_) => println!("ERR: unrecognized option."),
                    }
                }
            }
        }
        // Clear user input string
        user_input.clear();
    }
}