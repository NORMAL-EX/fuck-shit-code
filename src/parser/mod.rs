//! # 解析器模块
//! 
//! 提供各种编程语言的代码解析功能

mod base;
mod generic;
mod rust;
mod go;
mod javascript;
mod typescript;
mod python;
mod java;
mod c;
mod csharp;
mod php;
mod html;
mod css;

use crate::common::LanguageType;
use std::path::Path;

pub use base::{BaseParseResult, Function, ParseResult, Parser};
pub use generic::GenericParser;
pub use rust::RustParser;
pub use go::GoParser;
pub use javascript::JavaScriptParser;
pub use typescript::TypeScriptParser;
pub use python::PythonParser;
pub use java::JavaParser;
pub use c::CParser;
pub use csharp::CSharpParser;
pub use php::PHPParser;
pub use html::HTMLParser;
pub use css::CSSParser;

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