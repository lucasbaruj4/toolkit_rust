use std::env;
use std::fs;
use std::process;
use std::error::Error;
use std::process::Command;
use std::process::Output;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });

    if config.file_path != None {
        println!("Showing matches of {:?}", config.query);
        println!("Inside of {:?}", config.file_path.clone().unwrap());
        println!(" ");

        if let Err(e) = run(&config) {
            println!("Application error: {}", e);
            process::exit(1);
        }
    } else {
        println!("Showing matches of {:?}", config.query);
        println!("On the output of {:?}", config.command.clone().unwrap() + " " + &config.argument.clone().unwrap());
        println!(" ");
        let output = Command::new(Option::expect(config.command, "The command name isn't recognized")).arg(Option::expect(config.argument, "There weren't any arguments provided for the command")).output().expect("Failed to execute command");

        if let Err(e) = grep_command_output(config.query, output) {
            println!("Application error: {}", e);
            process::exit(1);
        }

    }
}

struct Config {
    query: String,
    file_path: Option<String>,
    command: Option<String>,
    argument: Option<String>
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments, this program expects at least 2");
        } else if args.len() == 4 {
            let possible_command = match args[2].clone().as_str() {
            "ls" => Some(String::from("ls")),
            _ => None,
            };

            return Ok(Config{
                query: args[1].clone(),
                file_path: None,
                command: possible_command,
                argument: Some(args[3].clone())
            })
        }   

        Ok(Config {
            query: args[1].clone(),
            file_path: Some(args[2].clone()),
            command: None,
            argument: None
        })
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(Option::expect(config.file_path.clone(), "No filepath was provided."))?;
    for line in grep::search_case_insensitive(&config.query, &contents) {
        println!("{line}");
    }
    
    Ok(())
}

fn grep_command_output(query: String, output: Output) -> Result<(), Box<dyn Error>> {
    let contents = String::from_utf8(output.stdout).unwrap();
    for line in grep::search_case_insensitive(&query, &contents) {
        println!("{line}");
    }

    Ok(())
}