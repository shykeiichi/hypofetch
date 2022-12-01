use whoami;
use humantime::format_duration;
use std::env::var;
use json;
use std::env;
use std::fs;
extern crate uptime_lib;

fn main() {
    let distro_text: String;
    if env::args().collect::<Vec<String>>().len() >= 2 {
        distro_text = fs::read_to_string(format!("/home/{}/.config/hypo/distro/{}", whoami::username(), env::args().collect::<Vec<String>>()[1])).unwrap();
    } else {
        distro_text = fs::read_to_string(format!("/home/{}/.config/hypo/distro/{}", whoami::username(), whoami::distro().replace(" ", "_"))).unwrap();
    }
    let distro: json::JsonValue = json::parse(&distro_text).unwrap();

    let color: &str;

    match distro["color"].as_str().unwrap() {
        "black" => color = "\x1b[0;30m",
        "red" => color = "\x1b[0;31m",
        "green" => color = "\x1b[0;32m",
        "yellow" => color = "\x1b[0;33m",
        "blue" => color = "\x1b[0;34m",
        "magenta" => color = "\x1b[0;35m",
        "cyan" => color = "\x1b[0;36m",
        "white" => color = "\x1b[0;37m",
        _ => color = ""
    }

    let color_bold: String = color.replace("0", "1");
    let color_reset: String = String::from("\x1b[0;0m");

    println!(" {}{}   {}{}{} {}@{}", color, distro["small"][0].as_str().unwrap(), &color_bold, "\x1b[f007", color_reset, whoami::username(), whoami::hostname());
    println!(" {}{}   {}{}{} {}"   , color, distro["small"][1].as_str().unwrap(), &color_bold, "  OS", color_reset, whoami::distro());
    println!(" {}{}   {}{}{} {}"   , color, distro["small"][2].as_str().unwrap(), &color_bold, "  WM", color_reset, var("DESKTOP_SESSION").unwrap());
    println!(" {}{}   {}{}{} {}"   , color, distro["small"][3].as_str().unwrap(), &color_bold, "  UP", color_reset, format_duration(uptime_lib::get().unwrap()));
}
