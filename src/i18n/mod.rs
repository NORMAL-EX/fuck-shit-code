//! # 国际化模块
//!
//! 提供多语言支持功能

mod en_us;
mod zh_cn;

use std::collections::HashMap;

/// 语言类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// 中文
    ZhCN,

    /// 英文
    EnUS,
}

/// 翻译器
#[derive(Clone)]
pub struct Translator {
    /// 当前语言
    language: Language,

    /// 消息映射
    messages: &'static HashMap<String, String>,
}

impl Translator {
    /// 创建新的翻译器
    ///
    /// # Arguments
    /// * `language` - 语言设置
    ///
    /// # Returns
    /// * `Self` - 翻译器实例
    pub fn new(language: Language) -> Self {
        let messages = match language {
            Language::ZhCN => &*zh_cn::MESSAGES,
            Language::EnUS => &*en_us::MESSAGES,
        };

        Translator { language, messages }
    }

    /// 翻译文本
    ///
    /// # Arguments
    /// * `key` - 消息键
    ///
    /// # Returns
    /// * `String` - 翻译后的文本
    pub fn translate(&self, key: &str) -> String {
        self.messages
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// 带参数的翻译
    ///
    /// # Arguments
    /// * `key` - 消息键
    /// * `args` - 参数列表
    ///
    /// # Returns
    /// * `String` - 翻译后的文本
    pub fn translate_with_args(&self, key: &str, args: Vec<String>) -> String {
        let template = self.translate(key);
        let mut result = template;

        // 替换占位符
        for (i, arg) in args.iter().enumerate() {
            // 支持 {} 格式
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);

            // 支持 %s 格式
            if result.contains("%s") {
                result = result.replacen("%s", arg, 1);
            }
        }

        result
    }

    /// 获取当前语言
    ///
    /// # Returns
    /// * `Language` - 当前语言
    pub fn get_language(&self) -> Language {
        self.language
    }

    /// 切换语言
    ///
    /// # Arguments
    /// * `language` - 新语言
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
        self.messages = match language {
            Language::ZhCN => &*zh_cn::MESSAGES,
            Language::EnUS => &*en_us::MESSAGES,
        };
    }
}
