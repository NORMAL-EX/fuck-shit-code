use crate::common::LanguageType;
use crate::parser::{BaseParseResult, Function, ParseResult, Parser};
use regex::Regex;
use std::path::Path;

pub struct PythonParser;

impl PythonParser {
    pub fn new() -> Self {
        PythonParser
    }
}

impl Parser for PythonParser {
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
            language: LanguageType::Python,
        }))
    }

    fn supported_languages(&self) -> Vec<LanguageType> {
        vec![LanguageType::Python]
    }
}

impl PythonParser {
    fn count_comment_lines(&self, lines: &[&str]) -> usize {
        let mut count = 0;
        let mut in_doc_string = false;
        let mut doc_delimiter = "";

        for line in lines {
            let trimmed = line.trim();

            if in_doc_string {
                count += 1;
                if trimmed.contains(doc_delimiter) {
                    in_doc_string = false;
                }
                continue;
            }

            if trimmed.starts_with('#') {
                count += 1;
                continue;
            }

            if trimmed.starts_with("\"\"\"") {
                count += 1;
                in_doc_string = true;
                doc_delimiter = "\"\"\"";
                if trimmed.matches("\"\"\"").count() > 1 {
                    in_doc_string = false;
                }
            } else if trimmed.starts_with("'''") {
                count += 1;
                in_doc_string = true;
                doc_delimiter = "'''";
                if trimmed.matches("'''").count() > 1 {
                    in_doc_string = false;
                }
            }
        }

        count
    }

    fn detect_functions(&self, lines: &[&str]) -> Vec<Function> {
        let mut functions = Vec::new();
        let func_regex = Regex::new(r"^\s*def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)").unwrap();

        for (i, line) in lines.iter().enumerate() {
            if let Some(captures) = func_regex.captures(line) {
                let func_name = captures.get(1).unwrap().as_str().to_string();
                let params_str = captures.get(2).unwrap().as_str();

                let params = if params_str.trim().is_empty() {
                    0
                } else {
                    params_str.split(',').count()
                };

                let end_line = self.find_function_end(lines, i);
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

    fn find_function_end(&self, lines: &[&str], start: usize) -> usize {
        if start >= lines.len() {
            return lines.len() - 1;
        }

        let base_indent = self.get_indent_level(lines[start]);

        for i in (start + 1)..lines.len() {
            let line = lines[i].trim();
            if !line.is_empty() && !line.starts_with('#') {
                let indent = self.get_indent_level(lines[i]);
                if indent <= base_indent {
                    return i - 1;
                }
            }
        }

        lines.len() - 1
    }

    fn get_indent_level(&self, line: &str) -> usize {
        let mut level = 0;
        for ch in line.chars() {
            match ch {
                ' ' => level += 1,
                '\t' => level += 4,
                _ => break,
            }
        }
        level
    }

    fn calculate_complexity(&self, function_lines: &[&str]) -> usize {
        let mut complexity = 1;

        for line in function_lines {
            complexity += line.matches(" if ").count();
            complexity += line.matches(" elif ").count();
            complexity += line.matches(" else:").count();
            complexity += line.matches(" for ").count();
            complexity += line.matches(" while ").count();
            complexity += line.matches(" except ").count();
            complexity += line.matches(" finally:").count();
            complexity += line.matches(" and ").count();
            complexity += line.matches(" or ").count();
        }

        complexity
    }
}
