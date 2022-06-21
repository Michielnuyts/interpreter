pub trait Node {
	fn token_literal(&self);
}

pub trait Statement: Node {}

pub trait Expression: Node {}

pub struct Program {
	pub statements: Vec<Box<dyn Statement>>,
}
