
use std::error::Error;
use clap::{App, Arg};
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()>{
    dbg!(config);    
    Ok(())
}

pub fn get_args() -> MyResult<Config>{
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Blake Hachen <blakehachen@ksu.edu>")
        .about("cat for rust")
        .arg(
            Arg::with_name("filename")
                .value_name("FILE")
                .help("name of file to display")
                .multiple(true)
                .default_value("-"),

        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("display all lines in file numbered")
                .conflicts_with("nonblank")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("nonblank")
                .short("b")
                .long("number-nonblank")
                .help("display all lines in file, only lines containing text numbered")
                .takes_value(false),
        )
        .get_matches();
    
    let files: Vec<String> = matches.values_of("filename").unwrap()
        .map(String::from).collect();
    
    Ok(Config{
        files: files,
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("nonblank"),
    })
}
