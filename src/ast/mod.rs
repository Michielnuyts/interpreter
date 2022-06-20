pub trait Node {
	fn token_literal() -> String;
}

pub trait Statement {
	fn statement_node();
}

pub trait Expression {
	fn expression_node();
}
