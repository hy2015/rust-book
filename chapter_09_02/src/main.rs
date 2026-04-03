use std::fs::{self, File};
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");

    // let username = read_username_from_file().expect("Could not read username from the file");
    // println!("Username: {}", username);

    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn recoverable() -> Result<(), String> {
    let file = File::open("hello.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);                
        }
    };

    let greeting_file = File::open("hello.txt");

    let greeting_file = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let greeting_file = File::open("hello.txt").unwrap();

    Ok(())
}

fn read_username_from_file_0() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_1() -> Result<String, std::io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_2() -> Result<String, std::io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
fn read_username_from_file() -> Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}