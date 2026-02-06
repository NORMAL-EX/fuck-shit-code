//! # 代码度量模块
//!
//! 提供各种代码质量度量指标的计算

mod base;
mod comment_ratio;
mod complexity;
mod duplication;
mod error_handling;
mod function_length;
mod naming;
mod structure;

use crate::i18n::Translator;

pub use base::{Metric, MetricResult};
pub use comment_ratio::CommentRatioMetric;
pub use complexity::CyclomaticComplexityMetric;
pub use duplication::CodeDuplicationMetric;
pub use error_handling::ErrorHandlingMetric;
pub use function_length::FunctionLengthMetric;
pub use naming::NamingConventionMetric;
pub use structure::StructureAnalysisMetric;

/// 度量工厂
///
/// 负责创建各种度量指标实例
pub struct MetricFactory {
    /// 翻译器
    translator: Translator,
}

impl MetricFactory {
    /// 创建新的度量工厂
    ///
    /// # Arguments
    /// * `translator` - 翻译器
    ///
    /// # Returns
    /// * `Self` - 工厂实例
    pub fn new(translator: Translator) -> Self {
        MetricFactory { translator }
    }

    /// 创建所有度量指标
    ///
    /// # Returns
    /// * `Vec<Box<dyn Metric>>` - 度量指标列表
    pub fn create_all_metrics(&self) -> Vec<Box<dyn Metric>> {
        vec![
            Box::new(CyclomaticComplexityMetric::new(self.translator.clone())),
            Box::new(FunctionLengthMetric::new(self.translator.clone())),
            Box::new(CommentRatioMetric::new(self.translator.clone())),
            Box::new(ErrorHandlingMetric::new(self.translator.clone())),
            Box::new(NamingConventionMetric::new(self.translator.clone())),
            Box::new(CodeDuplicationMetric::new(self.translator.clone())),
            Box::new(StructureAnalysisMetric::new(self.translator.clone())),
        ]
    }

    /// 创建核心度量指标
    ///
    /// # Returns
    /// * `Vec<Box<dyn Metric>>` - 核心指标列表
    pub fn create_core_metrics(&self) -> Vec<Box<dyn Metric>> {
        vec![
            Box::new(CyclomaticComplexityMetric::new(self.translator.clone())),
            Box::new(FunctionLengthMetric::new(self.translator.clone())),
            Box::new(CommentRatioMetric::new(self.translator.clone())),
        ]
    }
}
