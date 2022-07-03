use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

use std::mem;

use crate::ast::Expression;
use crate::lexer::TokenKind;
use crate::parser::Parser;

pub struct JIT {
    builder_context: FunctionBuilderContext,
    context: codegen::Context,
    module: JITModule,
}

impl Default for JIT {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("is_pic", "false");

        let isa_builder = cranelift_native::builder()
            .unwrap_or_else(|msg| panic!("host machine is not supported: {}", msg));

        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();

        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        let module = JITModule::new(builder);
        JIT {
            builder_context: FunctionBuilderContext::new(),
            context: module.make_context(),
            module,
        }
    }
}

impl JIT {
    pub fn compile(&mut self, src: &str) -> Result<fn() -> f64, String> {
        let mut parser = Parser::new(src);
        let expression = parser.parse().unwrap();

        self.translate(expression)?;

        // function must be declared to jit before they can be called or defined
        let id = self
            .module
            .declare_function(
                &format!("jit_{}", src),
                Linkage::Export,
                &self.context.func.signature,
            )
            .map_err(|e| e.to_string())?;

        self.module
            .define_function(id, &mut self.context)
            .map_err(|e| e.to_string())?;

        self.module.clear_context(&mut self.context);

        self.module.finalize_definitions();

        let code = self.module.get_finalized_function(id);

        unsafe { Ok(mem::transmute(code)) }
    }

    fn translate(&mut self, expr: Expression) -> Result<(), String> {
        // The only literal blox supports for now is the number literal(f64)
        let float = AbiParam::new(types::F64).value_type;

        self.context
            .func
            .signature
            .returns
            .push(AbiParam::new(float));

        let mut builder = FunctionBuilder::new(&mut self.context.func, &mut self.builder_context);

        // create block to emit code
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        // emit code in the block above

        builder.switch_to_block(entry_block);

        // seal because block will have no predeccessors
        builder.seal_block(entry_block);

        let mut translator = FunctionTranslator { builder };

        let ret = translator.translate_expression(expr).unwrap();

        translator.builder.ins().return_(&[ret]);
        translator.builder.finalize();
        Ok(())
    }
}

struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
}

impl<'a> FunctionTranslator<'a> {
    fn translate_expression(&mut self, expr: Expression) -> Result<Value, ()> {
        match expr {
            Expression::Number(num) => Ok(self.builder.ins().f64const(num)),
            Expression::Grouping(grouping_expression) => match *grouping_expression {
                _ => Ok(self.translate_expression(*grouping_expression)?),
            },
            Expression::Unary(operator, expression) => match operator {
                TokenKind::Minus => match *expression {
                    Expression::Number(num) => Ok(self.builder.ins().f64const(-num)),
                    _ => Err(eprintln!("Not correct value for negation")),
                },
                _ => unimplemented!("just takes negative numbers for now"),
            },

            Expression::Binary(left, operator, right) => {
                let left = self.translate_expression(*left)?;
                let right = self.translate_expression(*right)?;

                match operator {
                    TokenKind::Plus => Ok(self.builder.ins().fadd(left, right)),
                    TokenKind::Minus => Ok(self.builder.ins().fsub(left, right)),
                    TokenKind::Slash => Ok(self.builder.ins().fdiv(left, right)),
                    TokenKind::Star => Ok(self.builder.ins().fmul(left, right)),
                    _ => unimplemented!("other binary operations have not been implemented yet"),
                }
            }
            _ => unimplemented!("implement once you have functions"),
        }
    }
}
