use std::path::Path;
use swc_common::sync::Lrc;
use swc_common::{errors::{ColorConfig, Handler},SourceMap};
use swc_ecma_visit::swc_ecma_ast::{EsVersion,Module};
use swc_ecma_parser::{EsConfig,lexer::Lexer, Parser, StringInput, Syntax,TsConfig};

pub fn parse_module(input: &Path) -> std::result::Result<Module, ModuleParseError> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false,
        Some(cm.clone()));
    let source_file = cm.load_file(input)?;
    let syntax = if input.ends_with(".ts") || input.ends_with(".tsx") {
        let config: TsConfig = Default::default();
        Syntax::Typescript(TsConfig{
            tsx: input.extension().unwrap() == "tsx",
            decorators: config.decorators,
            dts: config.dts,
            no_early_errors: config.no_early_errors,
        })
    } else {
        let config: EsConfig = Default::default();
        Syntax::Es(EsConfig {
            jsx: input.extension().unwrap() == "jsx",
            fn_bind: config.fn_bind,
            decorators: config.decorators,
            decorators_before_export: config.decorators_before_export,
            export_default_from: config.export_default_from,
            import_assertions: config.import_assertions,
            private_in_object: config.private_in_object,
            allow_super_outside_method: config.allow_super_outside_method,
            allow_return_outside_function: config.allow_return_outside_function
        })
    };

    let lexer = Lexer::new(
        syntax,
        EsVersion::latest(),
        StringInput::from(&*source_file),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    for e in parser.take_errors() {
        eprintln!("Error creating parser from {}", input.display());
        e.into_diagnostic(&handler).emit();
    }

    let parsed = parser
        .parse_module()
        .map_err(|e| {
            eprintln!("Error parsing: {}", input.display());
            e.into_diagnostic(&handler).emit();
            return ModuleParseError{io_error: None};
        })?;
     return Ok(parsed);
}

pub struct ModuleParseError {
    pub io_error: Option<std::io::Error>,
}

impl From<std::io::Error> for ModuleParseError {
    fn from(err: std::io::Error) -> ModuleParseError {
        return ModuleParseError {
            io_error: Some(err),
        }
    }
}

impl From<swc_ecma_parser::error::Error> for ModuleParseError {
    fn from(_: swc_ecma_parser::error::Error) -> ModuleParseError {
        return ModuleParseError {
            io_error: None,
        }
    }
}