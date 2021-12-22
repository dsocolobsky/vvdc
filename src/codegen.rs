use crate::parser::{BoxExpression, ExpressionType};

struct Compiler {
    code: String,
    expressions: Vec<BoxExpression>,
}

impl Compiler {
    fn new(expressions: Vec<BoxExpression>) -> Compiler {
        Compiler {
            code: String::from(""),
            expressions: expressions,
        }
    }

    fn compile(&mut self) {
        self.build_prelude();
        for expression in self.expressions.iter() {
            self.emit_code_for_expression(*expression);
        }
    }

    fn asm_write(&mut self, line: &str) {
        self.code.push_str(&format!("{}\n", line).to_string());
    }

    fn asm_mov(&mut self, lhs: &str, rhs: &str) {
        self.asm_write(format!("mov {}, {}", lhs, rhs).as_str());
    }

    fn asm_add(&mut self, lhs: &str, rhs: &str) {
        self.asm_write(format!("add {}, {}", lhs, rhs).as_str());
    }

    fn build_prelude(&mut self) {
        self.asm_write("section .text");
        self.asm_write("global _start");
        self.asm_write("_start:");
    }

    fn emit_code_for_expression(&mut self, expression: BoxExpression) {
        match expression.get_type() {
            ExpressionType::Number => todo!(),
            ExpressionType::Infix => todo!(),
            ExpressionType::String => todo!(),
            ExpressionType::Identifier => todo!(),
            ExpressionType::Prefix => todo!(),
            ExpressionType::Return => {
                self.emit_code_for_return(expression);
            }
        }
    }

    fn emit_code_for_negation(&mut self, expression: BoxExpression) {
        let right_side = expression.right().unwrap();
        match right_side.get_type() {
            ExpressionType::Number => {
                let val = if right_side.literal().unwrap() == "false" { 0 } else { 1 };
                self.asm_write(format!("mov rbx, {}", val).as_str());
            },
            ExpressionType::Infix => todo!(),
            ExpressionType::String => todo!(),
            ExpressionType::Identifier => todo!(),
            ExpressionType::Prefix => {
                self.emit_code_for_negation(right_side);
                self.asm_write("xor rbx, 1");
            },
            ExpressionType::Return => panic!("can not prefix a return"),
        }
    }
    
    fn emit_code_for_addition(&mut self, expression: BoxExpression, first_iteration: bool) {
        if let Some(left_side) = expression.left() {
            match left_side.get_type() {
                ExpressionType::Number => {
                    let st = left_side.literal().unwrap();
                    if first_iteration {
                        self.asm_mov("rbx", &*st);
                    } else {
                        self.asm_add("rbx", &*st);
                    }
                },
                ExpressionType::Infix => todo!(),
                ExpressionType::String => todo!(),
                ExpressionType::Identifier => todo!(),
                ExpressionType::Prefix => todo!(),
                ExpressionType::Return => panic!("can not prefix a return"),
            }
        } else {
            self.asm_add("rbx", &expression.literal().unwrap());
        }

        if let Some(right_side) = expression.right() {
            match right_side.get_type() {
                ExpressionType::Number => {
                    self.asm_add("rbx", &*right_side.literal().unwrap());
                },
                ExpressionType::Infix => {
                    self.emit_code_for_addition(right_side.left().unwrap(), false);
                    self.emit_code_for_addition(right_side.right().unwrap(), false);
                },
                ExpressionType::String => todo!(),
                ExpressionType::Identifier => todo!(),
                ExpressionType::Prefix => todo!(),
                ExpressionType::Return => panic!("can not prefix a return"),
            }
        }
    }

    fn emit_code_for_return(&mut self, expression: BoxExpression) {
        let right_side = expression.right().unwrap();
        match right_side.get_type() {
            ExpressionType::Number => {
                self.asm_mov("rbx", &*right_side.literal().unwrap());
            },
            ExpressionType::Infix => {
                self.emit_code_for_addition(right_side, true);
            },
            ExpressionType::String => todo!(),
            ExpressionType::Identifier => todo!(),
            ExpressionType::Prefix => {
                self.emit_code_for_negation(right_side); // already leaves val in rbx
            },
            ExpressionType::Return => {
                panic!("can not return a return");
            },
        }
        self.asm_mov("rax", "1");
        self.asm_write("int 0x80");
    }
}

pub fn generate_code(expressions: Vec<BoxExpression>) -> String {
    let mut compiler = Compiler::new(expressions);
    compiler.compile();
    compiler.code
}
