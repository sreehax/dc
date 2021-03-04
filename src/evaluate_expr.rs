// Evaluate a prefix(RPN) expression
pub fn evaluate_expression(expr: Vec<&str>, stack: &mut Vec<f64>) {
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
                // Check whether stack is empty
                if stack.is_empty() {
                    println!("ERR: stack is empty");
                    continue;
                }
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