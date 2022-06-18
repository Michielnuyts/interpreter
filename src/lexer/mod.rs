use crate::token::{self, Token, TokenType};

pub struct Lexer {
	pub input: String,
	pub position: i64,
	pub read_position: usize,
	pub ch: u8,
}

impl Lexer {
	pub fn new(input: String) -> Self {
		let mut lexer = Self {
			input,
			position: 0,
			read_position: 0,
			ch: 0,
		};
		lexer.read_char();

		lexer
	}
	pub fn next_token(&mut self) -> token::Token {
		let tok = match self.ch {
			b'=' => Token::new(TokenType::Assign),
			b';' => Token::new(TokenType::Semicolon),
			b'(' => Token::new(TokenType::Lparen),
			b')' => Token::new(TokenType::Rparen),
			b',' => Token::new(TokenType::Comma),
			b'+' => Token::new(TokenType::Plus),
			b'{' => Token::new(TokenType::Lbrace),
			b'}' => Token::new(TokenType::Rbrace),
			_ => Token::new(TokenType::Eof),
		};

		self.read_char();

		tok
	}
	pub fn read_char(&mut self) {
		if self.read_position >= self.input.len() {
			self.ch = 0;
		} else {
			self.ch = self.input.as_bytes()[self.read_position];
			self.read_position += 1;
		}
	}
}

#[test]
fn test_next_token() {
	let input = String::from("=+(){},;");

	let tests = [
		(TokenType::Assign, "="),
		(TokenType::Plus, "+"),
		(TokenType::Lparen, "("),
		(TokenType::Rparen, ")"),
		(TokenType::Lbrace, "{"),
		(TokenType::Rbrace, "}"),
		(TokenType::Comma, ","),
		(TokenType::Semicolon, ";"),
		(TokenType::Eof, "EOF"),
	];

	let mut l = Lexer::new(input);

	for ct in tests {
		let token = l.next_token();

		assert_eq!(token.r#type, ct.0);
		assert_eq!(token.literal, ct.1);
	}
}
