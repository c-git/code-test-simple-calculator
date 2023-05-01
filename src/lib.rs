use std::io::{self, Lines};

#[derive(Debug)]
pub enum Input {
    File(Lines<std::io::BufReader<std::fs::File>>),
    StdIn(Lines<std::io::StdinLock<'static>>),
}

impl Input {
    fn get_line(&mut self) -> Option<io::Result<String>> {
        match self {
            Input::File(ref mut v) => v.next(),
            Input::StdIn(ref mut v) => v.next(),
        }
    }
}

pub fn run(mut input: Input) -> io::Result<()> {
    while let Some(line) = input.get_line() {
        let line = line?.trim().to_lowercase();
        if line == "quit" {
            break;
        }
        println!("{line}");
    }
    Ok(())
}
