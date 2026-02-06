//! # CSS解析器
//!
//! 专门用于解析CSS文件

use crate::common::LanguageType;
use crate::parser::{BaseParseResult, Function, ParseResult, Parser};
use regex::Regex;
use std::path::Path;

/// CSS解析器
pub struct CSSParser {
    /// CSS规则正则
    rule_regex: Regex,
}

impl CSSParser {
    /// 创建新的CSS解析器
    ///
    /// # Returns
    /// * `Self` - 解析器实例
    pub fn new() -> Self {
        let rule_regex = Regex::new(r"([^{]+)\s*\{([^}]*)\}").unwrap();

        CSSParser { rule_regex }
    }

    /// 计数CSS注释行
    ///
    /// # Arguments
    /// * `lines` - 代码行
    ///
    /// # Returns
    /// * `usize` - 注释行数
    fn count_comment_lines(&self, lines: &[&str]) -> usize {
        let mut count = 0;
        let mut in_comment = false;

        for line in lines {
            let trimmed = line.trim();

            if in_comment {
                count += 1;
                if trimmed.contains("*/") {
                    in_comment = false;
                }
                continue;
            }

            if trimmed.starts_with("/*") {
                count += 1;
                in_comment = true;
                if trimmed.contains("*/") {
                    in_comment = false;
                }
            }
        }

        count
    }

    /// 检测CSS规则
    ///
    /// # Arguments
    /// * `lines` - 代码行
    ///
    /// # Returns
    /// * `Vec<Function>` - CSS规则列表
    fn detect_css_rules(&self, lines: &[&str]) -> Vec<Function> {
        let mut rules = Vec::new();
        let mut in_rule = false;
        let mut rule_start = 0;
        let mut rule_content = String::new();
        let mut selector = String::new();
        let mut brace_count = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // 跳过注释和空行
            if trimmed.starts_with("/*") || trimmed.is_empty() {
                continue;
            }

            if !in_rule
                && (trimmed.contains('{')
                    || (trimmed.contains(':')
                        && !trimmed.contains('{')
                        && i + 1 < lines.len()
                        && lines[i + 1].trim().contains('{')))
            {
                in_rule = true;
                rule_start = i;
                rule_content.clear();
                selector = if trimmed.contains('{') {
                    trimmed.split('{').next().unwrap_or("").trim().to_string()
                } else {
                    trimmed.to_string()
                };
                brace_count = trimmed.matches('{').count();
            }

            if in_rule {
                rule_content.push_str(line);
                rule_content.push('\n');

                brace_count += line.matches('{').count();
                brace_count -= line.matches('}').count();

                if brace_count == 0 {
                    in_rule = false;

                    let complexity = self.calculate_rule_complexity(&selector, &rule_content);
                    let rule_name = self.extract_rule_name(&selector);

                    rules.push(Function::new(
                        rule_name,
                        rule_start + 1,
                        i + 1,
                        complexity,
                        0,
                    ));
                }
            }
        }

        rules
    }

    /// 提取规则名称
    ///
    /// # Arguments
    /// * `selector` - 选择器
    ///
    /// # Returns
    /// * `String` - 规则名称
    fn extract_rule_name(&self, selector: &str) -> String {
        let cleaned = selector.trim().replace('\n', " ").replace('\r', "");

        if cleaned.len() > 50 {
            format!("{}...", &cleaned[..47])
        } else if cleaned.is_empty() {
            "css_rule".to_string()
        } else {
            cleaned
        }
    }

    /// 计算CSS规则复杂度
    ///
    /// # Arguments
    /// * `selector` - 选择器
    /// * `content` - 规则内容
    ///
    /// # Returns
    /// * `usize` - 复杂度
    fn calculate_rule_complexity(&self, selector: &str, content: &str) -> usize {
        let mut complexity = 1;

        // 选择器复杂度
        complexity += self.calculate_selector_complexity(selector);

        // 属性复杂度
        complexity += self.calculate_properties_complexity(content);

        complexity
    }

    /// 计算选择器复杂度
    ///
    /// # Arguments
    /// * `selector` - 选择器
    ///
    /// # Returns
    /// * `usize` - 复杂度
    fn calculate_selector_complexity(&self, selector: &str) -> usize {
        let mut complexity = 0;

        // 后代选择器
        complexity += selector.matches(' ').count();

        // 子选择器
        complexity += selector.matches('>').count();

        // 相邻兄弟选择器
        complexity += selector.matches('+').count();

        // 通用兄弟选择器
        complexity += selector.matches('~').count();

        // 类选择器
        complexity += selector.matches('.').count();

        // ID选择器
        complexity += selector.matches('#').count() * 2; // ID权重更高

        // 属性选择器
        complexity += selector.matches('[').count();

        // 伪类和伪元素
        complexity += selector.matches(':').count();

        // 多个选择器
        complexity += selector.matches(',').count();

        complexity
    }

    /// 计算属性复杂度
    ///
    /// # Arguments
    /// * `content` - 规则内容
    ///
    /// # Returns
    /// * `usize` - 复杂度
    fn calculate_properties_complexity(&self, content: &str) -> usize {
        let mut complexity = 0;

        // 属性数量
        complexity += content.matches(':').count();

        // 复杂属性
        let complex_properties = [
            "transform",
            "animation",
            "transition",
            "background",
            "border",
            "box-shadow",
            "text-shadow",
            "filter",
        ];

        for prop in &complex_properties {
            complexity += content.matches(prop).count() * 2;
        }

        // calc() 函数
        complexity += content.matches("calc(").count() * 2;

        // 媒体查询
        complexity += content.matches("@media").count() * 3;

        // 嵌套规则（SCSS/Sass）
        complexity += content.matches('{').count();

        complexity
    }
}

impl Parser for CSSParser {
    /// 解析CSS文件
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

        // 检测CSS规则
        let functions = self.detect_css_rules(&lines);

        Ok(Box::new(BaseParseResult {
            functions,
            comment_lines,
            total_lines,
            language: LanguageType::CSS,
        }))
    }

    /// 获取支持的语言
    ///
    /// # Returns
    /// * `Vec<LanguageType>` - 语言列表
    fn supported_languages(&self) -> Vec<LanguageType> {
        vec![LanguageType::CSS]
    }
}
