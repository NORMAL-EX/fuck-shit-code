//! # 循环复杂度度量
//! 
//! 计算代码的循环复杂度

use crate::i18n::Translator;
use crate::metrics::{Metric, MetricResult};
use crate::parser::ParseResult;

/// 循环复杂度度量器
pub struct CyclomaticComplexityMetric {
    /// 翻译器
    translator: Translator,
}

impl CyclomaticComplexityMetric {
    /// 创建新的循环复杂度度量器
    /// 
    /// # Arguments
    /// * `translator` - 翻译器
    /// 
    /// # Returns
    /// * `Self` - 度量器实例
    pub fn new(translator: Translator) -> Self {
        CyclomaticComplexityMetric { translator }
    }
    
    /// 计算平均复杂度
    /// 
    /// # Arguments
    /// * `parse_result` - 解析结果
    /// 
    /// # Returns
    /// * `(f64, Vec<String>)` - (平均复杂度, 问题列表)
    fn calculate_average_complexity(&self, parse_result: &dyn ParseResult) -> (f64, Vec<String>) {
        let functions = parse_result.get_functions();
        
        if functions.is_empty() {
            return (0.0, vec![]);
        }
        
        let mut issues = Vec::new();
        let mut total_complexity = 0;
        
        // 分析每个函数
        for func in functions {
            total_complexity += func.complexity;
            
            // 检查复杂度问题
            if let Some(issue) = self.check_function_complexity(func) {
                issues.push(issue);
            }
        }
        
        let avg_complexity = total_complexity as f64 / functions.len() as f64;
        (avg_complexity, issues)
    }
    
    /// 检查函数复杂度
    /// 
    /// # Arguments
    /// * `func` - 函数信息
    /// 
    /// # Returns
    /// * `Option<String>` - 问题描述
    fn check_function_complexity(&self, func: &crate::parser::Function) -> Option<String> {
        if func.complexity > 15 {
            Some(format!(
                "函数 {} 的循环复杂度过高 ({}), 考虑重构",
                func.name, func.complexity
            ))
        } else if func.complexity > 10 {
            Some(format!(
                "函数 {} 的循环复杂度较高 ({}), 建议简化",
                func.name, func.complexity
            ))
        } else {
            None
        }
    }
    
    /// 计算复杂度得分
    /// 
    /// # Arguments
    /// * `avg_complexity` - 平均复杂度
    /// 
    /// # Returns
    /// * `f64` - 得分（0-1）
    fn calculate_score(&self, avg_complexity: f64) -> f64 {
        // 基础分0.4，每点复杂度增加0.1分
        let base_score = 0.4;
        let increase_per_level = 0.1;
        
        let score = base_score + (avg_complexity * increase_per_level);
        score.min(1.0)
    }
}

impl Metric for CyclomaticComplexityMetric {
    /// 获取指标名称
    fn name(&self) -> &str {
        "循环复杂度"
    }
    
    /// 获取指标描述
    fn description(&self) -> &str {
        "测量函数的控制流复杂度，复杂度越高，代码越难理解和测试"
    }
    
    /// 获取权重
    fn weight(&self) -> f64 {
        0.3
    }
    
    /// 分析复杂度
    /// 
    /// # Arguments
    /// * `parse_result` - 解析结果
    /// 
    /// # Returns
    /// * `MetricResult` - 度量结果
    fn analyze(&self, parse_result: &dyn ParseResult) -> MetricResult {
        let (avg_complexity, issues) = self.calculate_average_complexity(parse_result);
        let score = self.calculate_score(avg_complexity);
        
        MetricResult::new(
            score,
            self.weight(),
            self.description().to_string(),
            issues,
        )
    }
}