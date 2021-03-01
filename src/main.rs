use std::io;

fn main() {
    let mut user_input = String::new(); // Raw user input

    loop {
        // Read from stdin
        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read from stdin");

        // Remove newline and remove whitespace
        let tokens: Vec<char> = user_input.chars().collect();

        // For each item, match available operations
        for token in tokens {
            match token {
                '+' => {
                    println!("SUM");
                },

                '-' => {
                    println!("SUB");
                },

                '*' => {
                    println!("DOT");
                },

                '/' => {
                    println!("DIV");
                },

                number => {
                    println!("{:?}", number);
                }
                
            }
        }

        // Clear user input string
        user_input.clear();

    }

}
