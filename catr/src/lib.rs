use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>>{
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}


pub fn run(config: Config) -> MyResult<()>{    
    let mut num_str = 0;
    for filename in config.files {
        match open(&filename) {
           Err(err) => eprintln!("{}: {}", filename, err),
           Ok(file) => {
                for line in file.lines() { 
                    let real_line = line.unwrap(); 
                    if config.number_lines {
                        num_str += 1;
                        println!("{:>6}\t{}", num_str, real_line);
              
                    }else if config.number_nonblank_lines {
                        if real_line.trim().is_empty(){
                            println!();
                        }else{
                            num_str += 1;
                            println!("{:>6}\t{}", num_str, real_line);
                        } 
                    }else{
                        println!("{}", real_line);
                    }
                }
           }
       }
    }
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
