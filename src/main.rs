use std::{
    env,
    fs::File,
    io::{self, stdin, BufRead},
};

use code_test_simple_calculator::Input;

fn main() {
    // Determine input source
    let input = if let Some(filename) = env::args().nth(1) {
        let file = File::open(filename).expect("Unable to access {filename}");
        Input::File(io::BufReader::new(file).lines())
    } else {
        Input::StdIn(stdin().lock().lines())
    };

    // Run program
    match code_test_simple_calculator::run(input) {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
}
