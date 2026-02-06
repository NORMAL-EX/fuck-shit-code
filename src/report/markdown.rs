//! # Markdown报告生成
//!
//! 生成Markdown格式的报告

use crate::analyzer::AnalysisResult;
use crate::i18n::Translator;
use crate::report::ReportOptions;

/// Markdown报告生成器
pub struct MarkdownReport<'a> {
    /// 分析结果
    result: &'a AnalysisResult,

    /// 翻译器
    translator: &'a Translator,

    /// 报告选项
    options: &'a ReportOptions,
}

impl<'a> MarkdownReport<'a> {
    /// 创建新的Markdown报告生成器
    ///
    /// # Arguments
    /// * `result` - 分析结果
    /// * `translator` - 翻译器
    /// * `options` - 报告选项
    ///
    /// # Returns
    /// * `Self` - 生成器实例
    pub fn new(
        result: &'a AnalysisResult,
        translator: &'a Translator,
        options: &'a ReportOptions,
    ) -> Self {
        MarkdownReport {
            result,
            translator,
            options,
        }
    }

    /// 生成报告
    pub fn generate(&self) {
        if self.result.is_empty {
            self.generate_empty_report();
        } else {
            self.generate_full_report();
        }
    }

    /// 生成空项目报告
    fn generate_empty_report(&self) {
        println!("# 🏜️ 荒芜代码检测报告\n");
        println!("## 😅 这里什么都没有！\n");
        println!("**建议**:");
        println!("- 快去写点代码吧");
        println!("- 或者检查路径是否正确");
        println!("- 也可能是排除规则太严格了\n");
        println!("> 空的项目是最干净的，但也是最没用的！");
    }

    /// 生成完整报告
    fn generate_full_report(&self) {
        self.print_title();
        self.print_summary();
        self.print_metrics_table();

        if !self.options.summary_only {
            self.print_problem_files();
        }

        self.print_recommendations();
    }

    /// 打印标题
    fn print_title(&self) {
        println!("# 🌸 {} 🌸\n", self.translator.translate("report.title"));
    }

    /// 打印摘要
    fn print_summary(&self) {
        println!(
            "## {}\n",
            self.translator.translate("report.overall_assessment")
        );

        println!(
            "- **{}**: {:.2}/100",
            self.translator.translate("report.quality_score"),
            self.result.code_quality_score * 100.0
        );

        let level = self.get_quality_level(self.result.code_quality_score);
        println!(
            "- **{}**: {} - {}",
            self.translator.translate("report.quality_level"),
            self.translator.translate(level.0),
            self.translator.translate(level.1)
        );

        println!(
            "- **{}**: {}",
            self.translator.translate("report.analyzed_files"),
            self.result.total_files
        );

        println!(
            "- **{}**: {}\n",
            self.translator.translate("report.total_lines"),
            self.result.total_lines
        );
    }

    /// 打印指标表格
    fn print_metrics_table(&self) {
        println!(
            "## {}\n",
            self.translator.translate("report.quality_metrics")
        );

        // 打印表头
        self.print_table_header();

        // 排序并打印指标
        let mut metrics: Vec<_> = self.result.metrics.iter().collect();
        metrics.sort_by(|a, b| a.1.score.partial_cmp(&b.1.score).unwrap());

        for (name, result) in metrics {
            self.print_metric_row(name, result);
        }

        println!();
    }

    /// 打印表格头部
    fn print_table_header(&self) {
        println!(
            "| {} | {} | {} | {} |",
            self.translator.translate("report.metric"),
            self.translator.translate("report.score"),
            self.translator.translate("report.weight"),
            self.translator.translate("report.status")
        );

        println!("|------|------|------|------|");
    }

    /// 打印指标行
    ///
    /// # Arguments
    /// * `name` - 指标名称
    /// * `result` - 指标结果
    fn print_metric_row(&self, name: &str, result: &crate::metrics::MetricResult) {
        let score_percentage = result.score * 100.0;
        let status_emoji = self.get_status_emoji(score_percentage);

        println!(
            "| {} | {:.2} | {:.2} | {} |",
            name, score_percentage, result.weight, status_emoji
        );
    }

    /// 获取状态表情
    ///
    /// # Arguments
    /// * `score` - 分数
    ///
    /// # Returns
    /// * `&str` - 表情
    fn get_status_emoji(&self, score: f64) -> &str {
        match score {
            s if s < 20.0 => "✓✓",
            s if s < 35.0 => "✓",
            s if s < 50.0 => "○",
            s if s < 60.0 => "•",
            s if s < 70.0 => "⚠",
            s if s < 80.0 => "!",
            s if s < 90.0 => "!!",
            _ => "✗",
        }
    }

    /// 打印问题文件
    fn print_problem_files(&self) {
        println!(
            "## {} (Top {})\n",
            self.translator.translate("report.problem_files"),
            self.options.top_files
        );

        let mut files = self.result.files_analyzed.clone();
        files.sort_by(|a, b| b.file_score.partial_cmp(&a.file_score).unwrap());

        if files.is_empty() {
            println!("🎉 {}\n", self.translator.translate("report.no_issues"));
            return;
        }

        let max_files = self.options.top_files.min(files.len());

        for i in 0..max_files {
            self.print_file_section(i, &files[i]);
        }
    }

    /// 打印文件部分
    ///
    /// # Arguments
    /// * `index` - 索引
    /// * `file` - 文件分析结果
    fn print_file_section(&self, index: usize, file: &crate::analyzer::FileAnalysisResult) {
        println!(
            "### {}. {} ({}: {:.2})",
            index + 1,
            file.file_path,
            self.translator.translate("report.score"),
            file.file_score * 100.0
        );

        if !file.issues.is_empty() {
            println!("**{}**:", self.translator.translate("report.main_issues"));

            for issue in &file.issues {
                println!("- {}", issue);
            }
        }

        println!();
    }

    /// 打印改进建议
    fn print_recommendations(&self) {
        println!(
            "## {}\n",
            self.translator.translate("report.improvement_suggestions")
        );

        match self.result.code_quality_score {
            s if s < 0.3 => self.print_good_recommendations(),
            s if s < 0.6 => self.print_moderate_recommendations(),
            _ => self.print_bad_recommendations(),
        }
    }

    /// 打印良好代码的建议
    fn print_good_recommendations(&self) {
        println!("### {}", self.translator.translate("advice.priority.high"));
        println!("- {}\n", self.translator.translate("advice.good.maintain"));

        println!(
            "### {}",
            self.translator.translate("advice.priority.medium")
        );
        println!("- {}", self.translator.translate("advice.good.optimize"));
        println!("- {}\n", self.translator.translate("advice.good.document"));
    }

    /// 打印中等代码的建议
    fn print_moderate_recommendations(&self) {
        println!("### {}", self.translator.translate("advice.priority.high"));
        println!(
            "- {}",
            self.translator.translate("advice.moderate.refactor")
        );
        println!(
            "- {}\n",
            self.translator.translate("advice.moderate.complexity")
        );

        println!(
            "### {}",
            self.translator.translate("advice.priority.medium")
        );
        println!("- {}", self.translator.translate("advice.moderate.naming"));
        println!(
            "- {}",
            self.translator.translate("advice.moderate.comments")
        );
        println!(
            "- {}\n",
            self.translator.translate("advice.moderate.duplication")
        );
    }

    /// 打印较差代码的建议
    fn print_bad_recommendations(&self) {
        println!("### {}", self.translator.translate("advice.priority.high"));
        println!(
            "- {}",
            self.translator.translate("advice.bad.urgent_refactor")
        );
        println!("- {}", self.translator.translate("advice.bad.complexity"));
        println!(
            "- {}\n",
            self.translator.translate("advice.bad.error_handling")
        );

        println!(
            "### {}",
            self.translator.translate("advice.priority.medium")
        );
        println!("- {}", self.translator.translate("advice.bad.naming"));
        println!("- {}", self.translator.translate("advice.bad.duplication"));
        println!("- {}\n", self.translator.translate("advice.bad.comments"));
    }

    /// 获取质量等级
    ///
    /// # Arguments
    /// * `score` - 分数
    ///
    /// # Returns
    /// * `(&str, &str)` - (等级键, 描述键)
    fn get_quality_level(&self, score: f64) -> (&'static str, &'static str) {
        let adjusted_score = score * 100.0;

        match adjusted_score {
            s if s < 5.0 => ("level.clean", "level.clean.description"),
            s if s < 15.0 => ("level.mild", "level.mild.description"),
            s if s < 25.0 => ("level.moderate", "level.moderate.description"),
            s if s < 40.0 => ("level.bad", "level.bad.description"),
            s if s < 55.0 => ("level.terrible", "level.terrible.description"),
            s if s < 65.0 => ("level.disaster", "level.disaster.description"),
            s if s < 75.0 => ("level.disaster.severe", "level.disaster.severe.description"),
            s if s < 85.0 => (
                "level.disaster.very_bad",
                "level.disaster.very_bad.description",
            ),
            s if s < 95.0 => (
                "level.disaster.extreme",
                "level.disaster.extreme.description",
            ),
            s if s < 100.0 => ("level.disaster.worst", "level.disaster.worst.description"),
            _ => (
                "level.disaster.ultimate",
                "level.disaster.ultimate.description",
            ),
        }
    }
}
