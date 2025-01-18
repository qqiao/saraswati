use std::path::Path;
use std::rc::Rc;
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    FileName, SourceFile, SourceMap,
};
use swc_ecma_ast::BlockStmt;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::{Fold, VisitMut};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
}

pub struct Compiler {}

impl VisitMut for Compiler {
    fn visit_mut_block_stmt(&mut self, _stmt: &mut BlockStmt) {
        todo!()
    }
}

impl Fold for Compiler {}

impl Compiler {
    pub fn compile_file<P: AsRef<Path>>(&self, _path: P) -> Result<String, Error> {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.load_file(_path.as_ref())?;
        self.compile(cm.clone(), fm)
    }

    pub fn compile_source(&self, src: &str, file_name: &str) -> Result<String, Error> {
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(FileName::Custom(file_name.into()).into(), src.into());
        self.compile(cm.clone(), fm)
    }

    fn compile(&self, cm: Lrc<SourceMap>, fm: Rc<SourceFile>) -> Result<String, Error> {
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
        let lexer = Lexer::new(
            // We want to parse ecmascript
            Syntax::Es(Default::default()),
            // EsVersion defaults to es5
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);

        for e in parser.take_errors() {
            e.into_diagnostic(&handler).emit();
        }

        let _module = parser
            .parse_module()
            .map_err(|e| {
                // Unrecoverable fatal error occurred
                e.into_diagnostic(&handler).emit()
            })
            .expect("failed to parser module");
        todo!()
    }
}
