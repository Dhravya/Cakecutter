use std::{
    fs::{self, create_dir, File},
    io::Write,
};
use toml::Value;

fn main() {
    // Gets the Cake.toml file
    let toml = fs::read_to_string("Cake.toml").expect("Could not read the Cake.toml file.");
    let toml = match toml.parse::<Value>() {
        Ok(toml) => toml,
        Err(e) => panic!("Could not parse the Cake.toml file: {}", e),
    };

    let filestructure = toml["filestructure"].as_table().unwrap();
    let content = toml["content"].as_table().unwrap();

    for (key, value) in filestructure {
        create_dir(key.as_str()).unwrap();

        for file in value.as_array().unwrap() {
            let file = file.as_str().unwrap();
            let mut file = File::create(format!("{}/{}", key, file)).unwrap();

            let content_key = format!(
                "{}__{}",
                key,
                value.as_array().unwrap()[0].as_str().unwrap()
            );
            let content = content[content_key.as_str()].as_str().unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }
    }
}
