use crate::common::LanguageType;
use crate::parser::{BaseParseResult, Function, ParseResult, Parser};
use regex::Regex;
use std::path::Path;

/// Parser for JavaScript source files
/// Detects functions, methods, arrow functions, and class methods
pub struct JavaScriptParser;

impl JavaScriptParser {
    pub fn new() -> Self {
        JavaScriptParser
    }
}

impl Parser for JavaScriptParser {
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
            language: LanguageType::JavaScript,
        }))
    }

    fn supported_languages(&self) -> Vec<LanguageType> {
        vec![LanguageType::JavaScript]
    }
}

impl JavaScriptParser {
    /// Count comment lines (both // and /* */ style)
    fn count_comment_lines(&self, lines: &[&str]) -> usize {
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

                // Check if it ends on same line
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
            }
        }

        count
    }

    /// Detect all types of JavaScript functions
    fn detect_functions(&self, lines: &[&str]) -> Vec<Function> {
        let mut functions = Vec::new();

        // Detect different function patterns
        functions.extend(self.detect_regular_functions(lines));
        functions.extend(self.detect_arrow_functions(lines));
        functions.extend(self.detect_class_methods(lines));

        // Sort by start line to maintain order
        functions.sort_by_key(|f| f.start_line);
        functions
    }

    /// Detect regular function declarations
    fn detect_regular_functions(&self, lines: &[&str]) -> Vec<Function> {
        let mut functions = Vec::new();
        let pattern = r"function\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\(";
        let regex = Regex::new(pattern).unwrap();

        for (i, line) in lines.iter().enumerate() {
            if let Some(captures) = regex.captures(line) {
                let name = captures.get(1).unwrap().as_str().to_string();
                let end_line = self.find_function_end(lines, i);
                let complexity = self.calculate_complexity(&lines[i..=end_line]);

                functions.push(Function {
                    name,
                    start_line: i + 1,
                    end_line: end_line + 1,
                    complexity,
                    parameters: 0, // Simplified
                });
            }
        }

        functions
    }

    /// Detect arrow functions and function expressions
    fn detect_arrow_functions(&self, lines: &[&str]) -> Vec<Function> {
        let mut functions = Vec::new();
        let patterns = [
            r"(?:const|let|var)\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=\s*\([^)]*\)\s*=>",
            r"([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=\s*function\s*\(",
        ];

        for pattern in &patterns {
            let regex = Regex::new(pattern).unwrap();

            for (i, line) in lines.iter().enumerate() {
                if let Some(captures) = regex.captures(line) {
                    let name = captures.get(1).unwrap().as_str().to_string();
                    let end_line = self.find_function_end(lines, i);
                    let complexity = self.calculate_complexity(&lines[i..=end_line]);

                    functions.push(Function {
                        name,
                        start_line: i + 1,
                        end_line: end_line + 1,
                        complexity,
                        parameters: 0,
                    });
                }
            }
        }

        functions
    }

    /// Detect class methods
    fn detect_class_methods(&self, lines: &[&str]) -> Vec<Function> {
        let mut functions = Vec::new();
        let pattern = r"^\s*([a-zA-Z_$][a-zA-Z0-9_$]*)\s*\([^)]*\)\s*\{";
        let regex = Regex::new(pattern).unwrap();

        let mut in_class = false;

        for (i, line) in lines.iter().enumerate() {
            // Check if we're entering a class
            if line.contains("class ") && line.contains("{") {
                in_class = true;
                continue;
            }

            // Check if we're leaving a class
            if in_class && line.trim() == "}" {
                in_class = false;
                continue;
            }

            // Look for methods inside classes
            if in_class {
                if let Some(captures) = regex.captures(line) {
                    let name = captures.get(1).unwrap().as_str().to_string();

                    // Skip constructor and common lifecycle methods
                    if name != "constructor" {
                        let end_line = self.find_function_end(lines, i);
                        let complexity = self.calculate_complexity(&lines[i..=end_line]);

                        functions.push(Function {
                            name,
                            start_line: i + 1,
                            end_line: end_line + 1,
                            complexity,
                            parameters: 0,
                        });
                    }
                }
            }
        }

        functions
    }

    /// Find where a function ends by tracking braces
    fn find_function_end(&self, lines: &[&str], start: usize) -> usize {
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

    /// Calculate cyclomatic complexity
    fn calculate_complexity(&self, function_lines: &[&str]) -> usize {
        let mut complexity = 1;

        // Control flow keywords
        let keywords = ["if", "else", "for", "while", "switch", "case", "catch"];

        // Logical operators
        let operators = ["&&", "||", "?"];

        for line in function_lines {
            // Count keywords
            for keyword in &keywords {
                let pattern = format!(" {} ", keyword);
                complexity += line.matches(&pattern).count();
            }

            // Count operators
            for operator in &operators {
                complexity += line.matches(operator).count();
            }
        }

        complexity
    }
}
