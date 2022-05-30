use cranelift::prelude::*;

use crate::ast::Expression;
use crate::lexer::TokenKind;

pub struct Generator {
    builder_context: FunctionBuilderContext,

}

pub struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>
}

impl <'a>FunctionTranslator<'a> {
    fn translate_expression(&mut self, expr: Expression) -> Result<Value, ()> {
        match expr {
            Expression::Number(num) => Ok(self.builder.ins().f64const(num)),
            Expression::Grouping(grouping_expression) => match *grouping_expression {
                Expression::Number(num) => Ok(self.builder.ins().f64const(num)),
                _ => unimplemented!("only gets number expressions for now")
            }
            Expression::Unary { operator: op, expression: unary_expression } => {
                match op {
                    TokenKind::Minus => match *unary_expression {
                        Expression::Number(num) => Ok(self.builder.ins().f64const(-num)),
                        _ => Err(eprintln!("Not correct value for negation"))
                    }
                    _ => unimplemented!("just takes negative numbers for now")
                }
                }
                Expression::Binary { left, operator, right } => {
                    let left = self.translate_expression(*left)?;
                    let right = self.translate_expression(*right)?;

                    match operator {
                        TokenKind::Plus => Ok(self.builder.ins().fadd(left, right)),
                        TokenKind::Minus => Ok(self.builder.ins().fsub(left, right)),
                        TokenKind::Slash => Ok(self.builder.ins().fdiv(left, right)),
                        TokenKind::Star => Ok(self.builder.ins().fmul(left, right)),
                        _ => unimplemented!("other binary operations have not been implemented yet")
                    }
                }
            }
        }
    }
