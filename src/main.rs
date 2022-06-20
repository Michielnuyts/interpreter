mod ast;
mod lexer;
mod repl;
mod token;

fn main() {
	println!("Hello, this is the Monkey programming language!\n");
	println!("Type in some commands\n");

	repl::start();
}
