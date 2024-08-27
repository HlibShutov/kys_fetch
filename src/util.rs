use std::process::Command;

#[macro_export]
macro_rules! run {
    ($cmd:expr) => {{
        let args: Vec<&str> = $cmd.split_whitespace().collect();
        $crate::util::run(args)
    }};
}

pub fn run(args: Vec<&str>) -> String {
    // if args.contains(&"|") {
    //     String::from("kys")
    // } else {
        let output = Command::new(args[0]).args(&args[1..]).output().expect("Failed to execute command");
        String::from_utf8(output.stdout).expect("Failed to read output").trim().to_string()
    // }
}
