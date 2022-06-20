#[derive(PartialEq, Debug, Clone)]
pub enum Token {
	Illegal,
	Eof,

	Ident(String),
	Int(i64),

	Assign,
	Plus,
	Minus,
	Bang,
	Asterisk,
	Slash,

	LargerThan,
	GreaterThan,
	Equal,
	NotEqual,

	Comma,
	Semicolon,
	Lparen,
	Rparen,
	Lbrace,
	Rbrace,
	Function,
	Let,
	Boolean(bool),
	If,
	Else,
	Return,
}
