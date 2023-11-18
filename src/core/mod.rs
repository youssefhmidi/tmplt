mod interpreter;
mod parser;
mod syntax_tree;
mod token;

// short names
pub use interpreter::interpreter::*;
pub use parser::parser::Parser;
pub use syntax_tree::syntax_tree::*;
pub use token::token as Tokens;

use self::Tokens::UnknownSectionError;

pub fn construct_tree(file_data: Vec<String>) -> Result<Tree, UnknownSectionError>{
    Tree::construct(file_data)
}

pub fn construct_interpreter(tree: Tree) -> Interpreter {
    Interpreter::construct(tree)
}
