# rust-solutions

Solutions to problems from Command-Line Programming in Rust.

## Chapter 1

First chapter focuses on the installation of rust, getting to use cargo
commands and learning the basics of crates and macros.
* cargo build, cargo run
* hello/tests contains macros for writing tests (cargo test)
* main.rs contains rust starter code for simple hello world

## Chapter 2

Second chapter involves use of `clap` to process command line arguments in
order to create a clone of shell command echo. echor will take an Optional flag
before mandatory text and print said text on the command line
* use of App to create echor with suitable metadata
* use of Arg to collect command line arguments `cargo run [arg] TEXT-TO-PRINT`
* echor/tests contains test macros
..* Learned how to create type
..* Understand the use of Fat Pointers to gather length of object as well as
a pointer
..* write helper function to perform many tests
..* write shell script to create files containing real echo output
 
