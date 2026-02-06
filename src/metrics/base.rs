//! # 度量基础定义
//!
//! 定义度量指标的基础接口和数据结构

use crate::parser::ParseResult;

/// 度量结果
#[derive(Debug, Clone)]
pub struct MetricResult {
    /// 得分（0-1，越高越差）
    pub score: f64,

    /// 权重
    pub weight: f64,

    /// 描述
    pub description: String,

    /// 发现的问题
    pub issues: Vec<String>,
}

impl MetricResult {
    /// 创建新的度量结果
    ///
    /// # Arguments
    /// * `score` - 得分
    /// * `weight` - 权重
    /// * `description` - 描述
    /// * `issues` - 问题列表
    ///
    /// # Returns
    /// * `Self` - 度量结果实例
    pub fn new(score: f64, weight: f64, description: String, issues: Vec<String>) -> Self {
        MetricResult {
            score: score.min(1.0).max(0.0), // 确保在0-1范围内
            weight,
            description,
            issues,
        }
    }

    /// 获取加权得分
    ///
    /// # Returns
    /// * `f64` - 加权得分
    pub fn weighted_score(&self) -> f64 {
        self.score * self.weight
    }
}

/// 度量指标trait
pub trait Metric {
    /// 获取指标名称
    ///
    /// # Returns
    /// * `&str` - 名称
    fn name(&self) -> &str;

    /// 获取指标描述
    ///
    /// # Returns
    /// * `&str` - 描述
    fn description(&self) -> &str;

    /// 获取指标权重
    ///
    /// # Returns
    /// * `f64` - 权重
    fn weight(&self) -> f64;

    /// 分析代码
    ///
    /// # Arguments
    /// * `parse_result` - 解析结果
    ///
    /// # Returns
    /// * `MetricResult` - 度量结果
    fn analyze(&self, parse_result: &dyn ParseResult) -> MetricResult;
}
