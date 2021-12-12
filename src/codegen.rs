use crate::parser::Expression;

struct Compiler {
    code: String,
    expressions: Vec<Expression>,
}

impl Compiler {
    fn new(expressions: Vec<Expression>) -> Compiler {
        Compiler {
            code: String::from(""),
            expressions: expressions,
        }
    }

    fn compile(&mut self) {
        self.build_prelude();
        for expression in self.expressions.clone() {
            self.emit_code_for_expression(&expression);
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

    fn emit_code_for_expression(&mut self, expression: &Expression) {
        match expression.expression_type {
            crate::parser::ExpressionType::LiteralExpression => todo!(),
            crate::parser::ExpressionType::PrefixExpression => todo!(),
            crate::parser::ExpressionType::InfixExpression => todo!(),
            crate::parser::ExpressionType::ReturnExpression => {
                self.emit_code_for_return(expression);
            }
        }
    }

    fn emit_code_for_negation(&mut self, expression: &Expression) {
        let right_side = expression.right_side();
        match right_side.expression_type {
            crate::parser::ExpressionType::LiteralExpression => {
                let val = right_side.token.literal_as_boolean();
                let val = if val { 0 } else { 1 };
                self.asm_write(format!("mov rbx, {}", val).as_str());
            },
            crate::parser::ExpressionType::InfixExpression => {
                todo!()
            },
            crate::parser::ExpressionType::PrefixExpression => {
                self.emit_code_for_negation(right_side);
                self.asm_write("xor rbx, 1");
            },
            crate::parser::ExpressionType::ReturnExpression => panic!("can not prefix a return"),
        }
    }
    
    fn emit_code_for_addition(&mut self, expression: &Expression, first_iteration: bool) {
        if let Some(left_side) = expression.left.as_ref() {
            match left_side.as_ref().expression_type {
                crate::parser::ExpressionType::LiteralExpression => {
                    if first_iteration {
                        self.asm_mov("rbx", &left_side.literal_as_str());
                    } else {
                        self.asm_add("rbx", &left_side.literal_as_str());
                    }
                },
                crate::parser::ExpressionType::InfixExpression => todo!(),
                crate::parser::ExpressionType::PrefixExpression => todo!(),
                crate::parser::ExpressionType::ReturnExpression => panic!("can not prefix a return"),
            }
        } else {
            self.asm_add("rbx", &expression.literal_as_str());
        }

        if let Some(right_side) = expression.right.as_ref() {
            match right_side.expression_type {
                crate::parser::ExpressionType::LiteralExpression => {
                    self.asm_add("rbx", &right_side.literal_as_str());
                },
                crate::parser::ExpressionType::InfixExpression => {
                    self.emit_code_for_addition(right_side.left_side(), false);
                    self.emit_code_for_addition(right_side.right_side(), false);
                }
                crate::parser::ExpressionType::PrefixExpression => todo!(),
                crate::parser::ExpressionType::ReturnExpression => panic!("can not prefix a return"),
            }
        }
    }

    fn emit_code_for_return(&mut self, expression: &Expression) {
        let right_side = expression.right_side();
        match right_side.expression_type {
            crate::parser::ExpressionType::LiteralExpression => {
                self.asm_mov("rbx", &right_side.literal_as_str());
            }
            crate::parser::ExpressionType::PrefixExpression => {
                self.emit_code_for_negation(right_side); // already leaves val in rbx
            },
            crate::parser::ExpressionType::InfixExpression => {
                self.emit_code_for_addition(right_side, true);
            },
            crate::parser::ExpressionType::ReturnExpression => {
                panic!("can not return a return");
            },
        }
        self.asm_mov("rax", "1");
        self.asm_write("int 0x80");
    }
}

pub fn generate_code(expressions: Vec<Expression>) -> String {
    let mut compiler = Compiler::new(expressions);
    compiler.compile();
    compiler.code
}
