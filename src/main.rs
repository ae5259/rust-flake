use regex::Regex;
use std::fs;
use std::path::Path;

fn main() {
    let battery_path = get_battery_path();

    println!("{}", battery_path);
}

fn get_battery_path_attempt() -> String {
    let global_path = Path::new("./data/");
    let re = Regex::new(r"BAT[0-9]+").expect("Wrong RegEx");

    let mut batteries: Vec<&str> = Vec::new();

    global_path.read_dir().map(|el| {
        // if re.is_match(haystack)
    });

    for entry in global_path
        .read_dir()
        .expect("Couldn't read /sys/class/power_supply/")
    {
        let entry = entry.expect("Couldn't read the entry");
        let _match = re.is_match(entry.path().to_str().unwrap());

        if _match {
            // batteries.push(entry.path().to_str().unwrap());
        }

        println!("Does {} match RegEx? {}", entry.path().display(), _match)
    }

    String::new()
}

fn get_battery_path() -> String {
    let global_path = Path::new("./data/");
    let re = Regex::new(r"BAT[0-9]+").expect("Wrong RegEx");

    let mut batteries: Vec<&str> = Vec::new();

    for entry in global_path
        .read_dir()
        .expect("Couldn't read /sys/class/power_supply/")
    {
        let entry = entry.expect("Couldn't read the entry");
        let _match = re.is_match(entry.path().to_str().unwrap());

        if _match {
            // batteries.push(entry.path().to_str().unwrap());
        }

        println!("Does {} match RegEx? {}", entry.path().display(), _match)
    }

    String::new()
}

fn get_battery_percentage() -> String {
    fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .unwrap_or(String::from("No battery"))
}

fn get_battery_status() -> String {
    fs::read_to_string("/sys/class/power_supply/BAT0/status").unwrap_or(String::from(""))
}
