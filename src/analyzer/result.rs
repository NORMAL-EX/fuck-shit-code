//! # 分析结果定义
//! 
//! 定义分析结果的数据结构

use std::collections::HashMap;
use crate::metrics::MetricResult;

/// 分析结果
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// 代码质量得分（0-1）
    pub code_quality_score: f64,
    
    /// 各项指标结果
    pub metrics: HashMap<String, MetricResult>,
    
    /// 分析的文件列表
    pub files_analyzed: Vec<FileAnalysisResult>,
    
    /// 总文件数
    pub total_files: usize,
    
    /// 总代码行数
    pub total_lines: usize,
    
    /// 是否为空项目
    pub is_empty: bool,
}

/// 文件分析结果
#[derive(Debug, Clone)]
pub struct FileAnalysisResult {
    /// 文件路径
    pub file_path: String,
    
    /// 文件得分
    pub file_score: f64,
    
    /// 发现的问题
    pub issues: Vec<String>,
}