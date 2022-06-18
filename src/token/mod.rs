#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
	Illegal,
	Eof,
	Ident,
	Int,
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
}

impl TokenType {
	pub fn to_literal(&self) -> String {
		match &self {
			Self::Illegal => String::from("ILLEGAL"),
			Self::Eof => String::from("EOF"),
			Self::Ident => String::from("IDENT"),
			Self::Int => String::from("INT"),
			Self::Assign => String::from("="),
			Self::Plus => String::from("+"),
			Self::Comma => String::from(","),
			Self::Semicolon => String::from(";"),
			Self::Lparen => String::from("("),
			Self::Rparen => String::from(")"),
			Self::Lbrace => String::from("{"),
			Self::Rbrace => String::from("}"),
			Self::Function => String::from("FUNCTION"),
			Self::Let => String::from("LET"),
		}
	}
}

pub struct Token {
	pub r#type: TokenType,
	pub literal: String,
}

impl Token {
	pub fn new(token_type: TokenType) -> Self {
		Self {
			r#type: token_type.clone(),
			literal: token_type.to_literal(),
		}
	}
}
