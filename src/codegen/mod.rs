pub mod gen_c;

use crate::parser::ast::Stmt;
use gen_c::CGen;

pub fn generate_c(stmts: Vec<Stmt>) -> String {
    CGen::new(stmts).generate()
}
