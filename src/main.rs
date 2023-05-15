use std::{process::Command, io::{BufReader, BufRead}};

fn main() {
    let mut input_source_command = Command::new("ddcutil");
    input_source_command.arg("getvcp");
    input_source_command.arg("60");
    input_source_command.arg("-t");

    let input_soutce_command_output = input_source_command.output().unwrap();

    let input_reader = BufReader::new(&input_soutce_command_output.stdout[..]);
    let first_line = input_reader.lines().next().unwrap().unwrap();

    let input_source_info: Vec<&str> = first_line.split_whitespace().collect();
    println!("Current Active Source: {:?}", input_source_info);

    let mut command = Command::new("ddcutil");
    command.arg("setvcp");
    command.arg("60");
    
    if vec!["x1b", "x1B"].iter().any(|&i| i == input_source_info[3]) {
        command.arg("0x11");
    } else {
        command.arg("0x1b");
    }

    let output = command.output().unwrap();
    
    println!("Command: {:?} {:?}", command.get_program(), command.get_args());
    println!("stdout: {}", String::from_utf8(output.stdout).unwrap());
    println!("stderr: {}", String::from_utf8(output.stderr).unwrap());
}
