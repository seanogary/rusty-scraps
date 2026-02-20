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

        #[derive(PartialEq)]
        enum ParserStates {
            Start,
            ConvertStatement,
            CommandInvocation,
            Quantity,
            End,
        }

        #[derive(PartialEq)]
        enum ConvertStatement {
            Idle,
            From,
            Connector,
            To,
            Fulfilled,
        }

        #[derive(PartialEq)]
        enum CommandInvocation {
            Idle,
            Fulfilled,
        }

        let mut currency_from = String::new();
        let mut currency_to = String::new();
        let mut quantity = String::new();
        let mut command = String::new();

        let mut parser_state = ParserStates::Start;
        let mut convert_state = ConvertStatement::Idle;
        let mut command_state = CommandInvocation::Idle;

        let mut words_iterator = input_string.split_whitespace();
        while let Some(word) = words_iterator.next() {

            if let Ok(n) = word.parse::<f64>() {
                quantity = n.to_string();
                continue;
            }
            if word == "convert" {
                if (command_state == CommandInvocation::Fulfilled) {
                    // handle error
                    break;
                }
                if (convert_state == ConvertStatement::Idle) {
                    parser_state = ParserStates::ConvertStatement;
                    convert_state = ConvertStatement::From;
                }
                command = word.to_string();
                command_state = CommandInvocation::Fulfilled;
                continue;
            }

            if word == "USD" && convert_state == ConvertStatement::Idle {
                currency_from = word_str.to_string();
                parser_state = ParserStates::ConvertStatement;
                convert_state = ConvertStatement::Connector;
                continue;
            }

            if (parser_state == ParserStates::ConvertStatement) {
                if (convert_state == ConvertStatement::From) {
                    if (word == "USD") {
                        currency_from = word_str.to_string();
                        convert_state = ConvertStatement::Connector;
                        continue;
                    }

                    else {
                        // handle error here
                        break;
                    }
                }

                if (convert_state == ConvertStatement::Connector) {
                    if (word == "to") {
                        convert_state = ConvertStatement::To;
                        continue;
                    }

                    else {
                        // handle invalid token here
                        break;
                    }
                }

                if (convert_state == ConvertStatement::To) {
                    if (word == "USD") {
                        currency_to = word_str.to_string();
                        convert_state = ConvertStatement::Fulfilled;
                        continue;
                    }

                    else {
                        // handle invalid token here
                        break;
                    }
                }
            }
            
        }
            println!("currency from: {}", currency_from);
            println!("currency to: {}", currency_to);
            println!("quantity = {}", quantity);
            println!("command: {}", command);


    }
}

/*
Exercise log:
-> googled: flush buffer with Rust, found out it was on the Write trait duh
-> googled: method for splitting strings on space Rust
--> found split_whitespace method, returns an iterator hmmm
--> multiple ways to collect that shiz
--> I'll just iterate through it explicitly
*/