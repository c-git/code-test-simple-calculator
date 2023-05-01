use std::io::{self, BufRead};

pub fn run(input: &mut dyn BufRead) -> io::Result<()> {
    for line in input.lines() {
        let line = line?.trim().to_lowercase();
        if line == "quit" {
            break;
        }
        println!("{line}");
    }
    Ok(())
}
