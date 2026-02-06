//! # HTML解析器
//!
//! 专门用于解析HTML文件

use crate::common::LanguageType;
use crate::parser::{BaseParseResult, Function, ParseResult, Parser};
use regex::Regex;
use std::path::Path;

/// HTML解析器
pub struct HTMLParser {
    /// 脚本标签正则
    script_regex: Regex,
    /// 样式标签正则
    style_regex: Regex,
}

impl HTMLParser {
    /// 创建新的HTML解析器
    ///
    /// # Returns
    /// * `Self` - 解析器实例
    pub fn new() -> Self {
        let script_regex = Regex::new(r"<script[^>]*>(.*?)</script>").unwrap();
        let style_regex = Regex::new(r"<style[^>]*>(.*?)</style>").unwrap();

        HTMLParser {
            script_regex,
            style_regex,
        }
    }

    /// 计数HTML注释行
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
                if trimmed.contains("-->") {
                    in_comment = false;
                }
                continue;
            }

            if trimmed.starts_with("<!--") {
                count += 1;
                in_comment = true;
                if trimmed.contains("-->") {
                    in_comment = false;
                }
            }
        }

        count
    }

    /// 检测HTML结构复杂度
    ///
    /// # Arguments
    /// * `lines` - 代码行
    ///
    /// # Returns
    /// * `Vec<Function>` - 伪函数列表（代表HTML结构块）
    fn detect_html_blocks(&self, lines: &[&str]) -> Vec<Function> {
        let mut blocks = Vec::new();

        // 检测主要的HTML结构块
        blocks.extend(self.detect_script_blocks(lines));
        blocks.extend(self.detect_style_blocks(lines));
        blocks.extend(self.detect_form_blocks(lines));
        blocks.extend(self.detect_complex_elements(lines));

        blocks
    }

    /// 检测脚本块
    ///
    /// # Arguments
    /// * `lines` - 代码行
    ///
    /// # Returns
    /// * `Vec<Function>` - 脚本块列表
    fn detect_script_blocks(&self, lines: &[&str]) -> Vec<Function> {
        let mut blocks = Vec::new();
        let mut in_script = false;
        let mut script_start = 0;
        let mut script_lines = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if line.contains("<script") && !line.contains("</script>") {
                in_script = true;
                script_start = i;
                script_lines.clear();
                script_lines.push(*line);
            } else if in_script {
                script_lines.push(*line);
                if line.contains("</script>") {
                    in_script = false;

                    // 分析脚本复杂度
                    let complexity = self.calculate_js_complexity(&script_lines);

                    blocks.push(Function::new(
                        format!("script_block_{}", blocks.len() + 1),
                        script_start + 1,
                        i + 1,
                        complexity,
                        0,
                    ));
                }
            }
        }

        blocks
    }

    /// 检测样式块
    ///
    /// # Arguments
    /// * `lines` - 代码行
    ///
    /// # Returns
    /// * `Vec<Function>` - 样式块列表
    fn detect_style_blocks(&self, lines: &[&str]) -> Vec<Function> {
        let mut blocks = Vec::new();
        let mut in_style = false;
        let mut style_start = 0;
        let mut style_lines = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if line.contains("<style") && !line.contains("</style>") {
                in_style = true;
                style_start = i;
                style_lines.clear();
                style_lines.push(*line);
            } else if in_style {
                style_lines.push(*line);
                if line.contains("</style>") {
                    in_style = false;

                    // 分析样式复杂度
                    let complexity = self.calculate_css_complexity(&style_lines);

                    blocks.push(Function::new(
                        format!("style_block_{}", blocks.len() + 1),
                        style_start + 1,
                        i + 1,
                        complexity,
                        0,
                    ));
                }
            }
        }

        blocks
    }

    /// 检测表单块
    ///
    /// # Arguments
    /// * `lines` - 代码行
    ///
    /// # Returns
    /// * `Vec<Function>` - 表单块列表
    fn detect_form_blocks(&self, lines: &[&str]) -> Vec<Function> {
        let mut blocks = Vec::new();
        let mut in_form = false;
        let mut form_start = 0;
        let mut form_complexity = 1;

        for (i, line) in lines.iter().enumerate() {
            if line.contains("<form") {
                in_form = true;
                form_start = i;
                form_complexity = 1;
            } else if in_form {
                // 计算表单复杂度
                form_complexity += line.matches("<input").count();
                form_complexity += line.matches("<select").count();
                form_complexity += line.matches("<textarea").count();
                form_complexity += line.matches("<button").count();

                if line.contains("</form>") {
                    in_form = false;

                    blocks.push(Function::new(
                        format!("form_block_{}", blocks.len() + 1),
                        form_start + 1,
                        i + 1,
                        form_complexity,
                        0,
                    ));
                }
            }
        }

        blocks
    }

    /// 检测复杂元素
    ///
    /// # Arguments
    /// * `lines` - 代码行
    ///
    /// # Returns
    /// * `Vec<Function>` - 复杂元素列表
    fn detect_complex_elements(&self, lines: &[&str]) -> Vec<Function> {
        let mut blocks = Vec::new();
        let mut total_complexity = 1;

        for line in lines {
            // 计算HTML结构复杂度
            total_complexity += line.matches("<div").count();
            total_complexity += line.matches("<span").count();
            total_complexity += line.matches("<table").count();
            total_complexity += line.matches("<ul").count();
            total_complexity += line.matches("<ol").count();
        }

        // 如果整体复杂度较高，创建一个全局复杂度块
        if total_complexity > 50 {
            blocks.push(Function::new(
                "html_structure".to_string(),
                1,
                lines.len(),
                total_complexity / 10, // 缩放复杂度
                0,
            ));
        }

        blocks
    }

    /// 计算JavaScript复杂度
    ///
    /// # Arguments
    /// * `script_lines` - 脚本代码行
    ///
    /// # Returns
    /// * `usize` - 复杂度
    fn calculate_js_complexity(&self, script_lines: &[&str]) -> usize {
        let mut complexity = 1;

        for line in script_lines {
            complexity += line.matches(" if ").count();
            complexity += line.matches(" for ").count();
            complexity += line.matches(" while ").count();
            complexity += line.matches(" switch ").count();
            complexity += line.matches(" case ").count();
            complexity += line.matches(" && ").count();
            complexity += line.matches(" || ").count();
            complexity += line.matches(" ? ").count();
        }

        complexity
    }

    /// 计算CSS复杂度
    ///
    /// # Arguments
    /// * `style_lines` - 样式代码行
    ///
    /// # Returns
    /// * `usize` - 复杂度
    fn calculate_css_complexity(&self, style_lines: &[&str]) -> usize {
        let mut complexity = 1;

        for line in style_lines {
            // CSS选择器复杂度
            complexity += line.matches(' ').count(); // 后代选择器
            complexity += line.matches('>').count(); // 子选择器
            complexity += line.matches('+').count(); // 相邻选择器
            complexity += line.matches('~').count(); // 兄弟选择器
            complexity += line.matches('.').count(); // 类选择器
            complexity += line.matches('#').count(); // ID选择器
            complexity += line.matches('[').count(); // 属性选择器
            complexity += line.matches(':').count(); // 伪类选择器
        }

        complexity
    }
}

impl Parser for HTMLParser {
    /// 解析HTML文件
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

        // 检测HTML结构块
        let functions = self.detect_html_blocks(&lines);

        Ok(Box::new(BaseParseResult {
            functions,
            comment_lines,
            total_lines,
            language: LanguageType::HTML,
        }))
    }

    /// 获取支持的语言
    ///
    /// # Returns
    /// * `Vec<LanguageType>` - 语言列表
    fn supported_languages(&self) -> Vec<LanguageType> {
        vec![LanguageType::HTML]
    }
}
