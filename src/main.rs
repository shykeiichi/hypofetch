use whoami;
use std::fs;
use std::env::var;
mod logo;

fn get_time() -> String {
    let mut uptime: f64 = fs::read_to_string("/proc/uptime").unwrap().split(" ").map(|item| item.to_string()).collect::<Vec<String>>()[0].as_str().parse::<f64>().unwrap();
    let mut uptime_str: String = "".to_string();

    let days: f64 = uptime as f64 / 60.0 / 60.0 / 24.0;
    if days >= 1.0 {
        uptime_str.push_str(&format!("{} Days ", days as i32));
        uptime -= days * 24.0 * 60.0 * 60.0;
    }

    let hours: f64 = uptime / 60.0 / 60.0;
    if hours >= 1.0 {
        uptime_str.push_str(&format!("{} Hours ", hours as i32));
        uptime -= (hours as i32 * 60 * 60) as f64;
    }

    let minutes: f64 = uptime / 60.0;
    if minutes >= 1.0 {
        uptime_str.push_str(&format!("{} Minutes ", minutes as i32));
        uptime -= (minutes as i32 * 60) as f64;
    }

    uptime_str.push_str(&format!("{} Seconds", uptime as i32));
    uptime_str
}

fn main() {
    get_time();

    let distro: logo::Logos;

    match whoami::distro().as_str() {
        "Arch Linux" => distro = logo::Logos::ArchLinux,
        "Ubuntu" => distro = logo::Logos::Ubuntu,
        "Fedora" => distro = logo::Logos::Fedora,
        _ => distro = logo::Logos::ArchLinux
    };

    println!("            ╭────────╮");
    println!(" {}   \x1b[0;0m│ \x1b[1;31m User \x1b[0;0m│ {}\x1b[1m@\x1b[0m{}", logo::get_logo(&distro)[0], whoami::username(), whoami::hostname());
    println!(" {}   \x1b[0;0m│ \x1b[1;33m   OS \x1b[0;0m│ {}",    logo::get_logo(&distro)[1], whoami::distro());
    println!(" {}   \x1b[0;0m│ \x1b[1;36m   WM \x1b[0;0m│ {}",    logo::get_logo(&distro)[2], var("XDG_CURRENT_DESKTOP").unwrap().as_str());
    println!(" {}   \x1b[0;0m│ \x1b[1;34m   UP \x1b[0;0m│ {}",    logo::get_logo(&distro)[3], get_time());
    println!("            ╰────────╯");
}

