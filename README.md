<div align="center">
<img src = "./images/cakecutter.png" width="400">
<h1 align="center">Cakecutter</h1>
<br>
Create projects from pre-built cakes (templates)! Supports files, packages, content, running commands and more!
</div>

***

Cakecutter is a utility tool that quickly sets up a project from a pre-built template. 

All template files are in `.toml` format, which means they are easy to edit and share with others.

Here's a python project demo

Notice how it created the files and ran the VENV command to initialise the virtual environment, and started installing the dependencies
![Demo](https://us-east-1.tixte.net/uploads/img.dhravya.dev/l0ikgtw4f0a.gif)

## Features
- Create projects from pre-built cakes (templates) and make your own!
- Supports all languages (Python, Js, Rust, Go, you name it.)
- Cross-platform
- Super fast ⚡
- Get Cakes from github or use local Cakefiles

### Installation
```
cargo install cakecutter 
```

### Usage
```
cakecutter [TEMPLATE_NAME]
```
You can also use cakes from github (Provided they have a `Cake.toml` file in the root directory of the repository):
```
cakecutter https://github.com/dhravya/cakecutter
```

## Making your own Cakefile
It's really easy to make your own cakefile. There are 4 main sections:
```toml
[metadata]

[filestructure]

[content]

[commands]
```

### Basic rules
Since TOML doesn't support `.` and `/` as keys, we use `-` and `--` instead.

so instead of `main.py`, we use `main-py` and instead of `src/main.py` we use `src--main-py`

#### Metadata
Metadata is optional, but when you include it, make sure to include the following:
- `name`: The name of your cake
- `version`: Cake version
- `description`: What the cake is for
- `author`: The author of Cake

#### File structure
The file structure is where you define the structure of your project. 

To include files in the current directory, put them in the `root` list
`root = [".gitignore", "Cargo.toml", "README.md", "LICENSE"]`

For every other directory, use the following syntax:
`directory_name = [file1, file2]`
So for this repository, it looks something like
```toml
root = [".gitignore", "Cargo.toml", "README.md", "LICENSE"]
examples = ["Python.toml"]
```

#### Content
Content is where you define the content of your files.
It's pretty simple, just write the name of file (following the  basic rules) and the content of the files after it
```toml
[content]
src--main-py = """
print("Hello World")
"""
```
This will fill in the file `src/main.py` with the content of the string.

#### Commands
These are the commands that run when a cake is made (Stuff like installing dependencies)

All keys here should be numbers starting from one and increasing progressively.

Commands should be written as if written in a Dockerfile

Here's an examples:
```toml
[commands]
1 = ['python', '-m', 'venv', 'venv']
2 = ['pip', 'install', '-r', 'requirements.txt']
3 = ['venv/Scripts/activate']
3 = ['venv/Scripts/python.exe', 'src/main.py']
```


### License
This project is licensed under the mit license
### Show your support
Leave a ⭐ if you like this project

***
Readme made with 💖 using [README Generator by Dhravya Shah](https://github.com/Dhravya/readme-generator)
