//! # 分析器模块
//!
//! 负责协调整个代码分析流程

mod analyzer;
mod result;

pub use analyzer::CodeAnalyzer;
pub use result::{AnalysisResult, FileAnalysisResult};
