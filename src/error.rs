//! # 错误处理模块
//! 
//! 定义应用程序的错误类型和错误处理逻辑

use thiserror::Error;
use std::io;
use std::path::PathBuf;

/// 应用程序错误类型
#[derive(Error, Debug)]
pub enum AppError {
    /// IO错误
    #[error("IO错误: {0}")]
    Io(#[from] io::Error),
    
    /// 文件未找到
    #[error("文件未找到: {0}")]
    FileNotFound(PathBuf),
    
    /// 路径无效
    #[error("路径无效: {0}")]
    InvalidPath(String),
    
    /// 解析错误
    #[error("解析错误: {0}")]
    ParseError(String),
    
    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    /// 分析错误
    #[error("分析错误: {0}")]
    AnalysisError(String),
    
    /// 通用错误（用于anyhow::Error转换）
    #[error("错误: {0}")]
    Anyhow(#[from] anyhow::Error),
    
    /// 其他错误
    #[error("其他错误: {0}")]
    Other(String),
}

/// 错误结果类型别名
pub type AppResult<T> = Result<T, AppError>;