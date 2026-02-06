use crate::i18n::Translator;
use crate::metrics::{Metric, MetricResult};
use crate::parser::ParseResult;

pub struct NamingConventionMetric {
    translator: Translator,
}

impl NamingConventionMetric {
    pub fn new(translator: Translator) -> Self {
        NamingConventionMetric { translator }
    }
}

impl Metric for NamingConventionMetric {
    fn name(&self) -> &str {
        "命名规范"
    }

    fn description(&self) -> &str {
        "检测代码中的命名规范，良好的命名能提高代码可读性"
    }

    fn weight(&self) -> f64 {
        0.08
    }

    fn analyze(&self, parse_result: &dyn ParseResult) -> MetricResult {
        // 使用 dyn
        let functions = parse_result.get_functions();
        let mut issues = Vec::new();
        let mut bad_names = 0;

        for func in functions {
            if self.is_bad_name(&func.name) {
                issues.push(format!("函数名 '{}' 不符合规范", func.name));
                bad_names += 1;
            }
        }

        let bad_ratio = if !functions.is_empty() {
            bad_names as f64 / functions.len() as f64
        } else {
            0.0
        };

        let score = self.calculate_score(bad_ratio);

        MetricResult {
            score,
            weight: self.weight(),
            description: self.description().to_string(),
            issues,
        }
    }
}

impl NamingConventionMetric {
    fn is_bad_name(&self, name: &str) -> bool {
        // 检查是否是单字母变量名或太短的名称
        name.len() <= 2
            || name == "tmp"
            || name == "temp"
            || name == "xxx"
            || name == "foo"
            || name == "bar"
            || name == "test"
            || name.chars().all(|c| c == 'x' || c == 'y' || c == 'z')
    }

    fn calculate_score(&self, bad_ratio: f64) -> f64 {
        // 基础分0.4，每1%不良命名增加0.1分
        let base_score = 0.4;
        let increase_per_percent = 10.0;

        let score = base_score + (bad_ratio * increase_per_percent);

        if score > 1.0 {
            1.0
        } else {
            score
        }
    }
}
