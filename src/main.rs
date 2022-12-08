extern crate uptime_lib;
mod logo;
use std::env::var;

fn get_time() -> String {
    let mut uptime = match uptime_lib::get() {
        Ok(uptime) => {
            uptime.as_secs_f64()
        }
        Err(err) => {
            eprintln!("uptime: {}", err);
            std::process::exit(1);
        }
    };

    let mut uptime_str: String = String::new();

    let days: f64 = uptime as f64 / 60.0 / 60.0 / 24.0;
    if days >= 1.0 {
        uptime_str.push_str(&format!("{}d ", days as i32));
        uptime -= f64::from(days as i32 * 24 * 60 * 60);
    }

    let hours: f64 = uptime / 60.0 / 60.0;
    if hours >= 1.0 {
        uptime_str.push_str(&format!("{}h ", hours as i32));
        uptime -= f64::from(hours as i32 * 60 * 60);
    }

    let minutes: f64 = uptime / 60.0;
    if minutes >= 1.0 {
        uptime_str.push_str(&format!("{}m ", minutes as i32));
        uptime -= f64::from(minutes as i32 * 60);
    }

    uptime_str.push_str(&format!("{}s ", uptime as i32));
    uptime_str
}

#[cfg(target_os = "linux")]
fn get_session() -> String {
    match var("XDG_CURRENT_DESKTOP") {
        Ok(session) => String::from(session),
        Err(_e) => String::from("Error Finding Session")
    }
}

#[cfg(target_os = "windows")]
fn get_session() -> String {
    String::from("Windows")
}

fn main() {
    get_time();

    let distro: logo::Logos = match whoami::distro().as_str().split(" ").collect::<Vec<&str>>()[0] {
        "Arch" => logo::Logos::ArchLinux,
        "Ubuntu" => logo::Logos::Ubuntu,
        "Windows" => logo::Logos::Windows,
        _ => logo::Logos::Unknown(whoami::distro())
    };

    println!("            ╭────────╮");
    println!(" {}   \x1b[0;0m│ \x1b[1;31m User \x1b[0;0m│ {}\x1b[1m@\x1b[0m{}", logo::get(&distro)[0], whoami::username(), whoami::hostname());
    println!(" {}   \x1b[0;0m│ \x1b[1;33m   OS \x1b[0;0m│ {}",    logo::get(&distro)[1], whoami::distro());
    println!(" {}   \x1b[0;0m│ \x1b[1;36m   WM \x1b[0;0m│ {}",    logo::get(&distro)[2], get_session());
    println!(" {}   \x1b[0;0m│ \x1b[1;34m   UP \x1b[0;0m│ {}",    logo::get(&distro)[3], get_time());
    println!("            ╰────────╯");
}

