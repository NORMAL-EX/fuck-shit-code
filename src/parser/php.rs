//! # PHP语言解析器
//! 
//! 专门用于解析PHP源代码文件

use crate::common::LanguageType;
use crate::parser::{BaseParseResult, Function, ParseResult, Parser};
use regex::Regex;
use std::path::Path;

/// PHP解析器
pub struct PHPParser {
    /// 函数正则表达式
    function_regex: Regex,
    /// 类方法正则表达式
    method_regex: Regex,
}

impl PHPParser {
    /// 创建新的PHP解析器
    /// 
    /// # Returns
    /// * `Self` - 解析器实例
    pub fn new() -> Self {
        let function_regex = Regex::new(
            r"^\s*(public|private|protected)?\s*(static)?\s*function\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)"
        ).unwrap();
        
        let method_regex = Regex::new(
            r"^\s*(public|private|protected)\s+(static\s+)?(function\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)"
        ).unwrap();
        
        PHPParser { 
            function_regex,
            method_regex,
        }
    }
    
    /// 计数注释行
    /// 
    /// # Arguments
    /// * `lines` - 代码行
    /// 
    /// # Returns
    /// * `usize` - 注释行数
    fn count_comment_lines(&self, lines: &[&str]) -> usize {
        let mut count = 0;
        let mut in_block_comment = false;
        let mut in_doc_comment = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            // 处理块注释
            if in_block_comment {
                count += 1;
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }
            
            // 处理文档注释
            if in_doc_comment {
                count += 1;
                if trimmed.contains("*/") {
                    in_doc_comment = false;
                }
                continue;
            }
            
            // 检查单行注释
            if self.is_single_line_comment(trimmed) {
                count += 1;
                continue;
            }
            
            // 检查文档注释开始
            if trimmed.starts_with("/**") {
                count += 1;
                in_doc_comment = true;
                if trimmed.contains("*/") {
                    in_doc_comment = false;
                }
                continue;
            }
            
            // 检查块注释开始
            if trimmed.starts_with("/*") {
                count += 1;
                in_block_comment = true;
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
            }
        }
        
        count
    }
    
    /// 判断是否为单行注释
    /// 
    /// # Arguments
    /// * `line` - 代码行
    /// 
    /// # Returns
    /// * `bool` - 是否为单行注释
    fn is_single_line_comment(&self, line: &str) -> bool {
        line.starts_with("//") || line.starts_with('#')
    }
    
    /// 检测函数和方法
    /// 
    /// # Arguments
    /// * `lines` - 代码行
    /// 
    /// # Returns
    /// * `Vec<Function>` - 函数列表
    fn detect_functions(&self, lines: &[&str]) -> Vec<Function> {
        let mut functions = Vec::new();
        let mut i = 0;
        
        while i < lines.len() {
            if let Some(func) = self.try_parse_function(lines, i) {
                i = func.end_line.saturating_sub(1); // 跳到函数结束
                functions.push(func);
            } else {
                i += 1;
            }
        }
        
        functions
    }
    
    /// 尝试解析函数
    /// 
    /// # Arguments
    /// * `lines` - 代码行
    /// * `start` - 起始位置
    /// 
    /// # Returns
    /// * `Option<Function>` - 函数信息
    fn try_parse_function(&self, lines: &[&str], start: usize) -> Option<Function> {
        let line = lines[start];
        
        // 尝试匹配函数声明
        let (name, params_str) = if let Some(captures) = self.function_regex.captures(line) {
            let name = captures.get(3)?.as_str().to_string();
            let params_str = captures.get(4)?.as_str();
            (name, params_str)
        } else if let Some(captures) = self.method_regex.captures(line) {
            let name = captures.get(4)?.as_str().to_string();
            let params_str = captures.get(5)?.as_str();
            (name, params_str)
        } else {
            return None;
        };
        
        // 计算参数数量
        let parameters = self.count_parameters(params_str);
        
        // 查找函数结束位置
        let (end_line, found) = self.find_function_end(lines, start);
        
        if !found {
            return None;
        }
        
        // 计算复杂度
        let complexity = self.calculate_complexity(&lines[start..=end_line.min(lines.len() - 1)]);
        
        Some(Function::new(
            name,
            start + 1,  // 转换为1索引
            end_line + 1,  // 转换为1索引
            complexity,
            parameters,
        ))
    }
    
    /// 计数参数
    /// 
    /// # Arguments
    /// * `params_str` - 参数字符串
    /// 
    /// # Returns
    /// * `usize` - 参数数量
    fn count_parameters(&self, params_str: &str) -> usize {
        let trimmed = params_str.trim();
        
        if trimmed.is_empty() {
            return 0;
        }
        
        // 简单计数逗号分隔的参数
        trimmed.split(',').count()
    }
    
    /// 查找函数结束位置
    /// 
    /// # Arguments
    /// * `lines` - 代码行
    /// * `start` - 起始位置
    /// 
    /// # Returns
    /// * `(usize, bool)` - (结束位置, 是否找到)
    fn find_function_end(&self, lines: &[&str], start: usize) -> (usize, bool) {
        let mut brace_count = 0;
        let mut found_first_brace = false;
        
        for i in start..lines.len() {
            let brace_changes = self.count_braces(lines[i]);
            
            brace_count += brace_changes.0; // 左大括号
            if brace_changes.0 > 0 {
                found_first_brace = true;
            }
            
            brace_count -= brace_changes.1; // 右大括号
            
            if found_first_brace && brace_count == 0 {
                return (i, true);
            }
            
            // 检查是否是抽象方法或接口方法
            if i == start && lines[i].contains(';') {
                return (start, false);
            }
        }
        
        (lines.len() - 1, found_first_brace)
    }
    
    /// 计数大括号
    /// 
    /// # Arguments
    /// * `line` - 代码行
    /// 
    /// # Returns
    /// * `(usize, usize)` - (左大括号数, 右大括号数)
    fn count_braces(&self, line: &str) -> (usize, usize) {
        let left = line.matches('{').count();
        let right = line.matches('}').count();
        (left, right)
    }
    
    /// 计算循环复杂度
    /// 
    /// # Arguments
    /// * `function_lines` - 函数代码行
    /// 
    /// # Returns
    /// * `usize` - 复杂度
    fn calculate_complexity(&self, function_lines: &[&str]) -> usize {
        let mut complexity = 1;
        
        for line in function_lines {
            complexity += self.count_control_flow_keywords(line);
            complexity += self.count_logical_operators(line);
            complexity += self.count_php_specific(line);
        }
        
        complexity
    }
    
    /// 计数控制流关键字
    /// 
    /// # Arguments
    /// * `line` - 代码行
    /// 
    /// # Returns
    /// * `usize` - 关键字数量
    fn count_control_flow_keywords(&self, line: &str) -> usize {
        let keywords = [
            " if ", " else ", " elseif ", " for ", " foreach ", 
            " while ", " do ", " switch ", " case ", " catch ", " try "
        ];
        
        keywords.iter()
            .map(|kw| line.matches(kw).count())
            .sum()
    }
    
    /// 计数逻辑运算符
    /// 
    /// # Arguments
    /// * `line` - 代码行
    /// 
    /// # Returns
    /// * `usize` - 运算符数量
    fn count_logical_operators(&self, line: &str) -> usize {
        line.matches(" && ").count() + 
        line.matches(" || ").count() + 
        line.matches(" and ").count() + 
        line.matches(" or ").count()
    }
    
    /// 计数PHP特定结构
    /// 
    /// # Arguments
    /// * `line` - 代码行
    /// 
    /// # Returns
    /// * `usize` - 结构数量
    fn count_php_specific(&self, line: &str) -> usize {
        let mut count = 0;
        
        // PHP特有的复杂度
        count += line.matches("??").count(); // null合并操作符
        count += line.matches("?:").count(); // 三元操作符简写
        count += line.matches(" ? ").count(); // 三元操作符
        
        count
    }
}

impl Parser for PHPParser {
    /// 解析PHP文件
    /// 
    /// # Arguments
    /// * `_file_path` - 文件路径
    /// * `content` - 文件内容
    /// 
    /// # Returns
    /// * `Result<Box<dyn ParseResult>, Box<dyn std::error::Error>>` - 解析结果
    fn parse(
        &self,
        _file_path: &Path,
        content: &str,
    ) -> Result<Box<dyn ParseResult>, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();
        
        // 计算注释行数
        let comment_lines = self.count_comment_lines(&lines);
        
        // 检测函数
        let functions = self.detect_functions(&lines);
        
        Ok(Box::new(BaseParseResult {
            functions,
            comment_lines,
            total_lines,
            language: LanguageType::PHP,
        }))
    }
    
    /// 获取支持的语言
    /// 
    /// # Returns
    /// * `Vec<LanguageType>` - 语言列表
    fn supported_languages(&self) -> Vec<LanguageType> {
        vec![LanguageType::PHP]
    }
}