use std::{
    env,
    fs::File,
    io::{self, stdin},
};

use code_test_simple_calculator::run;

fn main() {
    let result = if let Some(filename) = env::args().nth(1) {
        // Read input from file
        let file = File::open(filename).expect("Unable to access {filename}");
        let mut input = io::BufReader::new(file);
        run(&mut input)
    } else {
        // Read input from stdin
        let mut input = stdin().lock();
        run(&mut input)
    };

    // Run program
    match result {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}
