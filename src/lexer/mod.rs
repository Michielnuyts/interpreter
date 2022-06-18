use crate::token::Token;

pub struct Lexer<'a> {
	pub input: &'a str,
	pub position: usize,
	pub read_position: usize,
	pub ch: u8,
}

impl<'a> Lexer<'a> {
	pub fn new(input: &'a str) -> Self {
		let mut lexer = Self {
			input,
			position: 0,
			read_position: 0,
			ch: 0,
		};
		lexer.read_char();

		lexer
	}
	pub fn next_token(&mut self) -> Token {
		let tok = match self.ch {
			b'=' => Token::Assign,
			b';' => Token::Semicolon,
			b'(' => Token::Lparen,
			b')' => Token::Rparen,
			b',' => Token::Comma,
			b'+' => Token::Plus,
			b'{' => Token::Lbrace,
			b'}' => Token::Rbrace,
			b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_identifier(),
			0 => Token::Eof,
			_ => Token::Illegal,
		};

		self.read_char();

		tok
	}
	pub fn read_char(&mut self) {
		if self.read_position >= self.input.len() {
			self.ch = 0;
		} else {
			self.ch = self.input.as_bytes()[self.read_position];
		}

		self.position = self.read_position;
		self.read_position += 1;
	}
	pub fn read_identifier(&mut self) -> Token {
		let start_position = self.position;

		loop {
			match self.ch {
				b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
					self.read_char();
				}
				_ => {
					break;
				}
			}
		}

		let literal = &self.input[start_position..self.position];

		match literal {
			"fn" => Token::Function,
			"let" => Token::Let,
			_ => Token::Ident(String::from(literal)),
		}
	}
}

#[test]
fn test_next_token() {
	let input = "=+(){},;";

	let tests = [
		Token::Assign,
		Token::Plus,
		Token::Lparen,
		Token::Rparen,
		Token::Lbrace,
		Token::Rbrace,
		Token::Comma,
		Token::Semicolon,
		Token::Eof,
	];

	let mut l = Lexer::new(input);

	for ct in tests {
		let token = l.next_token();

		assert_eq!(token, ct);
	}
}

#[test]
fn test_next_token_expanded() {
	let input = "let five = 5;
let ten = 10;
	
let add = fn(x, y) {
	x + y;
};
	
let result = add(five, ten);";

	let tests = [
		Token::Let,
		Token::Ident(String::from("five")),
		Token::Assign,
		Token::Int(5),
		Token::Semicolon,
		Token::Let,
		Token::Ident(String::from("ten")),
		Token::Assign,
		Token::Int(10),
		Token::Semicolon,
		Token::Let,
		Token::Ident(String::from("add")),
		Token::Assign,
		Token::Function,
		Token::Lparen,
		Token::Ident(String::from("x")),
		Token::Comma,
		Token::Ident(String::from("y")),
		Token::Rparen,
		Token::Lbrace,
		Token::Ident(String::from("x")),
		Token::Plus,
		Token::Ident(String::from("y")),
		Token::Semicolon,
		Token::Rbrace,
		Token::Semicolon,
		Token::Let,
		Token::Ident(String::from("result")),
		Token::Assign,
		Token::Ident(String::from("add")),
		Token::Lparen,
		Token::Ident(String::from("five")),
		Token::Comma,
		Token::Ident(String::from("ten")),
		Token::Rparen,
		Token::Semicolon,
		Token::Eof,
	];

	let mut l = Lexer::new(input);

	for ct in tests {
		let token = l.next_token();

		assert_eq!(token, ct);
	}
}
