use crate::parser::Expression;

fn build_prelude(code: &mut String) {
    code.push_str("section .text\n");
    code.push_str("global _start\n");
    code.push_str("_start:\n");
}

fn emit_code_for_return(code: &mut String, expression: &Expression) {
    // return 5;
    // mov rbx, 5
    // int 0x80
    // =========
    // return !5
    // mov rbx, 5
    // neg rbx
    // int 0x80
    let right_side = expression.right.as_ref().unwrap();
    match right_side.expression_type {
        crate::parser::ExpressionType::LiteralExpression => {
            code.push_str(&format!("mov rbx, {:?}\n", right_side.token.literal.as_ref().unwrap()).to_string());
        },
        crate::parser::ExpressionType::PrefixExpression => todo!(),
        crate::parser::ExpressionType::ReturnExpression => todo!(),
    }
    code.push_str("mov rax, 1\n");
    code.push_str("int 0x80\n");
}

fn emit_code_for_expression(code: &mut String, expression: &Expression) {
    match expression.expression_type {
        crate::parser::ExpressionType::LiteralExpression => todo!(),
        crate::parser::ExpressionType::PrefixExpression => todo!(),
        crate::parser::ExpressionType::ReturnExpression => {
            emit_code_for_return(code, expression);
        },
    }
}

pub fn generate_code(expressions: &Vec<Expression>) -> String {
    let mut code = String::from("");
    build_prelude(&mut code);

    for expression in expressions {
        emit_code_for_expression(&mut code, expression);
    }

    code
}
