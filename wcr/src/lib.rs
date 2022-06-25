use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    words: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>>{
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn get_args() -> MyResult<Config>{
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Blake Hachen <blakehachen@ksu.edu>")
        .about("wc for rust")
        .arg(
            Arg::with_name("filename")
                .value_name("FILE")
                .help("name of file(s) to count")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("display count of words")
                .takes_value(false),
        )
        .get_matches();
    let files = matches.values_of_lossy("filename").unwrap(); 
    
    Ok(Config{
        files: files,
        words: matches.is_present("words"),
    })
}



pub fn run(config: Config) -> MyResult<()>{
    let mut num_words = 0;
    for filename in config.files.iter() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
 
               for line in file.lines() {
                    num_words += line.unwrap().split_whitespace().count();
               }
                   
            }
                
        }
    }
    println!("{} Words in file", num_words);

    Ok(())
}
