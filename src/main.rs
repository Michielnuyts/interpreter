mod lexer;
mod token;

fn main() {
	println!("Hello, world! {}", token::TokenType::Assign.to_literal());
}
