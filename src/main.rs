use lexer::Lexer;
use token::Token;

mod lexer;
mod token;

fn main() {
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
