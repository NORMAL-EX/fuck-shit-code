//! # 语言检测模块
//! 
//! 提供编程语言检测功能

use std::path::Path;

/// 编程语言类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageType {
    /// Rust语言
    Rust,
    
    /// Go语言
    Go,
    
    /// JavaScript
    JavaScript,
    
    /// TypeScript
    TypeScript,
    
    /// Python
    Python,
    
    /// Java
    Java,
    
    /// C++
    CPlusPlus,
    
    /// C语言
    C,
    
    /// C#
    CSharp,
    
    /// PHP
    PHP,
    
    /// HTML
    HTML,
    
    /// CSS
    CSS,
    
    /// 不支持的语言
    Unsupported,
}

impl LanguageType {
    /// 根据文件扩展名判断语言类型
    /// 
    /// # Arguments
    /// * `ext` - 文件扩展名
    /// 
    /// # Returns
    /// * `Self` - 语言类型
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => LanguageType::Rust,
            "go" => LanguageType::Go,
            "js" | "mjs" | "cjs" => LanguageType::JavaScript,
            "ts" | "tsx" | "jsx" => LanguageType::TypeScript,
            "py" | "pyw" => LanguageType::Python,
            "java" => LanguageType::Java,
            "cpp" | "cc" | "cxx" | "hpp" | "h++" => LanguageType::CPlusPlus,
            "c" | "h" => LanguageType::C,
            "cs" | "razor" => LanguageType::CSharp,
            "php" | "php3" | "php4" | "php5" | "php7" | "php8" | "phtml" => LanguageType::PHP,
            "html" | "htm" | "xhtml" => LanguageType::HTML,
            "css" | "scss" | "sass" | "less" => LanguageType::CSS,
            _ => LanguageType::Unsupported,
        }
    }
    
    /// 获取语言的显示名称
    /// 
    /// # Returns
    /// * `&str` - 显示名称
    pub fn display_name(&self) -> &str {
        match self {
            LanguageType::Rust => "Rust",
            LanguageType::Go => "Go",
            LanguageType::JavaScript => "JavaScript",
            LanguageType::TypeScript => "TypeScript",
            LanguageType::Python => "Python",
            LanguageType::Java => "Java",
            LanguageType::CPlusPlus => "C++",
            LanguageType::C => "C",
            LanguageType::CSharp => "C#",
            LanguageType::PHP => "PHP",
            LanguageType::HTML => "HTML",
            LanguageType::CSS => "CSS",
            LanguageType::Unsupported => "Unknown",
        }
    }
}

/// 语言检测器
/// 
/// 负责检测文件的编程语言类型
pub struct LanguageDetector;

impl LanguageDetector {
    /// 创建新的语言检测器
    /// 
    /// # Returns
    /// * `Self` - 检测器实例
    pub fn new() -> Self {
        LanguageDetector
    }
    
    /// 检测文件的语言类型
    /// 
    /// # Arguments
    /// * `file_path` - 文件路径
    /// 
    /// # Returns
    /// * `LanguageType` - 语言类型
    pub fn detect_language(&self, file_path: &Path) -> LanguageType {
        file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(LanguageType::from_extension)
            .unwrap_or(LanguageType::Unsupported)
    }
    
    /// 判断是否为支持的文件
    /// 
    /// # Arguments
    /// * `file_path` - 文件路径
    /// 
    /// # Returns
    /// * `bool` - 是否支持
    pub fn is_supported_file(&self, file_path: &Path) -> bool {
        !matches!(self.detect_language(file_path), LanguageType::Unsupported)
    }
    
    /// 获取支持的文件扩展名列表
    /// 
    /// # Returns
    /// * `Vec<&str>` - 扩展名列表
    pub fn supported_extensions() -> Vec<&'static str> {
        vec![
            "rs",
            "go",
            "js", "mjs", "cjs",
            "ts", "tsx", "jsx",
            "py", "pyw",
            "java",
            "cpp", "cc", "cxx", "hpp", "h++",
            "c", "h",
            "cs", "razor",
            "php", "php3", "php4", "php5", "php7", "php8", "phtml",
            "html", "htm", "xhtml",
            "css", "scss", "sass", "less",
        ]
    }
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}