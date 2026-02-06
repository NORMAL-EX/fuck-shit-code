//! # 报告生成模块
//!
//! 负责生成分析报告

mod console;
mod markdown;

use crate::analyzer::AnalysisResult;
use crate::i18n::Translator;

/// 报告生成器
pub struct Report {
    /// 分析结果
    result: AnalysisResult,

    /// 翻译器
    translator: Translator,
}

/// 报告选项
#[derive(Debug, Clone)]
pub struct ReportOptions {
    /// 是否详细模式
    pub verbose: bool,

    /// 显示的文件数量
    pub top_files: usize,

    /// 每个文件显示的问题数
    pub max_issues: usize,

    /// 是否只显示摘要
    pub summary_only: bool,

    /// 是否输出markdown
    pub markdown_output: bool,
}

impl Report {
    /// 创建新的报告生成器
    ///
    /// # Arguments
    /// * `result` - 分析结果
    ///
    /// # Returns
    /// * `Self` - 报告生成器实例
    pub fn new(result: AnalysisResult) -> Self {
        Report {
            result,
            translator: Translator::new(crate::i18n::Language::ZhCN),
        }
    }

    /// 设置翻译器
    ///
    /// # Arguments
    /// * `translator` - 翻译器
    pub fn set_translator(&mut self, translator: Translator) {
        self.translator = translator;
    }

    /// 生成控制台报告
    ///
    /// # Arguments
    /// * `options` - 报告选项
    pub fn generate_console_report(&self, options: &ReportOptions) {
        // 检查是否为空项目
        if self.result.is_empty {
            return;
        }

        // 选择输出格式
        if options.markdown_output {
            self.generate_markdown_output(options);
        } else {
            self.generate_console_output(options);
        }
    }

    /// 生成控制台输出
    ///
    /// # Arguments
    /// * `options` - 报告选项
    fn generate_console_output(&self, options: &ReportOptions) {
        let console_report = console::ConsoleReport::new(&self.result, &self.translator, options);

        console_report.generate();
    }

    /// 生成Markdown输出
    ///
    /// # Arguments
    /// * `options` - 报告选项
    fn generate_markdown_output(&self, options: &ReportOptions) {
        let markdown_report =
            markdown::MarkdownReport::new(&self.result, &self.translator, options);

        markdown_report.generate();
    }
}
