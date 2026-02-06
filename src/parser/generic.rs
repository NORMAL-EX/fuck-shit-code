use crate::common::LanguageType;
use crate::parser::{BaseParseResult, Function, ParseResult, Parser};
use regex::Regex;
use std::path::Path;

/// Generic parser for unsupported or unknown file types
/// Provides basic parsing capabilities for common programming patterns
pub struct GenericParser;

impl GenericParser {
    pub fn new() -> Self {
        GenericParser
    }
}

impl Parser for GenericParser {
    fn parse(
        &self,
        file_path: &Path,
        content: &str,
    ) -> Result<Box<dyn ParseResult>, Box<dyn std::error::Error>> {
        let detector = crate::common::LanguageDetector::new();
        let language = detector.detect_language(file_path);

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        // Count comment lines based on language type
        let comment_lines = self.count_comment_lines(&lines, language);

        // Detect functions based on language patterns
        let functions = self.detect_functions(&lines, language);

        Ok(Box::new(BaseParseResult {
            functions,
            comment_lines,
            total_lines,
            language,
        }))
    }

    fn supported_languages(&self) -> Vec<LanguageType> {
        vec![LanguageType::Unsupported]
    }
}

impl GenericParser {
    /// Counts comment lines in the source code
    fn count_comment_lines(&self, lines: &[&str], language: LanguageType) -> usize {
        match language {
            LanguageType::Python => self.count_python_comments(lines),
            _ => self.count_c_style_comments(lines),
        }
    }

    /// Count Python-style comments (# and docstrings)
    fn count_python_comments(&self, lines: &[&str]) -> usize {
        let mut count = 0;
        let mut in_doc_string = false;
        let mut doc_delimiter = "";

        for line in lines {
            let trimmed = line.trim();

            // Handle docstring continuation
            if in_doc_string {
                count += 1;
                if trimmed.contains(doc_delimiter) {
                    in_doc_string = false;
                }
                continue;
            }

            // Single line comment
            if trimmed.starts_with('#') {
                count += 1;
                continue;
            }

            // Check for docstring start
            if let Some(delimiter) = self.get_docstring_delimiter(trimmed) {
                count += 1;
                in_doc_string = true;
                doc_delimiter = delimiter;

                // Check if docstring ends on same line
                let occurrences = trimmed.matches(delimiter).count();
                if occurrences > 1 {
                    in_doc_string = false;
                }
            }
        }

        count
    }

    /// Get the docstring delimiter if line starts with one
    fn get_docstring_delimiter(&self, line: &str) -> Option<&'static str> {
        if line.starts_with("\"\"\"") {
            Some("\"\"\"")
        } else if line.starts_with("'''") {
            Some("'''")
        } else {
            None
        }
    }

    /// Count C-style comments (// and /* */)
    fn count_c_style_comments(&self, lines: &[&str]) -> usize {
        let mut count = 0;
        let mut in_block_comment = false;

        for line in lines {
            let trimmed = line.trim();

            // Handle block comment continuation
            if in_block_comment {
                count += 1;
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }

            // Single line comment
            if trimmed.starts_with("//") {
                count += 1;
                continue;
            }

            // Block comment start
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

    /// Detect functions based on language patterns
    fn detect_functions(&self, lines: &[&str], language: LanguageType) -> Vec<Function> {
        let pattern = self.get_function_pattern(language);
        let regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => return vec![],
        };

        let mut functions = Vec::new();

        for (i, line) in lines.iter().enumerate() {
            if let Some(func_info) = self.extract_function_info(line, &regex) {
                let end_line = self.find_function_end(lines, i, language);
                let complexity = self.calculate_complexity(&lines[i..=end_line]);

                functions.push(Function {
                    name: func_info.name,
                    start_line: i + 1,
                    end_line: end_line + 1,
                    complexity,
                    parameters: func_info.param_count,
                });
            }
        }

        functions
    }

    /// Get regex pattern for function detection based on language
    fn get_function_pattern(&self, language: LanguageType) -> &'static str {
        match language {
            LanguageType::JavaScript | LanguageType::TypeScript => {
                r"(?:function\s+([a-zA-Z_$][a-zA-Z0-9_$]*)|([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=\s*function|([a-zA-Z_$][a-zA-Z0-9_$]*)\s*:\s*function|(?:const|let|var)\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=\s*\([^)]*\)\s*=>)"
            }
            LanguageType::Python => r"^\s*def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)",
            LanguageType::Java => {
                r"(?:public|private|protected|static|\s)+[\w\<\>\[\]]+\s+([\w]+)\s*\(([^\)]*)\)\s*(?:\{|throws)"
            }
            LanguageType::Go => r"func\s+(?:\([^)]*\)\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)",
            _ => {
                r"(?:function|def|void|int|bool|string|double|float)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\("
            }
        }
    }

    /// Extract function information from a line
    fn extract_function_info(&self, line: &str, regex: &Regex) -> Option<FunctionInfo> {
        regex.captures(line).map(|captures| {
            // Find the first non-None capture group for the name
            let name = captures
                .iter()
                .skip(1)
                .flatten()
                .next()
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| "anonymous".to_string());

            FunctionInfo {
                name,
                param_count: 0, // Simplified
            }
        })
    }

    /// Find where a function ends
    fn find_function_end(&self, lines: &[&str], start: usize, language: LanguageType) -> usize {
        match language {
            LanguageType::Python => self.find_python_function_end(lines, start),
            _ => self.find_brace_function_end(lines, start),
        }
    }

    /// Find function end for Python (indentation-based)
    fn find_python_function_end(&self, lines: &[&str], start: usize) -> usize {
        if start >= lines.len() {
            return lines.len() - 1;
        }

        let base_indent = self.get_indent_level(lines[start]);

        for i in (start + 1)..lines.len() {
            let line = lines[i].trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let indent = self.get_indent_level(lines[i]);
            if indent <= base_indent {
                return i - 1;
            }
        }

        lines.len() - 1
    }

    /// Find function end for brace-based languages
    fn find_brace_function_end(&self, lines: &[&str], start: usize) -> usize {
        let mut brace_count = 0;
        let mut found_first = false;

        for i in start..lines.len() {
            for ch in lines[i].chars() {
                if ch == '{' {
                    brace_count += 1;
                    found_first = true;
                } else if ch == '}' {
                    brace_count -= 1;
                    if found_first && brace_count == 0 {
                        return i;
                    }
                }
            }
        }

        lines.len() - 1
    }

    /// Get indentation level of a line
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

    /// Calculate cyclomatic complexity of a function
    fn calculate_complexity(&self, function_lines: &[&str]) -> usize {
        let mut complexity = 1;

        // Keywords that increase complexity
        let keywords = [
            "if", "else", "for", "while", "switch", "case", "catch", "match", "loop", "elif",
            "except", "finally",
        ];

        // Operators that increase complexity
        let operators = ["&&", "||", "?"];

        for line in function_lines {
            // Count keyword occurrences
            for keyword in &keywords {
                complexity += line.matches(keyword).count();
            }

            // Count operator occurrences
            for operator in &operators {
                complexity += line.matches(operator).count();
            }
        }

        complexity
    }
}

/// Information about a detected function
struct FunctionInfo {
    name: String,
    param_count: usize,
}
