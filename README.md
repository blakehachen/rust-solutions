# rust-solutions

Solutions to problems from Command-Line Programming in Rust.

## Chapter 1

First chapter focuses on the installation of rust, getting to use cargo
commands and learning the basics of crates and macros.
1. cargo build, cargo run
2. hello/tests contains macros for writing tests (cargo test)
3. main.rs contains rust starter code for simple hello world

## Chapter 2

Second chapter involves use of `clap` to process command line arguments in
order to create a clone of shell command echo. echor will take an Optional flag
before mandatory text and print said text on the command line
1. use of App to create echor with suitable metadata
2. use of Arg to collect command line arguments `cargo run [arg] TEXT-TO-PRINT`
3. echor/tests contains test macros
    * Learned how to create type
    * Understand the use of Fat Pointers to gather length of object as well as
a pointer
    * write helper function to perform many tests
    * write shell script to create files containing real echo output

## Chapter 3

Third chapter involved the use of `clap` to process command line arguments in
order to create a clone of shell command cat. catr will take an optional flag
for printing each line within a file numbered, non-blank lines only numbered,
or no numbers at all. The catr implementation has the ability to display
multiple files with a running line count
1. Used match keyword for better control flow when default parameter is tested
2. Used BufReader and iterator to examine lines of individual file
3. Learned more clap functions (multiple, required) `cargo run -- [FLAG] [FILES]`
4. Practiced test-driven development as seen in catr/tests
    * various test macros compare output of GNU cat command to catr
    * checks for file open errors and default arg
    * all possible stress tests for a simple cat command (multiple text files
      with cat output to compare to)

## Chapter 4

Fourth chapter focused on reimplementing GNU command head, it has optional
flags that take arguments. The command takes a list of files `[FILES] | cargo run -- [FLAG]` the two flags that I implemented were bytes and lines.
1. transpose Option to obtain result and unwrap to get value of result, this
   was the practice that I used to error check the command in order to make
sure a non negative integer was specified.
2. When reading the buffer both bytes and lines needed to be taken into
   account.
    * if bytes were specified I would use the take function to specify the
      amount of the file I wanted to be read the bytes into a vector and then
print the buffer accounting for utf8 encoding
    * if lines were specified then I would simply read lines up to the
      specified line count printing each line along the way.
3. mapping a function to a list turned out to be very helpful during this
   chapter.
