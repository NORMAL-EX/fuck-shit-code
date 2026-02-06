//! # 控制台报告生成
//!
//! 生成格式化的控制台输出报告

use crate::analyzer::AnalysisResult;
use crate::i18n::Translator;
use crate::report::ReportOptions;
use colored::*;

/// 控制台报告生成器
pub struct ConsoleReport<'a> {
    /// 分析结果
    result: &'a AnalysisResult,

    /// 翻译器
    translator: &'a Translator,

    /// 报告选项
    options: &'a ReportOptions,
}

impl<'a> ConsoleReport<'a> {
    /// 创建新的控制台报告生成器
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
        ConsoleReport {
            result,
            translator,
            options,
        }
    }

    /// 生成报告
    pub fn generate(&self) {
        self.print_header();
        self.print_score_summary();

        if !self.options.summary_only {
            self.print_metrics();
            self.print_files();
        }

        self.print_conclusion();

        if self.options.verbose {
            self.print_verbose_details();
        }

        self.print_footer();
    }

    /// 打印报告头部
    fn print_header(&self) {
        self.print_divider();
        println!(
            "\n  🌸 {} 🌸",
            self.translator.translate("report.title").yellow().bold()
        );
        self.print_divider();
    }

    /// 打印分数摘要
    fn print_score_summary(&self) {
        println!();

        // 打印总分
        let display_score = self.result.code_quality_score * 100.0;
        print!(
            "  {}: {:.2} / 100",
            self.translator
                .translate("report.quality_score")
                .cyan()
                .bold(),
            display_score
        );

        print!(" - ");
        self.print_score_comment(self.result.code_quality_score);
        println!();

        // 打印质量等级
        let level = self.get_quality_level(self.result.code_quality_score);
        println!(
            "  {} - {}",
            format!(
                "{}: {}",
                self.translator.translate("report.quality_level"),
                self.translator.translate(&level.0)
            )
            .cyan(),
            self.translator.translate(&level.1).cyan()
        );

        println!();
    }

    /// 打印分数评语
    ///
    /// # Arguments
    /// * `score` - 分数
    fn print_score_comment(&self, score: f64) {
        let comment = self.get_score_comment(score);

        let colored_comment = match score {
            s if s < 0.2 => comment.green().bold(),
            s if s < 0.4 => comment.green(),
            s if s < 0.55 => comment.blue(),
            s if s < 0.7 => comment.yellow(),
            s if s < 0.85 => comment.bright_red(),
            _ => comment.red(),
        };

        print!("{}", colored_comment);
    }

    /// 获取分数评语
    ///
    /// # Arguments
    /// * `score` - 分数
    ///
    /// # Returns
    /// * `String` - 评语
    fn get_score_comment(&self, score: f64) -> String {
        let score = score * 100.0;
        let score_range = (score as i32 / 10) * 10;

        let key = format!("score.comment.{}", score_range.min(90));
        self.translator.translate(&key)
    }

    /// 打印指标详情
    fn print_metrics(&self) {
        println!(
            "\n◆ {}\n",
            self.translator
                .translate("report.metrics_details")
                .magenta()
                .bold()
        );

        // 排序指标
        let mut metrics: Vec<_> = self.result.metrics.iter().collect();
        metrics.sort_by(|a, b| a.1.score.partial_cmp(&b.1.score).unwrap());

        // 打印每个指标
        for (name, result) in &metrics {
            self.print_metric_item(name, result);
        }

        println!();
    }

    /// 打印单个指标
    ///
    /// # Arguments
    /// * `name` - 指标名称
    /// * `result` - 指标结果
    fn print_metric_item(&self, name: &str, result: &crate::metrics::MetricResult) {
        let score_percentage = result.score * 100.0;

        let status_emoji = self.get_status_emoji(score_percentage);
        let status_color = self.get_status_color(name, status_emoji, score_percentage);
        let comment = self.get_metric_comment(name, score_percentage);

        println!(
            "  {:<30} {:.2}分\t  {}",
            status_color,
            score_percentage,
            comment.cyan()
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

    /// 获取状态颜色
    ///
    /// # Arguments
    /// * `name` - 指标名称
    /// * `emoji` - 状态表情
    /// * `score` - 分数
    ///
    /// # Returns
    /// * `ColoredString` - 着色字符串
    fn get_status_color(&self, name: &str, emoji: &str, score: f64) -> ColoredString {
        let text = format!("{} {}", emoji, name);

        match score {
            s if s < 20.0 => text.green().bold(),
            s if s < 35.0 => text.green(),
            s if s < 50.0 => text.bright_cyan(),
            s if s < 60.0 => text.blue(),
            s if s < 70.0 => text.bright_yellow(),
            s if s < 80.0 => text.yellow(),
            s if s < 90.0 => text.bright_red(),
            _ => text.red(),
        }
    }

    /// 获取指标评语
    ///
    /// # Arguments
    /// * `metric_name` - 指标名称
    /// * `score` - 分数
    ///
    /// # Returns
    /// * `String` - 评语
    fn get_metric_comment(&self, metric_name: &str, score: f64) -> String {
        let level = match score {
            s if s < 20.0 => "good",
            s if s < 60.0 => "medium",
            _ => "bad",
        };

        let metric_type = self.detect_metric_type(metric_name);
        let key = format!("metric.{}.{}", metric_type, level);

        self.translator.translate(&key)
    }

    /// 检测指标类型
    ///
    /// # Arguments
    /// * `metric_name` - 指标名称
    ///
    /// # Returns
    /// * `&str` - 类型标识
    fn detect_metric_type(&self, metric_name: &str) -> &str {
        if metric_name.contains("复杂度") || metric_name.contains("complexity") {
            "complexity"
        } else if metric_name.contains("状态") || metric_name.contains("长度") {
            "length"
        } else if metric_name.contains("注释") {
            "comment"
        } else if metric_name.contains("错误") {
            "error"
        } else if metric_name.contains("命名") {
            "naming"
        } else if metric_name.contains("重复") {
            "duplication"
        } else if metric_name.contains("结构") {
            "structure"
        } else {
            "unknown"
        }
    }

    /// 打印文件列表
    fn print_files(&self) {
        if self.options.verbose {
            self.print_all_files();
        } else {
            self.print_top_files();
        }
    }

    /// 打印问题最多的文件
    fn print_top_files(&self) {
        println!(
            "\n◆ {}\n",
            self.translator
                .translate("report.worst_files")
                .magenta()
                .bold()
        );

        // 排序文件
        let mut files = self.result.files_analyzed.clone();
        files.sort_by(|a, b| b.file_score.partial_cmp(&a.file_score).unwrap());

        if files.is_empty() {
            println!(
                "  🎉 {}",
                self.translator.translate("report.no_issues").green().bold()
            );
            return;
        }

        // 打印前N个文件
        let max_files = self.options.top_files.min(files.len());
        for i in 0..max_files {
            self.print_file_item(i, &files[i]);
        }
    }

    /// 打印单个文件项
    ///
    /// # Arguments
    /// * `index` - 索引
    /// * `file` - 文件分析结果
    fn print_file_item(&self, index: usize, file: &crate::analyzer::FileAnalysisResult) {
        let score_color = self.get_score_color(file.file_score);

        println!(
            "  {}. {} ({})",
            (index + 1).to_string().white().bold(),
            self.shorten_path(&file.file_path).magenta(),
            format!("屎气指数: {:.2}", file.file_score * 100.0).color(score_color)
        );

        // 显示问题
        self.print_file_issues(file);

        if index < self.options.top_files - 1 {
            println!();
        }
    }

    /// 打印文件问题
    ///
    /// # Arguments
    /// * `file` - 文件分析结果
    fn print_file_issues(&self, file: &crate::analyzer::FileAnalysisResult) {
        let max_issues = self.options.max_issues.min(file.issues.len());

        for i in 0..max_issues {
            println!("     {}", file.issues[i].yellow());
        }

        if file.issues.len() > max_issues {
            println!(
                "     🔍 {}",
                format!("...还有 {} 个问题", file.issues.len() - max_issues).yellow()
            );
        }
    }

    /// 打印所有文件
    fn print_all_files(&self) {
        println!(
            "\n◆ {}\n",
            self.translator
                .translate("verbose.all_files")
                .magenta()
                .bold()
        );

        let mut files = self.result.files_analyzed.clone();
        files.sort_by(|a, b| b.file_score.partial_cmp(&a.file_score).unwrap());

        if files.is_empty() {
            println!(
                "  {}",
                self.translator
                    .translate("verbose.no_files_found")
                    .green()
                    .bold()
            );
            return;
        }

        for (i, file) in files.iter().enumerate() {
            self.print_file_item(i, file);
        }
    }

    /// 打印结论
    fn print_conclusion(&self) {
        println!(
            "\n◆ {}\n",
            self.translator
                .translate("report.conclusion")
                .magenta()
                .bold()
        );

        let level = self.get_quality_level(self.result.code_quality_score);

        println!(
            "  🌸 {} - {}\n",
            self.translator.translate(&level.0).cyan(),
            self.translator.translate(&level.1).cyan()
        );

        self.print_advice();

        println!();
    }

    /// 打印建议
    fn print_advice(&self) {
        let advice = match self.result.code_quality_score {
            s if s < 0.3 => self.translator.translate("advice.good").green().bold(),
            s if s < 0.6 => self.translator.translate("advice.moderate").yellow(),
            _ => self.translator.translate("advice.bad").red(),
        };

        println!("  {}", advice);
    }

    /// 打印详细信息
    fn print_verbose_details(&self) {
        println!(
            "\n◆ {}\n",
            self.translator
                .translate("verbose.basic_statistics")
                .magenta()
                .bold()
        );

        self.print_statistics();
        self.print_metric_details();
    }

    /// 打印统计信息
    fn print_statistics(&self) {
        println!(
            "  📊 {}",
            self.translator
                .translate("verbose.basic_statistics")
                .blue()
                .bold()
        );

        println!(
            "    {:<15} {}",
            self.translator.translate("verbose.total_files"),
            self.result.total_files
        );

        println!(
            "    {:<15} {}",
            self.translator.translate("verbose.total_lines"),
            self.result.total_lines
        );

        println!(
            "    {:<15} {}",
            self.translator.translate("verbose.total_issues"),
            self.get_total_issues()
        );
    }

    /// 打印指标详情
    fn print_metric_details(&self) {
        println!(
            "\n  🔍 {}",
            self.translator
                .translate("verbose.metric_details")
                .blue()
                .bold()
        );

        for (name, result) in &self.result.metrics {
            self.print_metric_detail(name, result);
        }
    }

    /// 打印单个指标详情
    ///
    /// # Arguments
    /// * `name` - 指标名称
    /// * `result` - 指标结果
    fn print_metric_detail(&self, name: &str, result: &crate::metrics::MetricResult) {
        println!(
            "\n    【{}】({} {:.2})",
            name.cyan(),
            self.translator.translate("verbose.weight"),
            result.weight
        );

        println!(
            "      {} {}",
            self.translator.translate("verbose.description"),
            result.description
        );

        println!(
            "      {} {:.2}/100",
            self.translator.translate("verbose.score"),
            result.score * 100.0
        );
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

    /// 获取分数颜色
    ///
    /// # Arguments
    /// * `score` - 分数
    ///
    /// # Returns
    /// * `Color` - 颜色
    fn get_score_color(&self, score: f64) -> Color {
        match score {
            s if s < 0.2 => Color::BrightGreen,
            s if s < 0.35 => Color::Green,
            s if s < 0.5 => Color::BrightCyan,
            s if s < 0.6 => Color::Blue,
            s if s < 0.7 => Color::BrightYellow,
            s if s < 0.8 => Color::Yellow,
            s if s < 0.9 => Color::BrightRed,
            _ => Color::Red,
        }
    }

    /// 缩短路径显示
    ///
    /// # Arguments
    /// * `path` - 路径
    ///
    /// # Returns
    /// * `String` - 缩短的路径
    fn shorten_path(&self, path: &str) -> String {
        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() <= 4 {
            path.to_string()
        } else {
            format!("./{}", parts[parts.len() - 3..].join("/"))
        }
    }

    /// 获取总问题数
    ///
    /// # Returns
    /// * `usize` - 问题总数
    fn get_total_issues(&self) -> usize {
        self.result
            .files_analyzed
            .iter()
            .map(|f| f.issues.len())
            .sum()
    }

    /// 打印分割线
    fn print_divider(&self) {
        println!("{}", "─".repeat(80));
    }

    /// 打印页脚
    fn print_footer(&self) {
        self.print_divider();
        println!();
    }
}
