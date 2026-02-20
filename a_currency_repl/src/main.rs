/*

Goal: make REPL that converts currency using a currency conversion API 

fn main:
-> contains REPL loop
-> parses inputs for args by splitting on space
-> returns informative error if command is trash
-> calls synchronous function to make request with args

fn convert_currency():
-> makes blocking get request with reqwest
-> returns result type that contains the converted currency or an informative error

Notes:
- use explicit matching on results, no sugar
- use custom enum for E in the Result<T, E> returned by convert_currency()

*/

use std::io::Write;



fn main() {
    loop {
        print!("> ");
        match std::io::stdout().flush() {
            Ok(_) => {},
            Err(e) => panic!("{:?}", e),
        };

        // initialize mutable String container for storing user input
        let mut input_string = String::new();

        // write user input into input_string
        match std::io::stdin().read_line(&mut input_string) {
            Ok(_) => {},
            Err(e) => panic!("{:?}", e),
        };

        // convert N USD to EUR
        // list currencies


        let currentState: ParserState = ParserState::Idle;

        let mut words_iterator = input_string.split_whitespace();
        while let Some(word) = words_iterator.next() {
            let word_str: &str = word.as_ref();
            
            // match stuff or something
            
        }

    }
}

/*
Exercise log:
-> googled: flush buffer with Rust, found out it was on the Write trait duh
-> googled: method for splitting strings on space Rust
--> found split_whitespace method, returns an iterator hmmm
--> multiple ways to collect that shiz
*/