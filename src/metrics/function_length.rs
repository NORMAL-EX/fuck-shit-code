use crate::i18n::Translator;
use crate::metrics::{Metric, MetricResult};
use crate::parser::ParseResult;

pub struct FunctionLengthMetric {
    translator: Translator,
}

impl FunctionLengthMetric {
    pub fn new(translator: Translator) -> Self {
        FunctionLengthMetric { translator }
    }
}

impl Metric for FunctionLengthMetric {
    fn name(&self) -> &str {
        "状态管理"
    }
    
    fn description(&self) -> &str {
        "检测代码中状态变量的管理，良好的状态管理能提高代码可维护性和可预测性"
    }
    
    fn weight(&self) -> f64 {
        0.2
    }
    
    fn analyze(&self, parse_result: &dyn ParseResult) -> MetricResult {  // 使用 dyn
        let functions = parse_result.get_functions();
        let mut issues = Vec::new();
        
        if functions.is_empty() {
            return MetricResult {
                score: 0.0,
                weight: self.weight(),
                description: self.description().to_string(),
                issues,
            };
        }
        
        let mut long_functions = 0;
        let mut very_long_functions = 0;
        let mut extreme_long_functions = 0;
        
        for func in functions {
            let line_count = func.end_line - func.start_line + 1;
            
            if line_count > 120 {
                issues.push(format!("函数 '{}' 极度过长 ({} 行)，必须拆分", func.name, line_count));
                extreme_long_functions += 1;
            } else if line_count > 70 {
                issues.push(format!("函数 '{}' 过长 ({} 行)，建议拆分", func.name, line_count));
                very_long_functions += 1;
            } else if line_count > 40 {
                issues.push(format!("函数 '{}' 较长 ({} 行)，可考虑重构", func.name, line_count));
                long_functions += 1;
            }
            
            if func.complexity > 18 {
                issues.push(format!("函数 '{}' 复杂度严重过高 ({})，必须简化", func.name, func.complexity));
            } else if func.complexity > 12 {
                issues.push(format!("函数 '{}' 复杂度过高 ({})，建议简化", func.name, func.complexity));
            }
            
            if func.parameters > 8 {
                issues.push(format!("函数 '{}' 参数极多 ({} 个)，必须使用结构体封装", func.name, func.parameters));
            } else if func.parameters > 6 {
                issues.push(format!("函数 '{}' 参数过多 ({} 个)，建议使用结构体封装", func.name, func.parameters));
            }
        }
        
        let total_functions = functions.len() as f64;
        let long_ratio = long_functions as f64 / total_functions;
        let very_long_ratio = very_long_functions as f64 / total_functions;
        let extreme_long_ratio = extreme_long_functions as f64 / total_functions;
        
        let score = long_ratio * 0.3 + very_long_ratio * 0.5 + extreme_long_ratio * 0.8;
        let score = if score > 1.0 { 1.0 } else { score };
        
        MetricResult {
            score,
            weight: self.weight(),
            description: self.description().to_string(),
            issues,
        }
    }
}