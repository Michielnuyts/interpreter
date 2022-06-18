#[derive(PartialEq, Debug, Clone)]
pub enum Token {
	Illegal,
	Eof,

	Ident(String),
	Int(i64),

	Assign,
	Plus,
	Comma,
	Semicolon,
	Lparen,
	Rparen,
	Lbrace,
	Rbrace,
	Function,
	Let,
	Unknown,
}
