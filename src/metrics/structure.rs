use crate::i18n::Translator;
use crate::metrics::{Metric, MetricResult};
use crate::parser::ParseResult;

pub struct StructureAnalysisMetric {
    translator: Translator,
}

impl StructureAnalysisMetric {
    pub fn new(translator: Translator) -> Self {
        StructureAnalysisMetric { translator }
    }
}

impl Metric for StructureAnalysisMetric {
    fn name(&self) -> &str {
        "代码结构"
    }
    
    fn description(&self) -> &str {
        "检测代码的嵌套深度和引用复杂度，评估结构清晰度"
    }
    
    fn weight(&self) -> f64 {
        0.15
    }
    
    fn analyze(&self, parse_result: &dyn ParseResult) -> MetricResult {  // 使用 dyn
        let functions = parse_result.get_functions();
        let mut issues = Vec::new();
        
        // 分析嵌套深度（基于复杂度估算）
        let mut max_nesting_depth = 0;
        
        for func in functions {
            // 使用复杂度作为嵌套深度的估算
            let estimated_depth = (func.complexity as f64 / 3.0).ceil() as usize;
            
            if estimated_depth > max_nesting_depth {
                max_nesting_depth = estimated_depth;
            }
            
            if estimated_depth > 5 {
                issues.push(format!("函数 {} 嵌套深度过高 (估算 {} 层)，建议重构", func.name, estimated_depth));
            } else if estimated_depth > 3 {
                issues.push(format!("函数 {} 嵌套深度较高 (估算 {} 层)，考虑简化", func.name, estimated_depth));
            }
        }
        
        let score = self.calculate_score(max_nesting_depth);
        
        MetricResult {
            score,
            weight: self.weight(),
            description: self.description().to_string(),
            issues,
        }
    }
}

impl StructureAnalysisMetric {
    fn calculate_score(&self, max_nesting_depth: usize) -> f64 {
        let nesting_score = if max_nesting_depth > 1 {
            0.4 + (max_nesting_depth as f64 - 1.0) * 0.15
        } else {
            0.4
        };
        
        if nesting_score > 1.0 {
            1.0
        } else {
            nesting_score
        }
    }
}