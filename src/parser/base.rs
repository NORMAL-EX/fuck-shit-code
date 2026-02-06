//! # 解析器基础定义
//!
//! 定义解析器的基础接口和数据结构

use crate::common::LanguageType;
use std::path::Path;

/// 函数信息结构
#[derive(Debug, Clone)]
pub struct Function {
    /// 函数名称
    pub name: String,

    /// 起始行号
    pub start_line: usize,

    /// 结束行号
    pub end_line: usize,

    /// 循环复杂度
    pub complexity: usize,

    /// 参数数量
    pub parameters: usize,
}

impl Function {
    /// 创建新的函数信息
    ///
    /// # Arguments
    /// * `name` - 函数名
    /// * `start_line` - 起始行
    /// * `end_line` - 结束行
    /// * `complexity` - 复杂度
    /// * `parameters` - 参数数量
    ///
    /// # Returns
    /// * `Self` - 函数信息实例
    pub fn new(
        name: String,
        start_line: usize,
        end_line: usize,
        complexity: usize,
        parameters: usize,
    ) -> Self {
        Function {
            name,
            start_line,
            end_line,
            complexity,
            parameters,
        }
    }

    /// 获取函数行数
    ///
    /// # Returns
    /// * `usize` - 行数
    pub fn line_count(&self) -> usize {
        if self.end_line >= self.start_line {
            self.end_line - self.start_line + 1
        } else {
            0
        }
    }
}

/// 解析结果trait
pub trait ParseResult {
    /// 获取函数列表
    fn get_functions(&self) -> &[Function];

    /// 获取注释行数
    fn get_comment_lines(&self) -> usize;

    /// 获取总行数
    fn get_total_lines(&self) -> usize;

    /// 获取语言类型
    fn get_language(&self) -> LanguageType;
}

/// 解析器trait
pub trait Parser {
    /// 解析文件
    ///
    /// # Arguments
    /// * `file_path` - 文件路径
    /// * `content` - 文件内容
    ///
    /// # Returns
    /// * `Result<Box<dyn ParseResult>, Box<dyn std::error::Error>>` - 解析结果
    fn parse(
        &self,
        file_path: &Path,
        content: &str,
    ) -> Result<Box<dyn ParseResult>, Box<dyn std::error::Error>>;

    /// 获取支持的语言类型
    ///
    /// # Returns
    /// * `Vec<LanguageType>` - 支持的语言列表
    fn supported_languages(&self) -> Vec<LanguageType>;
}

/// 基础解析结果实现
#[derive(Debug, Clone)]
pub struct BaseParseResult {
    /// 函数列表
    pub functions: Vec<Function>,

    /// 注释行数
    pub comment_lines: usize,

    /// 总行数
    pub total_lines: usize,

    /// 语言类型
    pub language: LanguageType,
}

impl ParseResult for BaseParseResult {
    /// 获取函数列表
    fn get_functions(&self) -> &[Function] {
        &self.functions
    }

    /// 获取注释行数
    fn get_comment_lines(&self) -> usize {
        self.comment_lines
    }

    /// 获取总行数
    fn get_total_lines(&self) -> usize {
        self.total_lines
    }

    /// 获取语言类型
    fn get_language(&self) -> LanguageType {
        self.language
    }
}
