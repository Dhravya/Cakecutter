use std::{
    fs::{self, create_dir, File},
    io::Write,
};

fn main() {
    // Gets the Cake.toml file
    let toml = fs::read_to_string("Cake.toml").expect("Could not read the Cake.toml file.");
    let toml = match toml.parse::<toml::Value>() {
        Ok(toml) => toml,
        Err(e) => panic!("Could not parse the Cake.toml file: {}", e),
    };

    let filestructure = toml["filestructure"].as_table().unwrap();
    let content = toml["content"].as_table().unwrap();

    for (key, value) in filestructure {
        for file in value.as_array().unwrap() {
            let file = file.as_str().unwrap();

            let mut filepath = String::new();
            println!(
                "Creating file at {current}/{file} {filepath}",
                filepath = filepath,
                current = key,
                file = file
            );
            if key == "root" {
                filepath = format!("{}", file);
            } else {
                filepath = format!("{}/{}", key, file);
                match create_dir(key) {
                    Ok(_) => (),
                    Err(_) => (),
                };
            };

            let mut file = File::create(filepath).unwrap();

            let content_key = format!(
                "{}--{}",
                key,
                value.as_array().unwrap()[0].as_str().unwrap().replace(".", "-")
            );
            
            // Checks if content key exists
            if content.contains_key(content_key.as_str()) {
                let content = content[content_key.as_str()].as_str().unwrap();
                file.write_all(content.as_bytes()).unwrap();
            };
        }
    }
}
