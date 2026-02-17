use regex::Regex;
use std::fs;
use std::path::Path;

fn main() {
    let a = get_battery_path();
    let p = read_file("capacity", String::from("No battery"));
    let s = read_file("status", String::new());

    let all = get_battery_percentages_float(p.clone());
    println!("{:?}", all);
    if a.is_empty() {
        println!("No battery.");
        return;
    }

    println!(
        "Battery: {}Status: {}",
        p.first().unwrap(),
        s.first().unwrap()
    );
}

fn get_battery_path() -> Vec<fs::DirEntry> {
    let global_path = Path::new("/sys/class/power_supply/");
    let re = Regex::new(r"BAT[0-9]+").expect("Wrong RegEx");

    let entries = match global_path.read_dir() {
        Ok(els) => els,
        Err(_) => return Vec::new(),
    };

    entries
        .filter_map(|el| el.ok())
        .filter(|el| re.is_match(el.path().to_str().unwrap()))
        .collect()
}

fn read_file(file_name: &str, no_entry: String) -> Vec<String> {
    let batteries = get_battery_path();

    batteries
        .iter()
        .map(|el| {
            fs::read_to_string(format!("{}/{}", el.path().display(), file_name))
                .unwrap_or(no_entry.clone())
        })
        .collect()
}

fn get_battery_percentages_float(els: Vec<String>) -> Vec<u8> {
    els.iter().map(|el| el.parse::<u8>().unwrap()).collect()
}
