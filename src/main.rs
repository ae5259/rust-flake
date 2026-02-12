use regex::Regex;
use std::fs;
use std::path::Path;

fn main() {
    let _ = get_battery_path();
    let p = read_file("capacity", String::from("No battery"));
    let s = read_file("status", String::new());

    println!("{:?}", p);
    println!("{:?}", s);
}

fn get_battery_path() -> Vec<fs::DirEntry> {
    let global_path = Path::new("/sys/class/power_supply/");
    let re = Regex::new(r"BAT[0-9]+").expect("Wrong RegEx");

    global_path
        .read_dir()
        .unwrap()
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
// fn get_battery_percentages() -> Vec<String> {
//     let batteries = get_battery_path();
//
//     batteries
//         .iter()
//         .map(|el| {
//             fs::read_to_string(format!("{}/capacity", el.path().display()))
//                 .unwrap_or(String::from("No battery"))
//         })
//         .collect()
// }
//
// fn get_battery_status() -> Vec<String> {
//     let batteries = get_battery_path();
//
//     batteries
//         .iter()
//         .map(|el| {
//             fs::read_to_string(format!("{}/status", el.path().display()))
//                 .unwrap_or(String::from(""))
//         })
//         .collect()
// }
