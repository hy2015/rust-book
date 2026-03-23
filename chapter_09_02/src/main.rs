use std::fs::File;

fn main() {
    // let file = File::open("hello.txt");

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // };

    // let greeting_file = File::open("hello.txt");

    // let greeting_file = match greeting_file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         std::io::ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == std::io::ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
}
