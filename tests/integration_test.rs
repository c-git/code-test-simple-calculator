use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::{
    fs::{self, File},
    process::Command,
};

enum InputType {
    StdIn,
    Filename,
}

#[test]
fn one_register() -> Result<(), Box<dyn std::error::Error>> {
    run_executable(
        "case_one_reg_in.txt",
        &InputType::StdIn,
        "case_one_reg_out.txt",
    )
}

#[test]
fn two_register1() -> Result<(), Box<dyn std::error::Error>> {
    run_executable(
        "case_two_reg1_in.txt",
        &InputType::StdIn,
        "case_two_reg1_out.txt",
    )
}

#[test]
fn two_register2() -> Result<(), Box<dyn std::error::Error>> {
    run_executable(
        "case_two_reg2_in.txt",
        &InputType::StdIn,
        "case_two_reg2_out.txt",
    )
}

fn run_executable(
    input_filename: &str,
    input_type: &InputType,
    expected_output_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_folder = "tests/data/";
    let input_filename = format!("{data_folder}{input_filename}");
    let expected_output_filename = format!("{data_folder}{expected_output_filename}");

    let expected_output = fs::read_to_string(&expected_output_filename)
        .map_err(|e| format!("Failed to load output: {expected_output_filename} Error:{e}"))?
        .trim() // Trim expected output
        .to_owned();

    let mut command = Command::cargo_bin("code-test-simple-calculator")?;

    match input_type {
        InputType::StdIn => {
            let input_file = File::open(&input_filename)
                .map_err(|e| format!("Failed to load input: {input_filename} Error: {e}"))?;
            command.stdin(input_file)
        }
        InputType::Filename => command.arg(std::fs::canonicalize(input_filename)?),
    };

    command.assert().stdout(
        predicate::str::diff(expected_output).trim(), // Trims actual output
    );
    Ok(())
}
