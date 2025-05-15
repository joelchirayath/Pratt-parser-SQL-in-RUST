
mod tokenizer;  // Handles breaking SQL input into tokens
mod pratt;      // Handles expression parsing using Pratt parsing technique
mod parser;     // Main SQL parser logic
mod ast;        // Abstract Syntax Tree definitions

// Import standard I/O modules
use std::io::{self, Write};

// Bring in Tokenizer and Token from tokenizer module
use tokenizer::{Tokenizer, Token};

// Bring in the SQLParser struct from parser module
use parser::SQLParser;

// === Begin custom ParseError definition ===

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEnd, // Input ended unexpectedly
    ExpectedKeyword(String), // A specific keyword was expected but not found
    ExpectedIdentifier, // An identifier (e.g., table name) was expected
    InvalidExpression(String), // Expression syntax was invalid
    UnknownStartOfStatement(String), // Parser saw something unexpected at start
    ExpectedToken(String, Option<Token>), // Expected a token, but got something else
    UnexpectedToken(Token), // A completely unexpected token appeared
    General(String), // A general error message
}

// Implementing error messages
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEnd => write!(f, "Unexpected end of input"),
            ParseError::ExpectedKeyword(k) => write!(f, "Expected keyword: {}", k),
            ParseError::ExpectedIdentifier => write!(f, "Expected an identifier"),
            ParseError::InvalidExpression(e) => write!(f, "Invalid expression: {}", e),
            ParseError::UnknownStartOfStatement(t) => write!(f, "Unknown start of statement: {}", t),
            ParseError::ExpectedToken(expected, actual) => match actual {
                Some(t) => write!(f, "Expected token: {}, but found: {:?}", expected, t),
                None => write!(f, "Expected token: {}, but found end of input", expected),
            },
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {:?}", token),
            ParseError::General(e) => write!(f, "Error: {}", e),
        }
    }
}

// Implement standard error 
impl std::error::Error for ParseError {}

// === End custom ParseError definition ===

fn main() {
    println!("🔷 Welcome to SQL Parser :) ");
    println!("Enter your SQL command (type 'exit' to leave):\n");

    // Main REPL loop
    loop {
        // Print SQL prompt
        print!("sql> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed

        // Read user input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            // If there's an input error, show message and restart loop
            eprintln!("Trouble reading your input — please try again.");
            continue;
        }

        // Trim whitespace and check for exit command
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("👋 Farewell! See you next time, SQL wizard.");
            break;
        }

        // Tokenizing input string
        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = Vec::new();

        // Collect all tokens until EOF
        loop {
            let token = tokenizer.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        // Create parser with token stream
        let mut parser = SQLParser::new(&tokens);

        // Try parsing statement and print result or error
        match parser.parse_statement() {
            Ok(statement) => {
                println!("✅ Your parsed Statement is:\n{:#?}\n", statement);
            }
            Err(e) => {
                eprintln!("❌Error: {}\n", e);
            }
        }
    }
}
