use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::display::DisPlay;
use crate::file::File;

pub struct ParallelProcessor {
    query: Arc<String>,
    progress_bar: Option<ProgressBar>,
}

impl ParallelProcessor {
    pub fn new(query: String) -> Self {
        Self {
            query: Arc::new(query),
            progress_bar: None,
        }
    }

    pub fn process_directory(&mut self, dir_path: PathBuf, ignore_case: bool) -> Result<DashMap<String, Vec<DisPlay>>> {
        // 收集所有文件路径
        let files: Vec<PathBuf> = WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .collect();

        if files.is_empty() {
            return Ok(DashMap::new());
        }

        // 创建进度条
        let progress_bar = ProgressBar::new(files.len() as u64);
        progress_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        self.progress_bar = Some(progress_bar.clone());

        // 并行处理文件
        let results: DashMap<String, Vec<DisPlay>> = DashMap::new();
        
        files.par_iter().for_each(|file_path| {
            if let Ok(content) = fs::read_to_string(file_path) {
                let file_name = file_path.to_string_lossy().to_string();
                let mut file = File::new(file_name.clone(), crate::file::Format::Text, content);
                
                if let Ok(display_list) = self.process_single_file(&mut file, ignore_case) {
                    if !display_list.is_empty() {
                        results.insert(file_name, display_list);
                    }
                }
            }
            
            progress_bar.inc(1);
        });

        progress_bar.finish_with_message("并行搜索完成");
        Ok(results)
    }

    fn process_single_file(&self, file: &mut File, ignore_case: bool) -> Result<Vec<DisPlay>> {
        let mut display_list = Vec::new();

        let query = &self.query;
        let file_name = file.name.clone();
        let query_str = query.as_str();
        
        let mut line_index = 0;
        while let Some(line) = file.next_line() {
            let ori_query = query_str.to_string();
            let ori_line = line.clone();
            // 如果忽略大小写，则转换为小写
            let line = if ignore_case {
                line.to_lowercase()
            } else {
                line.clone()
            };

            let query_str = if ignore_case {
                query_str.to_lowercase()
            } else {
                query_str.to_string()
            };

            if !line.contains(&query_str) {
                line_index += 1;
                continue;
            }

            let mut match_indices = Vec::new();
            let mut matched_str = "";

            for (start, matched) in line.match_indices(&query_str) {
                match_indices.push(start);
                matched_str = matched;
            }

            if !match_indices.is_empty() {
                let display = DisPlay::new(
                    ori_query,
                    file_name.clone(),
                    line_index,
                    match_indices,
                    ori_line,
                    matched_str.to_string(),
                );
                display_list.push(display);
            }
            line_index += 1;
        }

        Ok(display_list)
    }
}
