use std::io;

use crate::{lexer::Lexer, token::Token};

const PROMPT: &str = ">> ";

pub fn start() {
	loop {
		println!("{}", PROMPT);

		let mut line = String::new();

		io::stdin()
			.read_line(&mut line)
			.expect("Failed to read line");

		let mut l = Lexer::new(line.as_str());

		loop {
			let token = l.next_token();

			match token {
				Token::Eof => break,
				_ => {
					print!("{:?} ", token);
				}
			}
		}
	}
}
