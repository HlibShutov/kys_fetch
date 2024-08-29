use std::cmp::Ordering;
use std::{env::var, fs};

mod prelude {
    pub use colored::{ColoredString, Colorize};
}

use prelude::*;

mod util;
use crate::util::colorize;

fn get_os() -> String {
    os_info::get().to_string()
}

fn get_kernel() -> String {
    run! {"uname -r"}
}

fn get_uptime() -> String {
    run! {"uptime -p"}
}

fn get_shell() -> String {
    let shell = var("SHELL").expect("failed to get shell variable");
    let command = format!("{} --version", shell);
    run! {command}.lines().next().unwrap().to_string()
}

fn get_resolution() -> String {
    let output = run! {"xdpyinfo | grep dimensions"};
    let splitted: Vec<&str> = output.split_whitespace().collect();
    splitted[1..].join(" ")
}

fn get_theme() -> String {
    run! {"gsettings get org.gnome.desktop.interface gtk-theme"}.replace("'", "")
}

fn get_cpu() -> String {
    let output = run! {"lscpu | grep Model"};
    let splitted: Vec<String> = output
        .lines()
        .map(|line| {
            let splitted_line: Vec<&str> = line.split_whitespace().collect();
            splitted_line[2..].join(" ")
        })
        .collect();
    format!("{}{}", splitted[0], splitted[1])
}

fn get_gpu() -> String {
    let output = run! {"lspci | grep -i vga"};
    let gpu = output.lines().next().unwrap().split(": ").next().unwrap();
    gpu.to_string()
}

fn get_memory() -> String {
    let output = run! {"free -h | grep Mem"};
    let memory: Vec<&str> = output.split_whitespace().collect();
    let total_memory = memory[1];
    let free_space = memory[3];
    format!("{}/{}", free_space, total_memory)
}

fn get_user() -> String {
    let user = var("USER").expect("Failed to get user");
    user
}

fn get_hostname() -> String {
    let hostname = run! {"hostname"};
    hostname
}

fn get_text_color() -> [u8; 3] {
    let home = var("HOME").unwrap();
    let command = format!("cat {}/.Xdefaults | grep urxvt.cursorColor", home);
    let output = run! {command};
    let hex_color = output.split_whitespace().collect::<Vec<&str>>()[1]
        .to_string()
        .trim_start_matches("#")
        .to_string();
    let red = u8::from_str_radix(&hex_color[0..2], 16).expect("Cant convert color");
    let green = u8::from_str_radix(&hex_color[2..4], 16).expect("Cant convert color");
    let blue = u8::from_str_radix(&hex_color[4..6], 16).expect("Cant convert color");
    [red, green, blue]
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
    let user = get_user();
    let hostname = get_hostname();
    let text_color = get_text_color();

    let logo = fs::read_to_string("logos/arch.txt").expect("Failed to read logo");
    let logo_lines: Vec<&str> = logo.lines().collect();

    let sys_info = vec![
        format!(
            "{}@{}",
            colorize(&user, &text_color),
            colorize(&hostname, &text_color)
        ),
        "-".repeat(user.len() + hostname.len()),
        format!("OS: {}", colorize(&os, &text_color)),
        format!("Kernel: {}", colorize(&kernel, &text_color)),
        format!("Uptime: {}", colorize(&uptime, &text_color)),
        format!("Shell: {}", colorize(&shell, &text_color)),
        format!("Resolution: {}", colorize(&resolution, &text_color)),
        format!("Theme: {}", colorize(&theme, &text_color)),
        format!("CPU: {}", colorize(&cpu, &text_color)),
        format!("GPU: {}", colorize(&gpu, &text_color)),
        format!("Memory: {}", colorize(&memory, &text_color)),
    ];

    let begin_info_line_number = (logo_lines.len() - sys_info.len()) / 2;

    for (i, logo_line) in logo_lines.into_iter().enumerate() {
        let info_line = match i.cmp(&begin_info_line_number) {
            Ordering::Less => "",
            _ => match sys_info.get(i.saturating_sub(begin_info_line_number)) {
                Some(line) => line,
                _ => "",
            },
        };
        println!(
            "{:<40} {}",
            logo_line.truecolor(text_color[0], text_color[1], text_color[2]),
            info_line
        );
    }
}
