use std::fs::{self, File, create_dir};
use toml::Value;

fn main() {
    // Gets the Cake.toml file
    let toml = fs::read_to_string("Cake.toml")
        .expect("Could not read the Cake.toml file.");
    let toml = toml.parse::<Value>().unwrap();
    
    let filestructure = toml["filestructure"].as_table().unwrap();

    for (key, value) in filestructure {
        create_dir(key.as_str()).unwrap();
    }
} 