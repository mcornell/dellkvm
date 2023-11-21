use std::{
    io::{BufRead, BufReader},
    process::Command,
};

fn main() {
    let mut input_source_command = Command::new("ddcutil");
    input_source_command.arg("getvcp").arg("60").arg("-t");

    let input_soutce_command_output = input_source_command
        .output()
        .expect("Failed to execute ddcutil");

    let mut input_reader = BufReader::new(&input_soutce_command_output.stdout[..]);

    let mut first_line = String::new();
    input_reader
        .read_line(&mut first_line)
        .expect("reading stdout from ddcutil should not fail");

    let input_source_info: Vec<&str> = first_line.split_whitespace().collect();
    println!("Current Active Source: {:?}", input_source_info);

    let mut ddcutil_command = Command::new("ddcutil");
    ddcutil_command.arg("setvcp").arg("60");

    if vec!["x1b", "x1B"]
        .iter()
        .any(|&i| i == input_source_info[3])
    {
        ddcutil_command.arg("0x11");
    } else {
        ddcutil_command.arg("0x1b");
    }

    let output = ddcutil_command
        .output()
        .expect("Failed to execute ddcutil to swtich inputs");

    println!(
        "Command: {:?} {:?}",
        ddcutil_command.get_program(),
        ddcutil_command.get_args()
    );
    println!("stdout: {}", String::from_utf8(output.stdout).unwrap());
    println!("stderr: {}", String::from_utf8(output.stderr).unwrap());
}
