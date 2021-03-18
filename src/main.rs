/* 
* dc is an arbitrary precision RPN calculator 
* (re)written in Rust. This tool is NOT 
* part of the GNU coreutils
* by Marco Cetica 2021
*
*/
extern crate clap;
const DC_VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Dependencies
use std::io;
use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, App};
mod evaluate_expr;

fn main(){
    let mut user_input = String::new(); // Raw user input
    let mut stack_vec: Vec<f64> = Vec::new(); // Emulate a stack for Reverse Polish Notation
    let mut cli_args: bool = false; // Check whether user called program trough non-interactive mode

    // Handle command line arguments
    let matches = App::new("dc - RPN Desk calculator")
                        .version(DC_VERSION)
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
            evaluate_expr::evaluate_expression(tokens, &mut stack_vec);

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
                Err(err) => {
                    println!("Unable to open file \"{0}\": \"{1}\"", &file_path, err);
                    std::process::exit(1);
                },
            };
            // Read file's content
            let mut file_buffer = String::new();
            match file_stream.read_to_string(&mut file_buffer) {
                Ok(_) => (),
                Err(err) => eprintln!("Unable to read from file \"{0}\": \"{1}\"", file_path, err),
            }

            // Tokenize buffer
            file_buffer.retain(|c| {c != '\n'});
            let tokens: Vec<&str> = file_buffer.split(' ').collect();
            // Evaluate buffer
            evaluate_expr::evaluate_expression(tokens, &mut stack_vec);
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
        evaluate_expr::evaluate_expression(tokens, &mut stack_vec); // We borrow stack, since we need to reuse it

        // Clear user input string
        user_input.clear();
    }
}

#[cfg(test)]
mod test_evaluate_expr;
