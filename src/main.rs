use regex::Regex;
use std::{fs, process::Command};

fn main() {
    let re = Regex::new(r"(?i)godot[\w.-]*\.exe").unwrap();
    let file_names: Vec<String> = fs::read_dir(".")
        .expect("Failed to read dir")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().into_string().ok()?;

            if re.is_match(&name) && !name.contains("console") && !name.contains("opener") {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    let handler = std::thread::spawn(|| {
        for entry in file_names {
            let mut command = String::from("start ");
            command.push_str(&entry);
            Command::new(format!("./{}", entry))
                .spawn()
                .expect("Failed to execute Godot");
        }
    });
    handler.join().unwrap();
}
