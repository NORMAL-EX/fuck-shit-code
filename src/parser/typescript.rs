use crate::common::LanguageType;
use crate::parser::{BaseParseResult, JavaScriptParser, ParseResult, Parser};
use std::path::Path;

pub struct TypeScriptParser;

impl TypeScriptParser {
    pub fn new() -> Self {
        TypeScriptParser
    }
}

impl Parser for TypeScriptParser {
    fn parse(
        &self,
        file_path: &Path,
        content: &str,
    ) -> Result<Box<dyn ParseResult>, Box<dyn std::error::Error>> {
        // TypeScript和JavaScript解析逻辑相似，复用JavaScript解析器
        let js_parser = JavaScriptParser::new();
        let result = js_parser.parse(file_path, content)?;

        // 创建一个新的结果，修改语言类型
        let functions = result.get_functions().to_vec();
        let comment_lines = result.get_comment_lines();
        let total_lines = result.get_total_lines();

        Ok(Box::new(BaseParseResult {
            functions,
            comment_lines,
            total_lines,
            language: LanguageType::TypeScript,
        }))
    }

    fn supported_languages(&self) -> Vec<LanguageType> {
        vec![LanguageType::TypeScript]
    }
}
