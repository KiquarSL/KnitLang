use crate::parser::ast::Stmt;
use std::fmt;

pub struct CGen {
    out: String,
    stmts: Vec<Stmt>,
}

impl CGen {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self {
            out: String::new(),
            stmts,
        }
    }

    pub fn generate(&mut self) -> String {
        self.out.push_str("#include <stdint.h>\n");
        self.out.push_str("#include <stdbool.h>\n");
        for stmt in &self.stmts {
            self.out.push_str(&format!("{stmt}\n"));
        }
        self.out.clone()
    }

    fn to_c_type(ty: String) -> String {
        match ty.as_str() {
            "i8" => "int8_t".to_string(),
            "i16" => "int16_t".to_string(),
            "i32" => "int32_t".to_string(),
            "i64" => "int64_t".to_string(),
            "u8" => "uint8_t".to_string(),
            "u16" => "uint16_t".to_string(),
            "u32" => "uint32_t".to_string(),
            "u64" => "uint64_t".to_string(),
            "f32" => "float".to_string(),
            "f64" => "double".to_string(),
            "string" => "char*".to_string(),
            _ => ty,
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::NewVar {
                mutable,
                name,
                type_name,
                value,
            } => {
                let const_kw = if *mutable { "" } else { "const " };
                if let Some(val) = value {
                    write!(
                        f,
                        "{}{} {} = {};",
                        const_kw,
                        CGen::to_c_type(type_name.to_string()),
                        name,
                        val
                    )
                } else {
                    write!(f, "{}{} {};", const_kw, type_name, name)
                }
            }
            Stmt::Assign { name, value } => {
                write!(f, "{name} = {value};")
            }
            Stmt::If {
                cond,
                then_body,
                else_body,
            } => {
                if let Some(else_b) = else_body {
                    write!(f, "if ({cond}) {{ {then_body} }} else {{ {else_b} }}")
                } else {
                    write!(f, "if ({cond}) {{ {then_body} }}")
                }
            }
            Stmt::While { cond, body } => {
                write!(f, "while ({cond}) {{ {body} }}")
            }
            Stmt::Fn {
                name,
                args,
                return_type,
                body,
            } => {
                let ret_type = match return_type {
                    Some(t) => CGen::to_c_type(t.to_string()),
                    None => "void".to_string(),
                };
                let args_str = args
                    .iter()
                    .map(|(name, ty)| format!("{} {name}", CGen::to_c_type(ty.to_string())))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{ret_type} {name}({args_str}) {{ {body} }}")
            }
            Stmt::Call { name, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{name}({args_str});")
            }
            Stmt::Block(block) => {
                let stmts_str = block
                    .stmts
                    .iter()
                    .map(|stmt| format!("    {stmt}"))
                    .collect::<Vec<_>>()
                    .join("\n");
                write!(f, "{{ {} }}", stmts_str)
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr {
                    write!(f, "return {e};")
                } else {
                    write!(f, "return;")
                }
            }
            Stmt::Pkg(_) => {
                write!(f, "")
            }
            Stmt::Use(package) => {
                let path = package.join("/");
                write!(f, "#include <{path}.h>")
            }
        }
    }
}
