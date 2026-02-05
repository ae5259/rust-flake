use std::path::Path;

use regex::Regex;

fn main() {
    let battery_path = get_battery_path();

    println!("{}", battery_path);
}

fn get_battery_path() -> String {
    let global_path = Path::new("/sys/class/power_supply/");
    let re = Regex::new(r"BAT[0-9]+").expect("Wrong RegEx");
    let mut battery_path = Path::new("");

    for entry in global_path
        .read_dir()
        .expect("Couldn't read /sys/class/power_supply/")
    {
        let entry = entry.expect("Couldn't read the entry");
        let _match = re.is_match(entry.path().as_path().to_str().unwrap());

        println!("Does {} match RegEx? {}", entry.path().display(), _match)
    }

    String::new()
}
