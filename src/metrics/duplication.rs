use crate::i18n::Translator;
use crate::metrics::{Metric, MetricResult};
use crate::parser::{ParseResult, Function};
use std::collections::HashMap;

pub struct CodeDuplicationMetric {
    translator: Translator,
}

impl CodeDuplicationMetric {
    pub fn new(translator: Translator) -> Self {
        CodeDuplicationMetric { translator }
    }
}

impl Metric for CodeDuplicationMetric {
    fn name(&self) -> &str {
        "代码重复度"
    }
    
    fn description(&self) -> &str {
        "评估代码中重复逻辑的比例，重复代码越多，越需要抽象和重构"
    }
    
    fn weight(&self) -> f64 {
        0.15
    }
    
    fn analyze(&self, parse_result: &dyn ParseResult) -> MetricResult {
        let functions = parse_result.get_functions();
        let mut issues = Vec::new();
        
        if functions.len() < 2 {
            return MetricResult {
                score: 0.0,
                weight: self.weight(),
                description: self.description().to_string(),
                issues,
            };
        }
        
        // 多维度分析函数相似性
        let mut similarity_groups = self.group_similar_functions(functions);
        let mut duplication_score = 0.0;
        let mut total_duplicated_lines = 0;
        let total_lines: usize = functions.iter()
            .map(|f| f.end_line - f.start_line + 1)
            .sum();
        
        // 分析每个相似组
        for (pattern, group) in &mut similarity_groups {
            if group.len() > 1 {
                // 排序以便生成一致的输出
                group.sort_by(|a, b| a.name.cmp(&b.name));
                
                let similarity_score = self.calculate_similarity_score(pattern);
                
                if similarity_score > 0.7 {
                    // 高度相似
                    let duplicated_lines: usize = group.iter()
                        .skip(1) // 第一个不算重复
                        .map(|f| f.end_line - f.start_line + 1)
                        .sum();
                    
                    total_duplicated_lines += duplicated_lines;
                    
                    issues.push(format!(
                        "高度相似的函数（相似度 {:.0}%）: {}",
                        similarity_score * 100.0,
                        group.iter().map(|f| &f.name[..]).collect::<Vec<_>>().join(", ")
                    ));
                    
                    duplication_score += similarity_score * group.len() as f64;
                    
                } else if similarity_score > 0.5 {
                    // 中度相似
                    issues.push(format!(
                        "相似的函数结构: {}",
                        group.iter().map(|f| &f.name[..]).collect::<Vec<_>>().join(", ")
                    ));
                    
                    duplication_score += similarity_score * 0.5 * group.len() as f64;
                }
            }
        }
        
        // 检测命名模式重复（如 handleClick1, handleClick2, handleClick3）
        let naming_duplicates = self.detect_naming_pattern_duplication(functions);
        for (base_name, duplicates) in naming_duplicates {
            if duplicates.len() > 2 {
                issues.push(format!(
                    "发现重复的命名模式 '{}*': {} 个类似函数，建议使用更有意义的命名或合并逻辑",
                    base_name,
                    duplicates.len()
                ));
                duplication_score += 0.3 * duplicates.len() as f64;
            }
        }
        
        // 检测参数列表重复
        let param_duplicates = self.detect_parameter_duplication(functions);
        if param_duplicates > 3 {
            issues.push(format!(
                "发现 {} 个函数有相同的参数数量和复杂度，可能存在逻辑重复",
                param_duplicates
            ));
            duplication_score += 0.2 * param_duplicates as f64;
        }
        
        // 计算最终分数
        let duplication_ratio = if total_lines > 0 {
            total_duplicated_lines as f64 / total_lines as f64
        } else {
            0.0
        };
        
        // 综合评分
        let normalized_duplication_score = (duplication_score / functions.len() as f64).min(1.0);
        let score = duplication_ratio * 0.4 + normalized_duplication_score * 0.6;
        
        MetricResult {
            score: score.min(1.0),
            weight: self.weight(),
            description: self.description().to_string(),
            issues,
        }
    }
}

impl CodeDuplicationMetric {
    fn group_similar_functions<'a>(&self, functions: &'a [Function]) -> HashMap<String, Vec<&'a Function>> {
        let mut groups: HashMap<String, Vec<&'a Function>> = HashMap::new();
        
        for func in functions {
            let pattern = self.extract_function_pattern(func);
            groups.entry(pattern).or_insert_with(Vec::new).push(func);
        }
        
        groups
    }
    
    fn extract_function_pattern(&self, func: &Function) -> String {
        // 创建函数的特征模式
        let lines = func.end_line - func.start_line + 1;
        
        // 根据多个维度创建模式
        let size_category = match lines {
            0..=10 => "tiny",
            11..=30 => "small",
            31..=60 => "medium",
            61..=100 => "large",
            _ => "huge",
        };
        
        let complexity_category = match func.complexity {
            0..=3 => "trivial",
            4..=7 => "simple",
            8..=12 => "moderate",
            13..=20 => "complex",
            _ => "very_complex",
        };
        
        let param_category = match func.parameters {
            0 => "no_params",
            1 => "single_param",
            2..=3 => "few_params",
            4..=5 => "several_params",
            _ => "many_params",
        };
        
        // 检查函数名称模式
        let name_pattern = self.extract_name_pattern(&func.name);
        
        format!(
            "{}:{}:{}:{}:{}",
            size_category,
            complexity_category,
            param_category,
            lines, // 精确行数用于更细粒度的比较
            name_pattern
        )
    }
    
    fn extract_name_pattern(&self, name: &str) -> String {
        // 提取函数名的模式（去掉数字后缀等）
        let mut pattern = String::new();
        let mut has_uppercase = false;
        let mut has_underscore = false;
        
        for ch in name.chars() {
            if ch.is_uppercase() {
                has_uppercase = true;
            } else if ch == '_' {
                has_underscore = true;
            }
        }
        
        // 检查常见前缀
        let prefixes = ["get", "set", "handle", "process", "check", "validate", "init", "create", "update", "delete"];
        for prefix in &prefixes {
            if name.starts_with(prefix) {
                pattern.push_str(prefix);
                pattern.push('_');
                break;
            }
        }
        
        // 添加命名风格
        if has_uppercase {
            pattern.push_str("camel");
        } else if has_underscore {
            pattern.push_str("snake");
        } else {
            pattern.push_str("flat");
        }
        
        pattern
    }
    
    fn calculate_similarity_score(&self, pattern: &str) -> f64 {
        let parts: Vec<&str> = pattern.split(':').collect();
        
        if parts.len() < 4 {
            return 0.0;
        }
        
        let mut score: f64 = 0.0;  // 明确指定类型为 f64
        
        // 相同大小类别
        if parts[0] == "tiny" || parts[0] == "small" {
            score += 0.2;
        } else if parts[0] == "medium" {
            score += 0.3;
        } else {
            score += 0.4;
        }
        
        // 相同复杂度类别
        match parts[1] {
            "trivial" => score += 0.1,
            "simple" => score += 0.2,
            "moderate" => score += 0.3,
            "complex" | "very_complex" => score += 0.4,
            _ => {}
        }
        
        // 相同参数类别
        if parts[2] != "no_params" {
            score += 0.2;
        }
        
        // 如果行数完全相同，增加相似度
        if let Ok(lines) = parts[3].parse::<usize>() {
            if lines > 20 {
                score += 0.2; // 长函数行数相同更可能是复制
            } else if lines > 10 {
                score += 0.1;
            }
        }
        
        score.min(1.0)
    }
    
    fn detect_naming_pattern_duplication<'a>(&self, functions: &'a [Function]) -> HashMap<String, Vec<&'a Function>> {
        let mut pattern_groups: HashMap<String, Vec<&'a Function>> = HashMap::new();
        
        for func in functions {
            // 去掉末尾的数字和常见后缀
            let base_name = self.get_base_name(&func.name);
            
            if !base_name.is_empty() && base_name != func.name {
                pattern_groups.entry(base_name).or_insert_with(Vec::new).push(func);
            }
        }
        
        // 只保留有多个函数的组
        pattern_groups.retain(|_, v| v.len() > 1);
        pattern_groups
    }
    
    fn get_base_name(&self, name: &str) -> String {
        // 去掉数字后缀
        let mut base = name.trim_end_matches(|c: char| c.is_numeric());
        
        // 去掉常见的编号后缀
        let suffixes = ["_v2", "_v3", "_new", "_old", "_temp", "_tmp", "_copy", "_backup"];
        for suffix in &suffixes {
            if base.ends_with(suffix) {
                base = &base[..base.len() - suffix.len()];
                break;
            }
        }
        
        // 如果去掉后缀后名字太短，返回空
        if base.len() < 3 {
            String::new()
        } else {
            base.to_string()
        }
    }
    
    fn detect_parameter_duplication(&self, functions: &[Function]) -> usize {
        let mut param_signatures: HashMap<(usize, usize), usize> = HashMap::new();
        
        for func in functions {
            let signature = (func.parameters, func.complexity);
            *param_signatures.entry(signature).or_insert(0) += 1;
        }
        
        // 计算有相同签名的函数数量
        param_signatures.values()
            .filter(|&&count| count > 1)
            .map(|&count| count)
            .sum()
    }
}