use crate::common::LanguageType;
use crate::parser::{BaseParseResult, Function, ParseResult, Parser};
use regex::Regex;
use std::path::Path;

pub struct CParser;

impl CParser {
    pub fn new() -> Self {
        CParser
    }
}

impl Parser for CParser {
    fn parse(&self, file_path: &Path, content: &str) -> Result<Box<dyn ParseResult>, Box<dyn std::error::Error>> {
        let detector = crate::common::LanguageDetector::new();
        let language = detector.detect_language(file_path);
        
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();
        
        let comment_lines = self.count_comment_lines(&lines);
        let functions = self.detect_functions(&lines);
        
        Ok(Box::new(BaseParseResult {
            functions,
            comment_lines,
            total_lines,
            language,
        }))
    }
    
    fn supported_languages(&self) -> Vec<LanguageType> {
        vec![LanguageType::C, LanguageType::CPlusPlus]
    }
}

impl CParser {
    fn count_comment_lines(&self, lines: &[&str]) -> usize {
        let mut count = 0;
        let mut in_block_comment = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            if in_block_comment {
                count += 1;
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }
            
            if trimmed.starts_with("//") {
                count += 1;
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
        let func_regex = Regex::new(r"([\w\*]+\s+)+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^;]*)\)\s*\{").unwrap();
        
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.ends_with("{") && !trimmed.starts_with("{") {
                if let Some(captures) = func_regex.captures(line) {
                    let func_name = captures.get(2).unwrap().as_str().to_string();
                    let params_str = captures.get(3).unwrap().as_str();
                    
                    let params = if params_str.trim().is_empty() || params_str.trim() == "void" {
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
        }
        
        functions
    }
    
    fn find_function_end(&self, lines: &[&str], start: usize) -> usize {
        let mut brace_count = 1;
        
        for i in (start + 1)..lines.len() {
            for ch in lines[i].chars() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
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
            complexity += line.matches(" while ").count();
            complexity += line.matches(" do ").count();
            complexity += line.matches(" switch ").count();
            complexity += line.matches(" case ").count();
            complexity += line.matches(" && ").count();
            complexity += line.matches(" || ").count();
            complexity += line.matches(" ? ").count();
        }
        
        complexity
    }
}