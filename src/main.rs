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

fn get_resolution() -> String {
    let output = run!{"xdpyinfo | grep dimensions"};
    let splitted: Vec<&str> = output.split_whitespace().collect();
    splitted[1..].join(" ")
}

fn get_theme() -> String {
    run!{"gsettings get org.gnome.desktop.interface gtk-theme"}.replace("'", "")
}

fn get_cpu() -> String {
    let output = run!{"lscpu | grep Model"};
    let splitted: Vec<String> = output.lines().map(|line| {
        let splitted_line: Vec<&str> = line.split_whitespace().collect();
        splitted_line[2..].join(" ")
    }).collect();
    format!("{}{}", splitted[0], splitted[1])
}

fn get_gpu() -> String {
    let output = run!{"lspci | grep -i vga"};
    let gpu = output.lines().next().unwrap().split(": ").next().unwrap();
    gpu.to_string()
}

fn get_memory() -> String {
    let output = run!{"free -h | grep Mem"};
    let memory: Vec<&str> = output.split_whitespace().collect();
    let total_memory = memory[1];
    let free_space = memory[3];
    format!("{}/{}", free_space, total_memory)
}

fn main() {
    let os = get_os();
    let kernel = get_kernel();
    let uptime = get_uptime();
    let shell = get_shell();
    let resolution = get_resolution();
    let theme = get_theme();
    let cpu = get_cpu();
    let gpu = get_gpu();
    let memory = get_memory();
    println!("{memory}");
}
