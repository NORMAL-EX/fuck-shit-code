//! # 配置模块
//! 
//! 提供应用程序的各种配置结构和默认值

use serde::{Deserialize, Serialize};

/// 分析配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// 包含的文件模式
    pub include_patterns: Vec<String>,
    
    /// 排除的文件模式
    pub exclude_patterns: Vec<String>,
    
    /// 是否启用并行分析
    pub parallel: bool,
    
    /// 最大文件大小（字节）
    pub max_file_size: usize,
    
    /// 最小文件大小（字节）
    pub min_file_size: usize,
}

impl Default for AnalysisConfig {
    /// 创建默认配置
    fn default() -> Self {
        AnalysisConfig {
            include_patterns: vec![],
            exclude_patterns: vec![],
            parallel: true,
            max_file_size: 10 * 1024 * 1024, // 10MB
            min_file_size: 1,
        }
    }
}

/// 输出配置
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// 是否显示详细信息
    pub verbose: bool,
    
    /// 显示的文件数量
    pub top_files: usize,
    
    /// 每个文件显示的问题数量
    pub max_issues: usize,
    
    /// 是否只显示摘要
    pub summary_only: bool,
    
    /// 是否输出Markdown格式
    pub markdown_output: bool,
}

impl Default for OutputConfig {
    /// 创建默认输出配置
    fn default() -> Self {
        OutputConfig {
            verbose: false,
            top_files: 5,
            max_issues: 5,
            summary_only: false,
            markdown_output: false,
        }
    }
}