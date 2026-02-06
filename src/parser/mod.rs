//! # 解析器模块
//!
//! 提供各种编程语言的代码解析功能

mod base;
mod c;
mod csharp;
mod css;
mod generic;
mod go;
mod html;
mod java;
mod javascript;
mod php;
mod python;
mod rust;
mod typescript;

use crate::common::LanguageType;
use std::path::Path;

pub use base::{BaseParseResult, Function, ParseResult, Parser};
pub use c::CParser;
pub use csharp::CSharpParser;
pub use css::CSSParser;
pub use generic::GenericParser;
pub use go::GoParser;
pub use html::HTMLParser;
pub use java::JavaParser;
pub use javascript::JavaScriptParser;
pub use php::PHPParser;
pub use python::PythonParser;
pub use rust::RustParser;
pub use typescript::TypeScriptParser;

/// 根据文件创建对应的解析器
///
/// # Arguments
/// * `file_path` - 文件路径
///
/// # Returns
/// * `Box<dyn Parser>` - 解析器实例
pub fn create_parser_for_file(file_path: &Path) -> Box<dyn Parser> {
    let detector = crate::common::LanguageDetector::new();
    let language = detector.detect_language(file_path);

    create_parser_for_language(language)
}

/// 根据语言类型创建解析器
///
/// # Arguments
/// * `language` - 语言类型
///
/// # Returns
/// * `Box<dyn Parser>` - 解析器实例
pub fn create_parser_for_language(language: LanguageType) -> Box<dyn Parser> {
    match language {
        LanguageType::Rust => Box::new(RustParser::new()),
        LanguageType::Go => Box::new(GoParser::new()),
        LanguageType::JavaScript => Box::new(JavaScriptParser::new()),
        LanguageType::TypeScript => Box::new(TypeScriptParser::new()),
        LanguageType::Python => Box::new(PythonParser::new()),
        LanguageType::Java => Box::new(JavaParser::new()),
        LanguageType::C | LanguageType::CPlusPlus => Box::new(CParser::new()),
        LanguageType::CSharp => Box::new(CSharpParser::new()),
        LanguageType::PHP => Box::new(PHPParser::new()),
        LanguageType::HTML => Box::new(HTMLParser::new()),
        LanguageType::CSS => Box::new(CSSParser::new()),
        _ => Box::new(GenericParser::new()),
    }
}
