use std::process::{Command, Stdio};
use colored::{ColoredString, Colorize};

#[macro_export]
macro_rules! run {
    ($cmd:expr) => {{
        let commands: Vec<&str> = $cmd.split("|").collect();
        $crate::util::run(commands)
    }};
}

pub fn run(commands: Vec<&str>) -> String {
    if commands.len() == 1 {
        let args: Vec<&str> = commands[0].split_whitespace().collect();
        let output = Command::new(args[0])
            .args(&args[1..])
            .output()
            .expect("Failed to execute command");
        String::from_utf8(output.stdout)
            .expect("Failed to read output")
            .trim()
            .to_string()
    } else {
        let mut commands_iter = commands.into_iter();
        let previous_command_args: Vec<&str> =
            commands_iter.next().unwrap().split_whitespace().collect();
        let mut previous_command = Command::new(previous_command_args[0])
            .args(&previous_command_args[1..])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        for command in commands_iter {
            let current_args: Vec<&str> = command.split_whitespace().collect();
            let current_command = Command::new(current_args[0])
                .args(&current_args[1..])
                .stdin(Stdio::from(previous_command.stdout.unwrap()))
                .stdout(Stdio::piped())
                .spawn()
                .expect(&format!("failed to execute command {}", current_args[0]));
            previous_command = current_command;
        }

        let output = previous_command.wait_with_output().expect("Failed to execute piped commands");
        let result = String::from_utf8(output.stdout).unwrap();

        result
    }
}

pub fn colorize(text: &str, color: &[u8; 3]) -> ColoredString {
    text.truecolor(color[0], color[1], color[2])
}
