use crate::i18n::Translator;
use crate::metrics::{Metric, MetricResult};
use crate::parser::ParseResult;

pub struct CommentRatioMetric {
    translator: Translator,
}

impl CommentRatioMetric {
    pub fn new(translator: Translator) -> Self {
        CommentRatioMetric { translator }
    }
}

impl Metric for CommentRatioMetric {
    fn name(&self) -> &str {
        "注释覆盖率"
    }
    
    fn description(&self) -> &str {
        "检测代码的注释覆盖率，良好的注释能提高代码可读性和可维护性"
    }
    
    fn weight(&self) -> f64 {
        0.15
    }
    
    fn analyze(&self, parse_result: &dyn ParseResult) -> MetricResult {  // 使用 dyn
        let total_lines = parse_result.get_total_lines();
        let comment_lines = parse_result.get_comment_lines();
        
        let comment_ratio = if total_lines > 0 {
            comment_lines as f64 / total_lines as f64
        } else {
            0.0
        };
        
        let mut issues = Vec::new();
        
        if comment_ratio < 0.05 {
            issues.push(format!("代码注释率极低 ({:.2}%)，几乎没有注释", comment_ratio * 100.0));
        } else if comment_ratio < 0.1 {
            issues.push(format!("代码注释率较低 ({:.2}%)，建议增加注释", comment_ratio * 100.0));
        }
        
        let score = self.calculate_score(comment_ratio);
        
        MetricResult {
            score,
            weight: self.weight(),
            description: self.description().to_string(),
            issues,
        }
    }
}

impl CommentRatioMetric {
    fn calculate_score(&self, ratio: f64) -> f64 {
        // 基础分0.9，每1%注释减少0.05分
        let base_score = 0.9;
        let reduce_per_percent = 0.05;
        let percentage_comment = ratio * 100.0;
        
        let score = base_score - (percentage_comment * reduce_per_percent);
        
        if score < 0.0 {
            0.0
        } else {
            score
        }
    }
}