//! # 代码分析器
//! 
//! 核心分析逻辑的实现，负责协调各个组件完成代码分析

use crate::common::find_source_files;
use crate::config::AnalysisConfig;
use crate::error::{AppError, AppResult};
use crate::i18n::{Language, Translator};
use crate::metrics::{MetricFactory, MetricResult};
use crate::parser::{create_parser_for_file, ParseResult};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::result::{AnalysisResult, FileAnalysisResult};

/// 代码分析器主结构
pub struct CodeAnalyzer {
    /// 翻译器实例
    translator: Translator,
    
    /// 是否静默模式
    silent: bool,
    
    /// 度量工厂
    metric_factory: MetricFactory,
}

impl CodeAnalyzer {
    /// 创建新的代码分析器实例
    /// 
    /// # Returns
    /// * `Self` - 新的分析器实例
    pub fn new() -> Self {
        let translator = Translator::new(Language::ZhCN);
        let metric_factory = MetricFactory::new(translator.clone());
        
        CodeAnalyzer {
            translator,
            silent: false,
            metric_factory,
        }
    }
    
    /// 设置语言
    /// 
    /// # Arguments
    /// * `language` - 语言设置
    pub fn set_language(&mut self, language: Language) {
        self.translator = Translator::new(language);
        self.metric_factory = MetricFactory::new(self.translator.clone());
    }
    
    /// 设置静默模式
    /// 
    /// # Arguments
    /// * `silent` - 是否静默
    pub fn set_silent(&mut self, silent: bool) {
        self.silent = silent;
    }
    
    /// 使用配置进行分析
    /// 
    /// # Arguments
    /// * `path` - 分析路径
    /// * `config` - 分析配置
    /// 
    /// # Returns
    /// * `AppResult<AnalysisResult>` - 分析结果
    pub fn analyze_with_config(
        &self,
        path: &Path,
        config: &AnalysisConfig,
    ) -> AppResult<AnalysisResult> {
        // 验证路径
        self.validate_path(path)?;
        
        // 处理单文件情况
        if path.is_file() {
            return self.analyze_single_file(path);
        }
        
        // 搜索源文件
        let files = self.find_files(path, config)?;
        
        // 检查是否为空项目
        if files.is_empty() {
            return Ok(self.create_empty_result());
        }
        
        // 分析文件
        let file_results = self.analyze_files_parallel(&files)?;
        
        // 汇总结果
        self.aggregate_results(file_results)
    }
    
    /// 使用排除模式进行分析（向后兼容）
    /// 
    /// # Arguments
    /// * `path` - 分析路径
    /// * `include_patterns` - 包含模式
    /// * `exclude_patterns` - 排除模式
    /// 
    /// # Returns
    /// * `AppResult<AnalysisResult>` - 分析结果
    pub fn analyze_with_excludes(
        &self,
        path: &Path,
        include_patterns: &[String],
        exclude_patterns: &[String],
    ) -> AppResult<AnalysisResult> {
        let config = AnalysisConfig {
            include_patterns: include_patterns.to_vec(),
            exclude_patterns: exclude_patterns.to_vec(),
            ..Default::default()
        };
        
        self.analyze_with_config(path, &config)
    }
    
    /// 验证路径有效性
    /// 
    /// # Arguments
    /// * `path` - 要验证的路径
    /// 
    /// # Returns
    /// * `AppResult<()>` - 验证结果
    fn validate_path(&self, path: &Path) -> AppResult<()> {
        if !path.exists() {
            return Err(AppError::FileNotFound(path.to_path_buf()));
        }
        Ok(())
    }
    
    /// 查找源文件
    /// 
    /// # Arguments
    /// * `path` - 搜索路径
    /// * `config` - 配置
    /// 
    /// # Returns
    /// * `AppResult<Vec<PathBuf>>` - 找到的文件列表
    fn find_files(&self, path: &Path, config: &AnalysisConfig) -> AppResult<Vec<PathBuf>> {
        if !self.silent {
            self.print_search_progress();
        }
        
        let files = find_source_files(
            path,
            &config.include_patterns,
            &config.exclude_patterns,
            |count| {
                if !self.silent {
                    self.update_search_progress(count);
                }
            },
        )?;
        
        if !self.silent {
            self.print_files_found(files.len());
        }
        
        Ok(files)
    }
    
    /// 打印搜索进度
    fn print_search_progress(&self) {
        print!("🔍 {}", self.translator.translate("analyzer.searching_files"));
    }
    
    /// 更新搜索进度
    /// 
    /// # Arguments
    /// * `count` - 当前文件数
    fn update_search_progress(&self, count: usize) {
        print!(
            "\r🔍 {} {}",
            self.translator.translate("analyzer.searching_files"),
            count
        );
    }
    
    /// 打印找到的文件数
    /// 
    /// # Arguments
    /// * `count` - 文件数量
    fn print_files_found(&self, count: usize) {
        println!(
            "\r{}\r📂 {}: {}",
            " ".repeat(80),
            self.translator.translate("analyzer.files_found"),
            count
        );
    }
    
    /// 创建空项目结果
    /// 
    /// # Returns
    /// * `AnalysisResult` - 空结果
    fn create_empty_result(&self) -> AnalysisResult {
        if !self.silent {
            self.print_empty_project_message();
        }
        
        AnalysisResult {
            code_quality_score: 0.0,
            metrics: HashMap::new(),
            files_analyzed: vec![],
            total_files: 0,
            total_lines: 0,
            is_empty: true,
        }
    }
    
    /// 打印空项目消息
    fn print_empty_project_message(&self) {
        println!();
        println!("  {}", "╔════════════════════════════════════════╗".bright_yellow());
        println!("  {}", "║       🏜️  荒芜代码检测器  🏜️         ║".bright_yellow());
        println!("  {}", "╚════════════════════════════════════════╝".bright_yellow());
        println!();
        
        match self.translator.get_language() {
            Language::ZhCN => self.print_empty_message_zh(),
            Language::EnUS => self.print_empty_message_en(),
        }
        
        println!();
    }
    
    /// 打印中文空项目消息
    fn print_empty_message_zh(&self) {
        println!("  {}", "😅 检测到一片荒芜...".bright_cyan());
        println!("  {}", "📭 这里空空如也，连一行代码都没有！".yellow());
        println!();
        println!("  {}", "建议：".bright_magenta());
        println!("  {}", "1. 🎯 快去写点代码吧，不然我没东西可以吐槽了".green());
        println!("  {}", "2. 💡 或者检查一下路径是否正确？".green());
        println!("  {}", "3. 🤔 也可能是排除规则太严格了？".green());
        println!();
        println!("  {}", "💭 记住：空的项目是最干净的，但也是最没用的！".bright_blue());
    }
    
    /// 打印英文空项目消息
    fn print_empty_message_en(&self) {
        println!("  {}", "😅 Detected a wasteland...".bright_cyan());
        println!("  {}", "📭 It's empty here, not even a single line of code!".yellow());
        println!();
        println!("  {}", "Suggestions:".bright_magenta());
        println!("  {}", "1. 🎯 Go write some code, or I have nothing to roast!".green());
        println!("  {}", "2. 💡 Or check if the path is correct?".green());
        println!("  {}", "3. 🤔 Maybe the exclusion rules are too strict?".green());
        println!();
        println!("  {}", "💭 Remember: Empty projects are the cleanest, but also the most useless!".bright_blue());
    }
    
    /// 分析单个文件
    /// 
    /// # Arguments
    /// * `path` - 文件路径
    /// 
    /// # Returns
    /// * `AppResult<AnalysisResult>` - 分析结果
    fn analyze_single_file(&self, path: &Path) -> AppResult<AnalysisResult> {
        // 读取文件内容
        let content = self.read_file(path)?;
        
        // 解析文件
        let parse_result = self.parse_file(path, &content)?;
        
        // 分析指标
        let metrics = self.analyze_metrics(&*parse_result);
        
        // 计算得分
        let file_score = self.calculate_score(&metrics);
        
        // 收集问题
        let issues = self.collect_issues(&metrics);
        
        // 创建结果
        Ok(self.create_single_file_result(
            path,
            file_score,
            metrics,
            issues,
            parse_result.get_total_lines(),
        ))
    }
    
    /// 读取文件内容
    /// 
    /// # Arguments
    /// * `path` - 文件路径
    /// 
    /// # Returns
    /// * `AppResult<String>` - 文件内容
    fn read_file(&self, path: &Path) -> AppResult<String> {
        fs::read_to_string(path)
            .map_err(|e| AppError::Io(e))
    }
    
    /// 解析文件
    /// 
    /// # Arguments
    /// * `path` - 文件路径
    /// * `content` - 文件内容
    /// 
    /// # Returns
    /// * `AppResult<Box<dyn ParseResult>>` - 解析结果
    fn parse_file(&self, path: &Path, content: &str) -> AppResult<Box<dyn ParseResult>> {
        let parser = create_parser_for_file(path);
        parser.parse(path, content)
            .map_err(|e| AppError::ParseError(e.to_string()))
    }
    
    /// 分析指标
    /// 
    /// # Arguments
    /// * `parse_result` - 解析结果
    /// 
    /// # Returns
    /// * `HashMap<String, MetricResult>` - 指标结果
    fn analyze_metrics(&self, parse_result: &dyn ParseResult) -> HashMap<String, MetricResult> {
        let metrics = self.metric_factory.create_all_metrics();
        let mut results = HashMap::new();
        
        for metric in metrics {
            let result = metric.analyze(parse_result);
            results.insert(metric.name().to_string(), result);
        }
        
        results
    }
    
    /// 计算文件得分
    /// 
    /// # Arguments
    /// * `metrics` - 指标结果
    /// 
    /// # Returns
    /// * `f64` - 得分
    fn calculate_score(&self, metrics: &HashMap<String, MetricResult>) -> f64 {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;
        
        for result in metrics.values() {
            total_score += result.score * result.weight;
            total_weight += result.weight;
        }
        
        if total_weight > 0.0 {
            total_score / total_weight
        } else {
            0.0
        }
    }
    
    /// 收集问题
    /// 
    /// # Arguments
    /// * `metrics` - 指标结果
    /// 
    /// # Returns
    /// * `Vec<String>` - 问题列表
    fn collect_issues(&self, metrics: &HashMap<String, MetricResult>) -> Vec<String> {
        metrics.values()
            .flat_map(|result| result.issues.clone())
            .collect()
    }
    
    /// 创建单文件结果
    /// 
    /// # Arguments
    /// * `path` - 文件路径
    /// * `score` - 得分
    /// * `metrics` - 指标结果
    /// * `issues` - 问题列表
    /// * `lines` - 行数
    /// 
    /// # Returns
    /// * `AnalysisResult` - 分析结果
    fn create_single_file_result(
        &self,
        path: &Path,
        score: f64,
        metrics: HashMap<String, MetricResult>,
        issues: Vec<String>,
        lines: usize,
    ) -> AnalysisResult {
        AnalysisResult {
            code_quality_score: score,
            metrics,
            files_analyzed: vec![FileAnalysisResult {
                file_path: path.display().to_string(),
                file_score: score,
                issues,
            }],
            total_files: 1,
            total_lines: lines,
            is_empty: false,
        }
    }
    
    /// 并行分析多个文件
    /// 
    /// # Arguments
    /// * `files` - 文件列表
    /// 
    /// # Returns
    /// * `AppResult<Vec<FileAnalysisData>>` - 分析数据列表
    fn analyze_files_parallel(&self, files: &[PathBuf]) -> AppResult<Vec<FileAnalysisData>> {
        let results = Arc::new(Mutex::new(Vec::new()));
        let progress = self.create_progress_bar(files.len());
        
        // 并行处理文件
        files.par_iter().for_each(|file| {
            if let Ok(data) = self.analyze_file_safe(file) {
                let mut res = results.lock().unwrap();
                res.push(data);
            }
            
            if let Some(ref pb) = progress {
                pb.inc(1);
            }
        });
        
        if let Some(pb) = progress {
            pb.finish_and_clear();
        }
        
        Arc::try_unwrap(results)
            .map_err(|_| AppError::Other("Failed to unwrap results".to_string()))?
            .into_inner()
            .map_err(|_| AppError::Other("Failed to get inner mutex".to_string()))
    }
    
    /// 创建进度条
    /// 
    /// # Arguments
    /// * `total` - 总数
    /// 
    /// # Returns
    /// * `Option<ProgressBar>` - 进度条
    fn create_progress_bar(&self, total: usize) -> Option<ProgressBar> {
        if self.silent {
            return None;
        }
        
        let pb = ProgressBar::new(total as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        
        Some(pb)
    }
    
    /// 安全地分析文件（捕获错误）
    /// 
    /// # Arguments
    /// * `file` - 文件路径
    /// 
    /// # Returns
    /// * `AppResult<FileAnalysisData>` - 分析数据
    fn analyze_file_safe(&self, file: &PathBuf) -> AppResult<FileAnalysisData> {
        let content = self.read_file(file)?;
        let parse_result = self.parse_file(file, &content)?;
        let metrics = self.analyze_metrics(&*parse_result);
        let issues = self.collect_issues(&metrics);
        
        Ok(FileAnalysisData {
            path: file.clone(),
            metrics,
            issues,
            lines: parse_result.get_total_lines(),
        })
    }
    
    /// 汇总分析结果
    /// 
    /// # Arguments
    /// * `file_results` - 文件分析数据
    /// 
    /// # Returns
    /// * `AppResult<AnalysisResult>` - 汇总结果
    fn aggregate_results(&self, file_results: Vec<FileAnalysisData>) -> AppResult<AnalysisResult> {
        let mut total_lines = 0;
        let mut all_metrics: HashMap<String, Vec<MetricResult>> = HashMap::new();
        let mut files_analyzed = Vec::new();
        
        // 处理每个文件的结果
        for data in file_results {
            let file_score = self.calculate_score(&data.metrics);
            
            files_analyzed.push(FileAnalysisResult {
                file_path: data.path.display().to_string(),
                file_score,
                issues: data.issues,
            });
            
            // 收集指标
            for (name, result) in data.metrics {
                all_metrics.entry(name).or_insert_with(Vec::new).push(result);
            }
            
            total_lines += data.lines;
        }
        
        // 计算平均指标
        let aggregated_metrics = self.calculate_average_metrics(all_metrics);
        
        // 计算总体评分
        let code_quality_score = self.calculate_score(&aggregated_metrics);
        
        let total_files = files_analyzed.len();
        
        Ok(AnalysisResult {
            code_quality_score,
            metrics: aggregated_metrics,
            files_analyzed,
            total_files,
            total_lines,
            is_empty: false,
        })
    }
    
    /// 计算平均指标
    /// 
    /// # Arguments
    /// * `all_metrics` - 所有指标
    /// 
    /// # Returns
    /// * `HashMap<String, MetricResult>` - 平均指标
    fn calculate_average_metrics(
        &self,
        all_metrics: HashMap<String, Vec<MetricResult>>,
    ) -> HashMap<String, MetricResult> {
        let mut aggregated = HashMap::new();
        
        for (name, results) in all_metrics {
            if !results.is_empty() {
                let avg_score = results.iter()
                    .map(|r| r.score)
                    .sum::<f64>() / results.len() as f64;
                
                let first = &results[0];
                
                aggregated.insert(name, MetricResult {
                    score: avg_score,
                    weight: first.weight,
                    description: first.description.clone(),
                    issues: vec![],
                });
            }
        }
        
        aggregated
    }
}

/// 文件分析数据
#[derive(Debug)]
struct FileAnalysisData {
    /// 文件路径
    path: PathBuf,
    
    /// 指标结果
    metrics: HashMap<String, MetricResult>,
    
    /// 问题列表
    issues: Vec<String>,
    
    /// 代码行数
    lines: usize,
}