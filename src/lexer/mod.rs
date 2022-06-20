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
		self.skip_whitespace();

		let tok = match self.ch {
			b'=' => {
				if self.peek_char() == b'=' {
					self.read_char();
					Token::Equal
				} else {
					Token::Assign
				}
			}
			b'!' => {
				if self.peek_char() == b'=' {
					self.read_char();
					Token::NotEqual
				} else {
					Token::Bang
				}
			}
			b';' => Token::Semicolon,
			b'(' => Token::Lparen,
			b')' => Token::Rparen,
			b',' => Token::Comma,
			b'+' => Token::Plus,
			b'-' => Token::Minus,
			b'*' => Token::Asterisk,
			b'/' => Token::Slash,
			b'{' => Token::Lbrace,
			b'}' => Token::Rbrace,
			b'<' => Token::LargerThan,
			b'>' => Token::GreaterThan,
			b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
				return self.read_identifier();
			}
			b'0'..=b'9' => {
				return self.read_number();
			}
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
	pub fn peek_char(&self) -> u8 {
		if self.read_position >= self.input.len() {
			0
		} else {
			self.input.as_bytes()[self.read_position]
		}
	}
	pub fn skip_whitespace(&mut self) {
		while let b' ' | b'\t' | b'\n' | b'\r' = self.ch {
			self.read_char();
		}
	}
	pub fn read_identifier(&mut self) -> Token {
		let start_position = self.position;

		while let b'a'..=b'z' | b'A'..=b'Z' | b'_' = self.ch {
			self.read_char();
		}

		let literal = &self.input[start_position..self.position];

		match literal {
			"fn" => Token::Function,
			"let" => Token::Let,
			"true" => Token::Boolean(true),
			"false" => Token::Boolean(false),
			"if" => Token::If,
			"else" => Token::Else,
			"return" => Token::Return,
			_ => Token::Ident(String::from(literal)),
		}
	}
	pub fn read_number(&mut self) -> Token {
		let start_position = self.position;

		while let b'0'..=b'9' = self.ch {
			self.read_char();
		}

		let literal = &self.input[start_position..self.position];

		Token::Int(literal.parse::<i64>().unwrap())
	}
}

#[test]
fn test_next_token() {
	let input = r#"
	let five = 5;
	let ten = 10;

	let add = fn(x, y) {
	  x + y;
	};

	let result = add(five, ten);
	!-/*5;
	5 < 10 > 5;

	if (5 < 10) {
		return true;
	} else {
		return false;
	}

	10 == 10;
	10 != 9;
	"#;

	let tests = [
		// --- line 1
		Token::Let,
		Token::Ident(String::from("five")),
		Token::Assign,
		Token::Int(5),
		Token::Semicolon,
		// --- line 2
		Token::Let,
		Token::Ident(String::from("ten")),
		Token::Assign,
		Token::Int(10),
		Token::Semicolon,
		// --- line 3
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
		// --- line 4
		Token::Ident(String::from("x")),
		Token::Plus,
		Token::Ident(String::from("y")),
		Token::Semicolon,
		// --- line 5
		Token::Rbrace,
		Token::Semicolon,
		// --- line 6
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
		// line 7
		Token::Bang,
		Token::Minus,
		Token::Slash,
		Token::Asterisk,
		Token::Int(5),
		Token::Semicolon,
		// line 8
		Token::Int(5),
		Token::LargerThan,
		Token::Int(10),
		Token::GreaterThan,
		Token::Int(5),
		Token::Semicolon,
		Token::If,
		Token::Lparen,
		Token::Int(5),
		Token::LargerThan,
		Token::Int(10),
		Token::Rparen,
		Token::Lbrace,
		Token::Return,
		Token::Boolean(true),
		Token::Semicolon,
		Token::Rbrace,
		Token::Else,
		Token::Lbrace,
		Token::Return,
		Token::Boolean(false),
		Token::Semicolon,
		Token::Rbrace,
		Token::Int(10),
		Token::Equal,
		Token::Int(10),
		Token::Semicolon,
		Token::Int(10),
		Token::NotEqual,
		Token::Int(9),
		Token::Semicolon,
		Token::Eof,
	];

	let mut l = Lexer::new(input);

	for ct in tests {
		let token = l.next_token();

		assert_eq!(token, ct);
	}
}
