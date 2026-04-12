use chapter_12_03::search;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // let config = Config::new(&args);

    // println!("Search for {}", config.query);
    // println!("In file {}", config.file_path);

    // let contents = std::fs::read_to_string(config.file_path)
    //     .expect("Something went wrong reading the file");

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     std::process::exit(1);
    // });

    let config = Config::new(&args);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;

    // println!("With text:\n{contents}");
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() != 3 {
            panic!("Usage: <query> <file_path>");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// fn parse_config(args: &[String]) -> Config {
//     if args.len() != 3 {
//         panic!("Usage: <query> <file_path>");
//     }

//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }