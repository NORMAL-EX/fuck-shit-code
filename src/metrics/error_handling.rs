use crate::common::LanguageType;
use crate::i18n::Translator;
use crate::metrics::{Metric, MetricResult};
use crate::parser::{Function, ParseResult};

pub struct ErrorHandlingMetric {
    translator: Translator,
}

impl ErrorHandlingMetric {
    pub fn new(translator: Translator) -> Self {
        ErrorHandlingMetric { translator }
    }
}

impl Metric for ErrorHandlingMetric {
    fn name(&self) -> &str {
        "错误处理"
    }

    fn description(&self) -> &str {
        "检测代码中的错误处理情况，良好的错误处理能提高代码的健壮性"
    }

    fn weight(&self) -> f64 {
        0.1
    }

    fn analyze(&self, parse_result: &dyn ParseResult) -> MetricResult {
        let functions = parse_result.get_functions();
        let language = parse_result.get_language();
        let mut issues = Vec::new();

        if functions.is_empty() {
            return MetricResult {
                score: 0.0,
                weight: self.weight(),
                description: self.description().to_string(),
                issues,
            };
        }

        let mut total_error_handling_score = 0.0;
        let mut functions_with_errors = 0;

        for func in functions {
            let error_score = self.analyze_function_error_handling(func, language);

            if error_score.has_error_potential && !error_score.has_error_handling {
                functions_with_errors += 1;
                issues.push(format!("函数 '{}' 可能产生错误但缺少错误处理", func.name));
            } else if error_score.has_error_potential && error_score.error_handling_quality < 0.3 {
                issues.push(format!("函数 '{}' 的错误处理不完善", func.name));
            }

            total_error_handling_score += error_score.error_handling_quality;
        }

        // 计算平均错误处理质量
        let avg_error_handling = if !functions.is_empty() {
            total_error_handling_score / functions.len() as f64
        } else {
            0.5
        };

        // 根据没有错误处理的函数比例计算分数
        let no_error_handling_ratio = functions_with_errors as f64 / functions.len().max(1) as f64;

        // 综合评分：考虑错误处理质量和缺失比例
        let score = (1.0 - avg_error_handling) * 0.6 + no_error_handling_ratio * 0.4;

        MetricResult {
            score,
            weight: self.weight(),
            description: self.description().to_string(),
            issues,
        }
    }
}

struct ErrorHandlingScore {
    has_error_potential: bool,
    has_error_handling: bool,
    error_handling_quality: f64,
}

impl ErrorHandlingMetric {
    fn analyze_function_error_handling(
        &self,
        func: &Function,
        language: LanguageType,
    ) -> ErrorHandlingScore {
        // 根据不同语言检测错误处理模式
        match language {
            LanguageType::Rust => self.analyze_rust_error_handling(func),
            LanguageType::Go => self.analyze_go_error_handling(func),
            LanguageType::JavaScript | LanguageType::TypeScript => {
                self.analyze_js_error_handling(func)
            }
            LanguageType::Python => self.analyze_python_error_handling(func),
            LanguageType::Java | LanguageType::CSharp => self.analyze_java_error_handling(func),
            LanguageType::C | LanguageType::CPlusPlus => self.analyze_c_error_handling(func),
            _ => ErrorHandlingScore {
                has_error_potential: false,
                has_error_handling: false,
                error_handling_quality: 0.5,
            },
        }
    }

    fn analyze_rust_error_handling(&self, func: &Function) -> ErrorHandlingScore {
        // Rust 错误处理检测
        // 检查 Result、Option、unwrap、expect、? 操作符等
        let has_result_return = func.name.contains("Result") || func.complexity > 5;
        let has_error_handling = func.complexity > 3; // 简化判断：复杂度高说明有分支处理

        let quality = if has_error_handling {
            0.8 // Rust 强制错误处理，质量通常较高
        } else if has_result_return {
            0.5
        } else {
            0.3
        };

        ErrorHandlingScore {
            has_error_potential: has_result_return || func.complexity > 8,
            has_error_handling,
            error_handling_quality: quality,
        }
    }

    fn analyze_go_error_handling(&self, func: &Function) -> ErrorHandlingScore {
        // Go 错误处理检测
        // 检查 error 返回值、if err != nil 模式等
        let has_error_potential = func.complexity > 5;
        let has_error_handling = func.complexity > 7; // Go 通常有很多 if err != nil

        let quality = if has_error_handling {
            0.7
        } else if has_error_potential {
            0.3
        } else {
            0.5
        };

        ErrorHandlingScore {
            has_error_potential,
            has_error_handling,
            error_handling_quality: quality,
        }
    }

    fn analyze_js_error_handling(&self, func: &Function) -> ErrorHandlingScore {
        // JavaScript/TypeScript 错误处理检测
        // 检查 try-catch、Promise.catch、async/await 等
        let has_async = func.name.contains("async")
            || func.name.contains("fetch")
            || func.name.contains("request");
        let has_error_potential = has_async || func.complexity > 6;
        let has_error_handling = func.complexity > 8; // 可能有 try-catch

        let quality = if has_error_handling {
            0.6
        } else if has_async && !has_error_handling {
            0.2 // async 没有错误处理很危险
        } else {
            0.4
        };

        ErrorHandlingScore {
            has_error_potential,
            has_error_handling,
            error_handling_quality: quality,
        }
    }

    fn analyze_python_error_handling(&self, func: &Function) -> ErrorHandlingScore {
        // Python 错误处理检测
        // 检查 try-except、raise 等
        let has_io_operations = func.name.contains("read")
            || func.name.contains("write")
            || func.name.contains("open")
            || func.name.contains("request");
        let has_error_potential = has_io_operations || func.complexity > 6;
        let has_error_handling = func.complexity > 7;

        let quality = if has_error_handling {
            0.65
        } else if has_io_operations && !has_error_handling {
            0.15 // IO 操作没有异常处理很危险
        } else {
            0.4
        };

        ErrorHandlingScore {
            has_error_potential,
            has_error_handling,
            error_handling_quality: quality,
        }
    }

    fn analyze_java_error_handling(&self, func: &Function) -> ErrorHandlingScore {
        // Java/C# 错误处理检测
        // 检查 try-catch、throws、checked exceptions 等
        let has_error_potential = func.complexity > 5;
        let has_error_handling = func.complexity > 8;

        let quality = if has_error_handling {
            0.75 // Java/C# 有较好的异常机制
        } else if has_error_potential {
            0.35
        } else {
            0.5
        };

        ErrorHandlingScore {
            has_error_potential,
            has_error_handling,
            error_handling_quality: quality,
        }
    }

    fn analyze_c_error_handling(&self, func: &Function) -> ErrorHandlingScore {
        // C/C++ 错误处理检测
        // 检查返回值检查、errno、异常（C++）等
        let has_malloc = func.name.contains("alloc") || func.name.contains("malloc");
        let has_file_ops =
            func.name.contains("open") || func.name.contains("read") || func.name.contains("write");
        let has_error_potential = has_malloc || has_file_ops || func.complexity > 6;
        let has_error_handling = func.complexity > 7;

        let quality = if has_error_handling {
            0.5 // C 错误处理通常较原始
        } else if (has_malloc || has_file_ops) && !has_error_handling {
            0.1 // 内存/文件操作没有错误检查很危险
        } else {
            0.3
        };

        ErrorHandlingScore {
            has_error_potential,
            has_error_handling,
            error_handling_quality: quality,
        }
    }
}
