//! # 屎山代码检测器 - 主入口
//! 
//! 这是一个专门用于分析代码质量的工具，能够客观评估代码的各项指标，
//! 并生成详细的质量报告。
//! 
//! ## 主要功能
//! - 分析代码复杂度
//! - 检测代码重复
//! - 评估注释覆盖率
//! - 检查命名规范
//! - 分析代码结构
//! - 评估错误处理质量

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;
use anyhow::Result;

mod analyzer;
mod common;
mod i18n;
mod metrics;
mod parser;
mod report;
mod config;
mod error;

use crate::analyzer::CodeAnalyzer;
use crate::config::{AnalysisConfig, OutputConfig};
use crate::i18n::{Language, Translator};
use crate::report::{Report, ReportOptions};

/// 命令行参数解析结构
#[derive(Parser)]
#[command(name = "fuck-shit-code")]
#[command(version = "1.0.0")]
#[command(about = "💻 屎山代码检测器 - 客观评估您的代码质量", long_about = None)]
struct Cli {
    /// 要分析的路径
    path: Option<PathBuf>,

    /// 指定输出语言（支持：zh-CN, en-US，默认：zh-CN）
    #[arg(short = 'l', long, default_value = "zh-CN")]
    lang: String,

    /// 显示详细分析报告
    #[arg(short = 'v', long)]
    verbose: bool,

    /// 显示问题最多的文件数量
    #[arg(short = 't', long, default_value = "5")]
    top: usize,

    /// 每个文件显示多少条问题
    #[arg(short = 'i', long, default_value = "5")]
    issues: usize,

    /// 只看结论，过程略过
    #[arg(short = 's', long)]
    summary: bool,

    /// 输出Markdown格式的精简报告
    #[arg(short = 'm', long)]
    markdown: bool,

    /// 排除的文件/目录模式
    #[arg(short = 'e', long)]
    exclude: Vec<String>,

    /// 跳过所有 index.js/index.ts 文件
    #[arg(short = 'x', long)]
    skipindex: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

/// 子命令定义
#[derive(Subcommand)]
enum Commands {
    /// 分析代码质量并输出评分
    Analyze {
        /// 要分析的路径
        path: Option<PathBuf>,

        /// 指定输出语言
        #[arg(short = 'l', long, default_value = "zh-CN")]
        lang: String,

        /// 显示详细分析报告
        #[arg(short = 'v', long)]
        verbose: bool,

        /// 显示问题最多的文件数量
        #[arg(short = 't', long, default_value = "5")]
        top: usize,

        /// 每个文件显示多少条问题
        #[arg(short = 'i', long, default_value = "5")]
        issues: usize,

        /// 只看结论，过程略过
        #[arg(short = 's', long)]
        summary: bool,

        /// 输出Markdown格式的精简报告
        #[arg(short = 'm', long)]
        markdown: bool,

        /// 排除的文件/目录模式
        #[arg(short = 'e', long)]
        exclude: Vec<String>,

        /// 跳过所有 index.js/index.ts 文件
        #[arg(short = 'x', long)]
        skipindex: bool,
    },
}

/// 程序主入口
/// 
/// 解析命令行参数，初始化配置，并执行代码分析
fn main() {
    // 初始化日志系统
    init_logger();
    
    // 解析命令行参数
    let cli = Cli::parse();
    
    // 执行主逻辑
    if let Err(e) = run(cli) {
        eprintln!("错误: {}", e);
        process::exit(1);
    }
}

/// 初始化日志系统
fn init_logger() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn")
    ).init();
}

/// 主要执行逻辑
/// 
/// # Arguments
/// * `cli` - 命令行参数
/// 
/// # Returns
/// * `Result<()>` - 执行结果
fn run(cli: Cli) -> Result<()> {
    // 解析语言设置
    let language = parse_language(&cli.lang);
    let translator = Translator::new(language);
    
    // 获取分析路径
    let path = get_analysis_path(&cli, &translator)?;
    
    // 构建分析配置
    let config = build_analysis_config(&cli)?;
    
    // 构建输出配置
    let output_config = build_output_config(&cli);
    
    // 执行分析
    execute_analysis(path, config, output_config, translator)?;
    
    Ok(())
}

/// 解析语言设置
/// 
/// # Arguments
/// * `lang_str` - 语言字符串
/// 
/// # Returns
/// * `Language` - 语言枚举
fn parse_language(lang_str: &str) -> Language {
    match lang_str.to_lowercase().as_str() {
        "en" | "en-us" | "english" => Language::EnUS,
        _ => Language::ZhCN,
    }
}

/// 获取要分析的路径
/// 
/// # Arguments
/// * `cli` - 命令行参数
/// * `translator` - 翻译器
/// 
/// # Returns
/// * `Result<PathBuf>` - 分析路径
fn get_analysis_path(cli: &Cli, translator: &Translator) -> Result<PathBuf> {
    // 从命令或参数中获取路径
    let path = match &cli.command {
        Some(Commands::Analyze { path, .. }) => {
            path.clone()
        }
        None => cli.path.clone(),
    };
    
    // 如果没有提供路径，显示帮助信息
    match path {
        Some(p) => Ok(p),
        None => {
            if cli.command.is_none() && cli.path.is_none() {
                show_help_and_exit(translator);
            }
            Ok(PathBuf::from("."))
        }
    }
}

/// 显示帮助信息并退出
/// 
/// # Arguments
/// * `translator` - 翻译器
fn show_help_and_exit(translator: &Translator) -> ! {
    println!("{}", translator.translate("cmd.short"));
    println!("{}", translator.translate("cmd.long"));
    process::exit(0);
}

/// 构建分析配置
/// 
/// # Arguments
/// * `cli` - 命令行参数
/// 
/// # Returns
/// * `Result<AnalysisConfig>` - 分析配置
fn build_analysis_config(cli: &Cli) -> Result<AnalysisConfig> {
    let mut config = AnalysisConfig::default();
    
    // 添加排除模式
    config.exclude_patterns = get_exclude_patterns(cli);
    
    // 应用其他配置
    apply_cli_options(&mut config, cli);
    
    Ok(config)
}

/// 获取排除模式列表
/// 
/// # Arguments
/// * `cli` - 命令行参数
/// 
/// # Returns
/// * `Vec<String>` - 排除模式列表
fn get_exclude_patterns(cli: &Cli) -> Vec<String> {
    let mut patterns = get_default_excludes();
    
    // 添加用户指定的排除模式
    match &cli.command {
        Some(Commands::Analyze { exclude, skipindex, .. }) => {
            patterns.extend(exclude.clone());
            if *skipindex {
                add_index_excludes(&mut patterns);
            }
        }
        None => {
            patterns.extend(cli.exclude.clone());
            if cli.skipindex {
                add_index_excludes(&mut patterns);
            }
        }
    }
    
    patterns
}

/// 添加index文件排除模式
/// 
/// # Arguments
/// * `patterns` - 模式列表
fn add_index_excludes(patterns: &mut Vec<String>) {
    patterns.extend(vec![
        "**/index.js".to_string(),
        "**/index.ts".to_string(),
        "**/index.jsx".to_string(),
        "**/index.tsx".to_string(),
    ]);
}

/// 应用命令行选项到配置
/// 
/// # Arguments
/// * `_config` - 分析配置
/// * `_cli` - 命令行参数
fn apply_cli_options(_config: &mut AnalysisConfig, _cli: &Cli) {
    // 预留用于将来的配置扩展
}

/// 构建输出配置
/// 
/// # Arguments
/// * `cli` - 命令行参数
/// 
/// # Returns
/// * `OutputConfig` - 输出配置
fn build_output_config(cli: &Cli) -> OutputConfig {
    match &cli.command {
        Some(Commands::Analyze { verbose, top, issues, summary, markdown, .. }) => {
            OutputConfig {
                verbose: *verbose,
                top_files: *top,
                max_issues: *issues,
                summary_only: *summary,
                markdown_output: *markdown,
            }
        }
        None => {
            OutputConfig {
                verbose: cli.verbose,
                top_files: cli.top,
                max_issues: cli.issues,
                summary_only: cli.summary,
                markdown_output: cli.markdown,
            }
        }
    }
}

/// 执行代码分析
/// 
/// # Arguments
/// * `path` - 分析路径
/// * `config` - 分析配置
/// * `output_config` - 输出配置
/// * `translator` - 翻译器
/// 
/// # Returns
/// * `Result<()>` - 执行结果
fn execute_analysis(
    path: PathBuf,
    config: AnalysisConfig,
    output_config: OutputConfig,
    translator: Translator,
) -> Result<()> {
    // 显示开始信息
    if !output_config.markdown_output {
        print_analysis_start(&path, &config, &translator);
    }
    
    // 创建分析器
    let mut analyzer = CodeAnalyzer::new();
    analyzer.set_language(translator.get_language());
    analyzer.set_silent(output_config.markdown_output);
    
    // 执行分析
    let result = analyzer.analyze_with_config(&path, &config)?;
    
    // 生成报告
    generate_report(result, output_config, translator)?;
    
    Ok(())
}

/// 打印分析开始信息
/// 
/// # Arguments
/// * `path` - 分析路径
/// * `config` - 分析配置
/// * `translator` - 翻译器
fn print_analysis_start(path: &PathBuf, config: &AnalysisConfig, translator: &Translator) {
    println!(
        "🔍 {}",
        translator.translate_with_args(
            "cmd.start_analyzing",
            vec![path.display().to_string()]
        )
    );
    
    // 显示排除模式
    if !config.exclude_patterns.is_empty() {
        print_exclude_patterns(&config.exclude_patterns, translator);
    }
}

/// 打印排除模式列表
/// 
/// # Arguments
/// * `patterns` - 排除模式列表
/// * `translator` - 翻译器
fn print_exclude_patterns(patterns: &[String], translator: &Translator) {
    println!("📂 {}", translator.translate("cmd.exclude_patterns"));
    
    // 显示前5个模式
    let display_count = patterns.len().min(5);
    for pattern in &patterns[..display_count] {
        println!("  - {}", pattern);
    }
    
    // 如果有更多模式，显示数量
    if patterns.len() > 5 {
        println!("  ... 还有 {} 个排除模式", patterns.len() - 5);
    }
    
    println!();
}

/// 生成分析报告
/// 
/// # Arguments
/// * `result` - 分析结果
/// * `output_config` - 输出配置
/// * `translator` - 翻译器
/// 
/// # Returns
/// * `Result<()>` - 执行结果
fn generate_report(
    result: analyzer::AnalysisResult,
    output_config: OutputConfig,
    translator: Translator,
) -> Result<()> {
    // 创建报告生成器
    let mut report = Report::new(result);
    report.set_translator(translator);
    
    // 转换配置
    let options = ReportOptions {
        verbose: output_config.verbose,
        top_files: output_config.top_files,
        max_issues: output_config.max_issues,
        summary_only: output_config.summary_only,
        markdown_output: output_config.markdown_output,
    };
    
    // 生成报告
    report.generate_console_report(&options);
    
    Ok(())
}

/// 获取默认排除模式列表
/// 
/// # Returns
/// * `Vec<String>` - 默认排除模式列表
fn get_default_excludes() -> Vec<String> {
    vec![
        // 前端项目通用排除
        "**/node_modules/**".to_string(),
        "**/dist/**".to_string(),
        "**/build/**".to_string(),
        "**/.next/**".to_string(),
        "**/public/assets/**".to_string(),
        "**/out/**".to_string(),
        "**/.cache/**".to_string(),
        "**/.nuxt/**".to_string(),
        "**/.output/**".to_string(),
        "**/coverage/**".to_string(),
        "**/.vscode/**".to_string(),
        "**/.idea/**".to_string(),
        "**/.git/**".to_string(),
        "**/bower_components/**".to_string(),
        "**/*.min.js".to_string(),
        "**/*.bundle.js".to_string(),
        "**/*.chunk.js".to_string(),
        "**/static/js/*.js".to_string(),
        "**/static/css/*.css".to_string(),
        
        // 后端项目通用排除
        "**/vendor/**".to_string(),
        "**/bin/**".to_string(),
        "**/obj/**".to_string(),
        "**/target/**".to_string(),
        "**/__pycache__/**".to_string(),
        "**/*.pyc".to_string(),
        "**/venv/**".to_string(),
        "**/.env/**".to_string(),
        "**/migrations/**".to_string(),
        "**/generated/**".to_string(),
        "**/logs/**".to_string(),
        "**/tmp/**".to_string(),
        "**/temp/**".to_string(),
        "**/test-results/**".to_string(),
        "**/testdata/**".to_string(),
        
        // Rust特定排除
        "**/target/**".to_string(),
        "**/Cargo.lock".to_string(),
        
        // Tauri项目排除
        "**/src-tauri/target/**".to_string(),
        
        // PHP特定排除
        "**/vendor/**".to_string(),
        "**/composer.lock".to_string(),
        "**/.phpunit.cache/**".to_string(),
        "**/storage/logs/**".to_string(),
        "**/storage/cache/**".to_string(),
        "**/bootstrap/cache/**".to_string(),
        
        // 前端构建文件
        "**/*.min.css".to_string(),
        "**/*.min.js".to_string(),
        "**/*.bundle.css".to_string(),
        "**/*.chunk.css".to_string(),
        
        // 测试文件排除
        "**/*_test.go".to_string(),
        "**/test_*.py".to_string(),
        "**/*_test.py".to_string(),
        "**/tests/**/*.py".to_string(),
        "**/*.spec.js".to_string(),
        "**/*.test.js".to_string(),
        "**/__tests__/**".to_string(),
        "**/*.spec.ts".to_string(),
        "**/*.test.ts".to_string(),
        "**/src/test/**/*.java".to_string(),
        "**/*Test.java".to_string(),
        "**/*_test.c".to_string(),
        "**/*_test.cpp".to_string(),
        "**/tests/**".to_string(),
        "**/*_test.rs".to_string(),
        "**/test_*.rs".to_string(),
        "**/*Test.php".to_string(),
        "**/*_test.php".to_string(),
        "**/tests/**/*.php".to_string(),
    ]
}