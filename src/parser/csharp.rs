use crate::common::LanguageType;
use crate::parser::{BaseParseResult, Function, ParseResult, Parser};
use regex::Regex;
use std::path::Path;

pub struct CSharpParser;

impl CSharpParser {
    pub fn new() -> Self {
        CSharpParser
    }
}

impl Parser for CSharpParser {
    fn parse(
        &self,
        _file_path: &Path,
        content: &str,
    ) -> Result<Box<dyn ParseResult>, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let comment_lines = self.count_comment_lines(&lines);
        let functions = self.detect_functions(&lines);

        Ok(Box::new(BaseParseResult {
            functions,
            comment_lines,
            total_lines,
            language: LanguageType::CSharp,
        }))
    }

    fn supported_languages(&self) -> Vec<LanguageType> {
        vec![LanguageType::CSharp]
    }
}

impl CSharpParser {
    fn count_comment_lines(&self, lines: &[&str]) -> usize {
        let mut count = 0;
        let mut in_block_comment = false;
        let mut in_xml_doc = false;

        for line in lines {
            let trimmed = line.trim();

            if in_block_comment {
                count += 1;
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }

            if in_xml_doc {
                count += 1;
                if !trimmed.starts_with("///") && !trimmed.starts_with("*") && !trimmed.is_empty() {
                    in_xml_doc = false;
                }
                continue;
            }

            if trimmed.starts_with("//") {
                count += 1;
                continue;
            }

            if trimmed.starts_with("///") {
                count += 1;
                in_xml_doc = true;
                continue;
            }

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

    fn detect_functions(&self, lines: &[&str]) -> Vec<Function> {
        let mut functions = Vec::new();
        let method_regex = Regex::new(
            r"^\s*(?:(?:public|private|protected|internal|static|virtual|override|abstract|sealed|async)\s+)*([a-zA-Z_][a-zA-Z0-9_<>\[\]]*(?:\?)?)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)\s*(?:\{|=>)"
        ).unwrap();

        for (i, line) in lines.iter().enumerate() {
            if let Some(captures) = method_regex.captures(line) {
                let func_name = captures.get(2).unwrap().as_str().to_string();
                let params_str = captures.get(3).unwrap().as_str();

                let params = if params_str.trim().is_empty() {
                    0
                } else {
                    params_str.split(',').count()
                };

                let end_line = self.find_method_end(lines, i);
                let complexity = self.calculate_complexity(&lines[i..=end_line]);

                functions.push(Function {
                    name: func_name,
                    start_line: i + 1,
                    end_line: end_line + 1,
                    complexity,
                    parameters: params,
                });
            }
        }

        functions
    }

    fn find_method_end(&self, lines: &[&str], start: usize) -> usize {
        // 检查是否是表达式方法体 (=>)
        if lines[start].contains("=>") {
            for i in start..lines.len() {
                if lines[i].contains(";") {
                    return i;
                }
            }
            return start;
        }

        // 普通方法体
        let mut brace_count = 0;
        let mut found_first = false;

        for i in start..lines.len() {
            for ch in lines[i].chars() {
                match ch {
                    '{' => {
                        brace_count += 1;
                        found_first = true;
                    }
                    '}' => {
                        brace_count -= 1;
                        if found_first && brace_count == 0 {
                            return i;
                        }
                    }
                    _ => {}
                }
            }
        }

        lines.len() - 1
    }

    fn calculate_complexity(&self, function_lines: &[&str]) -> usize {
        let mut complexity = 1;

        for line in function_lines {
            complexity += line.matches(" if ").count();
            complexity += line.matches(" else ").count();
            complexity += line.matches(" for ").count();
            complexity += line.matches(" foreach ").count();
            complexity += line.matches(" while ").count();
            complexity += line.matches(" do ").count();
            complexity += line.matches(" switch ").count();
            complexity += line.matches(" case ").count();
            complexity += line.matches(" catch ").count();
            complexity += line.matches(" && ").count();
            complexity += line.matches(" || ").count();
            complexity += line.matches(" ?? ").count();
            complexity += line.matches(" ? ").count();
        }

        complexity
    }
}
