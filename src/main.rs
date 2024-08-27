use std::env::var;

mod util;


fn get_os() -> String {
    os_info::get().to_string()
}

fn get_kernel() -> String {
    run!{"uname -r"}
}

fn get_uptime() -> String {
    run!{"uptime -p"}
}

fn get_shell() -> String {
    let shell = var("SHELL").expect("failed to get shell variable");
    let command = format!("{} --version", shell);
    run!{command}.lines().next().unwrap().to_string()
}

// fn get_resolution() -> String {
//     let output = run!{"xdpyinfo | grep dimensions"};
//     let splitted: Vec<&str> = output.split_whitespace().collect();
//     println!("{output:?}");
//     splitted[1..].join(" ")
// }

fn main() {
    let os = get_os();
    let kernel = get_kernel();
    let uptime = get_uptime();
    let shell = get_shell();
    // let resolution = get_resolution();
    println!("{shell}");
}
