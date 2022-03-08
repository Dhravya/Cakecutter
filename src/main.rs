use std::{
    fs::{self, create_dir, File},
    io::Write,
};

fn main() {
    let path_to_toml = std::env::args().nth(1).unwrap_or(String::from("Cake.toml"));

    // Gets the Cake.toml file
    let toml = fs::read_to_string(path_to_toml).expect("Could not read the Cake.toml file.");
    let toml = match toml.parse::<toml::Value>() {
        Ok(toml) => toml,
        Err(e) => panic!("Could not parse the Cake.toml file: {}", e),
    };

    let filestructure = toml["filestructure"].as_table().unwrap();
    let content = toml["content"].as_table().unwrap();
    let commands = toml["commands"].as_table().unwrap();
    
    if toml.get("metadata").is_some()  {
        let metadata = toml["metadata"].as_table().unwrap();

        let name = metadata["name"].as_str().unwrap();
        let description = metadata["description"].as_str().unwrap();
        let version = metadata["version"].as_str().unwrap();
        let author = metadata["author"].as_str().unwrap();
        
        println!("Using {}", name);
        println!("{}", description);
        println!("Version: {}", version);
        println!("Author: {}", author);
    }
    

    // Creates the directories and files, and fills with content if any
    for (key, value) in filestructure {
        for file in value.as_array().unwrap() {
            let file = file.as_str().unwrap();
            println!("Creating {}", file);

            let mut filepath = String::new();

            // If key is root, create in the current directory
            if key == "root" {
                filepath = format!("{}", file);
            } else {
                let mut created = String::new();
                for folder in key.split("--") {
                    //  We are matching to prevent the panic from happening
                    created += &format!("{}/", folder);
                    match create_dir(created.clone()) {
                        Ok(_) => (),
                        Err(_) => (),
                    }
                }
                filepath = format!("{}/{}", created, file);
            };

            let mut file = File::create(filepath).unwrap();

            // Creates the "content key" which is basically how it's written in the toml file
            let content_key = format!(
                "{}--{}",
                key,
                value.as_array().unwrap()[0]
                    .as_str()
                    .unwrap()
                    .replace(".", "-")
            );

            // Checks if content key exists
            if content.contains_key(content_key.as_str()) {
                let content = content[content_key.as_str()].as_str().unwrap();
                file.write_all(content.as_bytes()).unwrap();
            };
        }
    }

    // Runs the commands one by one according to the number
    // I've done this by iterating over numbers from 1 to the number of commands

    for n in 1..commands.len() + 1 {
        println!("Running command {}", &n.to_string());
        let command = commands[&n.to_string()].as_array().unwrap();

        // Runs the command
        std::process::Command::new(command[0].as_str().unwrap())
            .args(
                command[1..]
                    .iter()
                    .map(|x| x.as_str().unwrap())
                    .collect::<Vec<&str>>(),
            )
            .spawn()
            .expect("Could not run the command");
    }
}
