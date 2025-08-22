//! # 文件操作模块
//! 
//! 提供文件搜索、过滤等功能

use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use anyhow::Result;

use super::LanguageDetector;

/// 文件查找器
/// 
/// 负责在指定目录中查找符合条件的源文件
pub struct FileFinder {
    /// 根目录
    root_dir: PathBuf,
    
    /// 包含模式集
    include_patterns: GlobSet,
    
    /// 排除模式集
    exclude_patterns: GlobSet,
    
    /// 语言检测器
    detector: LanguageDetector,
}

impl FileFinder {
    /// 创建新的文件查找器
    /// 
    /// # Arguments
    /// * `root_dir` - 根目录
    /// * `include_patterns` - 包含模式
    /// * `exclude_patterns` - 排除模式
    /// 
    /// # Returns
    /// * `Result<Self>` - 查找器实例
    pub fn new(
        root_dir: &Path,
        include_patterns: &[String],
        exclude_patterns: &[String],
    ) -> Result<Self> {
        let include_set = Self::build_glob_set(include_patterns)?;
        let exclude_set = Self::build_glob_set(exclude_patterns)?;
        
        Ok(FileFinder {
            root_dir: root_dir.to_path_buf(),
            include_patterns: include_set,
            exclude_patterns: exclude_set,
            detector: LanguageDetector::new(),
        })
    }
    
    /// 构建glob模式集
    /// 
    /// # Arguments
    /// * `patterns` - 模式列表
    /// 
    /// # Returns
    /// * `Result<GlobSet>` - 模式集
    fn build_glob_set(patterns: &[String]) -> Result<GlobSet> {
        let mut builder = GlobSetBuilder::new();
        
        for pattern in patterns {
            let glob = Glob::new(pattern)?;
            builder.add(glob);
        }
        
        Ok(builder.build()?)
    }
    
    /// 查找源文件
    /// 
    /// # Arguments
    /// * `progress_callback` - 进度回调
    /// 
    /// # Returns
    /// * `Vec<PathBuf>` - 找到的文件列表
    pub fn find_source_files<F>(&self, progress_callback: F) -> Vec<PathBuf>
    where
        F: Fn(usize),
    {
        let mut files = Vec::new();
        let mut visited_dirs = HashSet::new();
        let mut file_count = 0;
        
        // 创建目录遍历器
        let walker = WalkDir::new(&self.root_dir)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| self.should_visit_dir(e, &mut visited_dirs));
        
        // 遍历文件
        for entry in walker {
            if let Ok(entry) = entry {
                if self.is_valid_source_file(&entry) {
                    files.push(entry.path().to_path_buf());
                    file_count += 1;
                    progress_callback(file_count);
                }
            }
        }
        
        files
    }
    
    /// 判断是否应该访问目录
    /// 
    /// # Arguments
    /// * `entry` - 目录项
    /// * `visited` - 已访问集合
    /// 
    /// # Returns
    /// * `bool` - 是否访问
    fn should_visit_dir(&self, entry: &DirEntry, visited: &mut HashSet<PathBuf>) -> bool {
        let path = entry.path();
        
        // 避免重复访问
        if !visited.insert(path.to_path_buf()) {
            return false;
        }
        
        // 跳过隐藏目录
        if self.is_hidden_dir(path) {
            return false;
        }
        
        // 检查排除模式
        !self.is_excluded(path)
    }
    
    /// 判断是否为隐藏目录
    /// 
    /// # Arguments
    /// * `path` - 路径
    /// 
    /// # Returns
    /// * `bool` - 是否隐藏
    fn is_hidden_dir(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with('.') && name != ".")
            .unwrap_or(false)
    }
    
    /// 判断是否被排除
    /// 
    /// # Arguments
    /// * `path` - 路径
    /// 
    /// # Returns
    /// * `bool` - 是否排除
    fn is_excluded(&self, path: &Path) -> bool {
        if let Ok(rel_path) = path.strip_prefix(&self.root_dir) {
            return self.exclude_patterns.is_match(rel_path);
        }
        false
    }
    
    /// 判断是否为有效的源文件
    /// 
    /// # Arguments
    /// * `entry` - 目录项
    /// 
    /// # Returns
    /// * `bool` - 是否有效
    fn is_valid_source_file(&self, entry: &DirEntry) -> bool {
        // 必须是文件
        if !entry.file_type().is_file() {
            return false;
        }
        
        let path = entry.path();
        
        // 必须是支持的文件类型
        if !self.detector.is_supported_file(path) {
            return false;
        }
        
        // 检查是否应该包含
        self.should_include_file(path)
    }
    
    /// 判断是否应该包含文件
    /// 
    /// # Arguments
    /// * `path` - 文件路径
    /// 
    /// # Returns
    /// * `bool` - 是否包含
    fn should_include_file(&self, path: &Path) -> bool {
        if let Ok(rel_path) = path.strip_prefix(&self.root_dir) {
            // 检查排除模式
            if self.exclude_patterns.is_match(rel_path) {
                return false;
            }
            
            // 如果没有包含模式，默认包含
            if self.include_patterns.is_empty() {
                return true;
            }
            
            // 检查包含模式
            return self.include_patterns.is_match(rel_path);
        }
        
        false
    }
}

/// 查找源文件（便捷函数）
/// 
/// # Arguments
/// * `root_dir` - 根目录
/// * `include_patterns` - 包含模式
/// * `exclude_patterns` - 排除模式
/// * `progress_callback` - 进度回调
/// 
/// # Returns
/// * `Result<Vec<PathBuf>>` - 文件列表
pub fn find_source_files<F>(
    root_dir: &Path,
    include_patterns: &[String],
    exclude_patterns: &[String],
    progress_callback: F,
) -> Result<Vec<PathBuf>>
where
    F: Fn(usize),
{
    let finder = FileFinder::new(root_dir, include_patterns, exclude_patterns)?;
    Ok(finder.find_source_files(progress_callback))
}