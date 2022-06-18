use clap::{App, Arg};

fn main() {
    
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Blake Hachen <blakehachen@ksu.edu>")
        .about("rust echo command")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .long("newline")
                .help("Omit newline at end of string")
                .takes_value(false),
        )
        .get_matches();
    let text = _matches.values_of_lossy("text").unwrap(); 
    let omit_newline = _matches.is_present("omit_newline");
    let ending = if omit_newline {""} else {"\n"};
    print!("{}{}", text.join(" "), ending);
}
