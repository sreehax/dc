// Convert from degree to radians and vice versa
pub fn deg_to_rad(angle: f64) -> f64 {
    (angle * std::f64::consts::PI)/180.0
}

pub fn rad_to_deg(radian: f64) -> f64 {
    (radian * 180.0)/std::f64::consts::PI
}

/* Evaluate a prefix(RPN) expression
 * Returns false on syntax/domain errors */
pub fn evaluate_expression(expr: Vec<&str>, stack: &mut Vec<f64>) -> bool {
    // For each item, match available operations
    for token in expr {
        match token {
            // Math options
            "+" => { // ADDITION
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Pops two elements, sum them and push the result back to the stack
                let sum: f64 = stack.pop().unwrap() + stack.pop().unwrap();
                stack.push(sum);
            },

            "-" => { // SUBTRACTION
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Pops two elements, subtract them and push the result back to the stack
                let sub: f64 = -(stack.pop().unwrap() - stack.pop().unwrap());
                stack.push(sub);
            },

            "*" => { // MULTIPLICATION
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Pops two elements, multiply them and push the result back to the stack
                let mul: f64 = stack.pop().unwrap() * stack.pop().unwrap();
                stack.push(mul);
            },

            "/" => { // DIVISION
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Pops two elements
                let divisor: f64 = stack.pop().unwrap();
                // Check if divisor is equal to zero
                if divisor == 0.0 {
                    println!("ERR: unable to divide by zero.");
                    return false;
                } else {
                    let dividend : f64= stack.pop().unwrap();
                    stack.push(dividend / divisor);
                }
            },

            "%" => { // MODULO
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Pops two ele:ments
                let second_term: f64 = stack.pop().unwrap();
                // Check if remainder is equal to zero
                if second_term == 0.0 {
                    println!("ERR: unable to divide by zero.");
                    return false;
                } else {
                    let first_term: f64 = stack.pop().unwrap();
                    stack.push(first_term % second_term);
                }
            },

            "^" => { // EXPONENTIAL
                // Check if stack has enough items
                if stack.len() < 2 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Pops two elements
                let exponent: f64 = stack.pop().unwrap();
                let base: f64 = stack.pop().unwrap();
                // Compute the exponent and push the result to the stack
                stack.push(base.powf(exponent));
            },

            "v" => { // SQUARE ROOT
                // Check if stack has enough items
                if stack.len() < 1 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Pops one element
                let argument: f64 = stack.pop().unwrap();
                stack.push(argument.sqrt());
            },

            "sin" => { // SINE
                // Check if stack has enough items
                if stack.len() < 1 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Convert angle to radians
                let angle: f64 = stack.pop().unwrap();
                let rad: f64 = deg_to_rad(angle);
                stack.push(rad.sin());
            },

            "cos" => { // COSINE
                // Check if stack has enough items
                if stack.len() < 1 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Convert angle to radians
                let angle: f64 = stack.pop().unwrap();
                let rad: f64 = deg_to_rad(angle);
                stack.push(rad.cos());
            },

            "tan" => { // TANGENT
                // Check if stack has enough items
                if stack.len() < 1 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Convert angle to radians
                let angle: f64 = stack.pop().unwrap();
                let rad: f64 = deg_to_rad(angle);
                stack.push(rad.tan());
            },

            "asin" => { // ARCSINE
                // Check if stack has enough items
                if stack.len() < 1 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Compute function
                let angle: f64 = stack.pop().unwrap();
                // Check if number is not in the range [-1;1]
                if angle >= -1.0 && angle <= 1.0 {
                    stack.push(angle.asin());
                } else {
                    println!("ERR: asin() function is defined in the range [-1;1]");
                    return false;
                }
            },

            "acos" => { // ARCCOSINE
                // Check if stack has enough items
                if stack.len() < 1 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Compute function
                let angle: f64 = stack.pop().unwrap();
                // Check if number is not in the range [-1;1]
                if angle >= -1.0 && angle <= 1.0 {
                    stack.push(angle.acos());
                } else {
                    println!("ERR: acos() function is defined in the range [-1;1]");
                    return false;
                }
            },

            "atan" => { // ARCTANGENT
                // Check if stack has enough items
                if stack.len() < 1 {
                    println!("ERR: stack is empty or either has too few elements.");
                    return false;
                }
                // Compute function
                let angle: f64 = stack.pop().unwrap();
                // Check if number is not in the range [-1;1]
                if angle >= -(std::f64::consts::PI/2.0) && angle <= (std::f64::consts::PI/2.0) {
                    stack.push(angle.atan());
                } else {
                    println!("ERR: atan() function is defined in the range [-pi/2;pi/2]");
                    return false;
                }
            },
            // Stack options

            "c" => { // CLEAR THE STACK
                stack.clear();
            },

            "d" => { // DUPLICATE LAST ELEMENT OF THE STACK
                // Check whether stack is empty
                if stack.is_empty() {
                    println!("ERR: stack is empty");
                    return false;
                }
                let top_el: std::option::Option<f64> = stack.last().cloned();
                stack.push(top_el.unwrap());
            },

            "p" => { // PRINT TOP ELEMENT OF THE STACK
                // Check whether stack is empty
                if stack.is_empty() {
                    println!("ERR: stack is empty");
                    return false;
                }
                let head: &f64 = stack.last().unwrap();
                if head.fract() == 0.0 { // If number is integer
                    println!("{0}", head); // Print without any rounding
                } else {
                    println!("{0:.4}", head); // Print with radix = 4
                }
            },

            "pd" => { // PRINT TOP ELEMENT IN DEGREE
                // Check whether stack is empty
                if stack.is_empty() {
                    println!("ERR: stack is empty!");
                    return false;
                }
                let head: &f64 = stack.last().unwrap();
                if head.fract() == 0.0 { // If number is integer
                    println!("{0}", rad_to_deg(*head));
                } else {
                    println!("{0:.4}", rad_to_deg(*head));
                }
            },

            "f" => { // PRINT ENTIRE STACK
                for item in stack.clone() {
                    println!("{0}", item); // Print without rounding
                }
            },

            "q" => { // EXIT
                std::process::exit(0);
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

    true
}
