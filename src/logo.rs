macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub enum Logos {
    ArchLinux,
    Ubuntu,
    Unknown(String)
}

pub fn get(logo: &Logos) -> Vec<String> {
    match logo {
        Logos::ArchLinux => vec_of_strings![
            "\x1b[0;36m   /\\   ",
            "\x1b[0;36m  /\\ \\  ",
            "\x1b[0;36m / __ \\ ",
            "\x1b[0;36m/_|  |_\\"],
        Logos::Ubuntu => vec_of_strings![
            "\x1b[0;31m /‾‾〇\\ ",
            "\x1b[0;31m〇     |",
            "\x1b[0;31m|      |",
            "\x1b[0;31m \\__〇/ "],
        Logos::Unknown(distro) => vec_of_strings!["Distro not implemented", distro, "", ""]
    }
}
