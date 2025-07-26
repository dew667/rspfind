use std::fs;
use std::path::{PathBuf};
use std::collections::{HashSet, HashMap};
use std::process::Output;

use clap::{Parser, Subcommand};
use anyhow::{anyhow, Result};

mod file;
use file::File;

mod display;
use display::DisPlay;

mod format;
mod parallel;
use parallel::ParallelProcessor;

use owo_colors::OwoColorize;

#[derive(Parser)]
#[command(name = "rspfind")]
#[command(version = "0.1")]
#[command(about = "A tool to find content in files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Find {
        #[arg(short, long, required = true)]
        query: String,

        #[arg(short, long, num_args=1..)]
        file_path: Vec<String>,

        #[arg(short, long, num_args=1..)]
        dir: Vec<String>,

        #[arg(short, long, default_value = "false")]
        ignore_case: bool,

        #[arg(short, long)]
        output: Option<String>,
    },
    Diff {
        #[arg(short, long)]
        left_file: String,

        #[arg(short, long)]
        right_file: String,
    }
}

fn handle_file_path_vec(query: String, file_paths: Vec<String>, ignore_case: bool, out_dir: Option<String>) -> Result<()> {
    let valid_file_paths = find_valid_paths(file_paths)?;
    let mut display_map: HashMap<String, Vec<DisPlay>> = HashMap::new();
    for file_path in valid_file_paths {
        let content = fs::read_to_string(&file_path);
        let content = match content {
            Ok(c) => c,
            Err(_) => continue,
        };
        let mut file = File::new(file_path.to_string_lossy().to_string(), file::Format::Text, content);
        let display_list = find_content_in_file(&query, &mut file, ignore_case)?;
        display_map.insert(file.name.clone(), display_list);
    }
    let output = get_output(display_map.clone(), false);
    println!("{}", output);
    if let Some(out_dir) = out_dir {
        let out_dir = PathBuf::from(out_dir);
        let canonical_dir = out_dir.canonicalize()?;
        let output_file = canonical_dir.join("output.txt");
        let output = get_output(display_map, true);
        fs::write(&output_file, output)?;
        println!("输出已保存到: {:?}", output_file);
    }
    Ok(())
}

fn get_output(display_map: HashMap<String, Vec<DisPlay>>, pure_text_output: bool) -> String {
    let mut output = String::new();
    display_map.iter().for_each(|(file_path, displays)| {
        let file_name_vec: Vec<&str> = file_path.rsplit('\\').collect();
        let file_name = file_name_vec.first().unwrap_or(&"Unknown file").to_string();
        if !displays.is_empty() {
            if !pure_text_output {
                let tip = format!("在文件 '{}' 中找到以下匹配: \n", file_name.yellow());
                output.push_str(&tip);
            } else {
                let tip = format!("在文件 '{}' 中找到以下匹配: \n", file_name);
                output.push_str(&tip);
            }
            for display in displays {
                let mut out_line: String = String::new();
                if pure_text_output {
                    out_line = String::from(display.pure_display());
                } else {
                    out_line = String::from(display.display());
                };
                output.push_str(out_line.as_str());
            }
        }
    });
    output
}

fn find_valid_paths(file_paths: Vec<String>) -> Result<Vec<PathBuf>> {
    let mut unique_paths = HashSet::new();
    let mut valid_paths = Vec::new();
    
    for file_path in file_paths {
        let path = PathBuf::from(&file_path);
        
        if !path.exists() {
            eprintln!("警告: '{}' 不存在", file_path);
            continue;
        }
        
        if !path.is_file() {
            eprintln!("警告: '{}' 不是有效的文件", file_path);
            continue;
        }
        
        let canonical_path = match path.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("警告: 无法规范化路径 '{}': {}", file_path, e);
                continue;
            }
        };
        
        if unique_paths.insert(canonical_path.clone()) {
            valid_paths.push(canonical_path);
        }
    }
    
    if valid_paths.is_empty() {
        return Err(anyhow!("没有找到有效的文件"));
    }
    
    Ok(valid_paths)
}

fn find_content_in_file(query: &String, file: &mut File, ignore_case: bool) -> Result<Vec<DisPlay>> {
    let mut display_list = Vec::new();
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
        
        let display = DisPlay::new(
            ori_query, 
            file_name.clone(), 
            line_index, 
            match_indices, 
            ori_line,
            matched_str.to_string()
        );
        display_list.push(display);
        line_index += 1;
    }
    
    Ok(display_list)
}

fn handle_dir_vec(query: String, dir_paths: Vec<String>, ignore_case: bool, out_dir: Option<String>) -> Result<()> {
    if dir_paths.len() > 1 {
        return Err(anyhow!("只能指定一个目录路径"));
    }
    let dir_path = dir_paths.first().ok_or_else(|| anyhow!("必须提供一个目录路径"))?;
    let valid_file_path = find_valid_dirs(dir_path.clone())?;
    
    // 使用并行处理
    let mut processor = ParallelProcessor::new(query);
    let results = processor.process_directory(valid_file_path, ignore_case)?;
    
    // 将DashMap转换为HashMap用于输出
    let mut display_map: HashMap<String, Vec<DisPlay>> = HashMap::new();
    for entry in results.iter() {
        display_map.insert(entry.key().clone(), entry.value().to_vec());
    }
    
    let output = get_output(display_map.clone(), false);
    println!("{}", output);
    if let Some(out_dir) = out_dir {
        let out_dir = PathBuf::from(out_dir);
        let canonical_dir = out_dir.canonicalize()?;
        let output_file = canonical_dir.join("output.txt");
        let output = get_output(display_map, true);
        fs::write(&output_file, output)?;
        println!("输出已保存到: {:?}", output_file);
    }
    Ok(())
}

fn find_valid_dirs(dir_path: String) -> Result<PathBuf> {
    let path = PathBuf::from(&dir_path);
    if !path.exists() {
        return Err(anyhow!("目录 '{}' 不存在", dir_path));
    }
    if !path.is_dir() {
        return Err(anyhow!("路径 '{}' 不是一个有效的目录", dir_path));
    }
    let canonical_path = match path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            return Err(anyhow!("警告: 无法规范化路径 '{}': {}", dir_path, e))
        }
    };
        
    Ok(canonical_path)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Find { query, file_path, dir, ignore_case, output }) => {
            match (file_path.is_empty(), dir.is_empty()) {
                (false, false) => {
                    return Err(anyhow!("只能指定 file_path 或 dir 中的一个参数，不能同时指定两者"));
                }
                (true, true) => {
                    return Err(anyhow!("必须指定 file_path 或 dir 中的一个参数"));
                }
                (false, true) => {
                    println!("在文件 {:?} 中查找内容", file_path);
                    if let Some(out_dir) = output.clone() {
                        if !PathBuf::from(&out_dir).exists() {
                            return Err(anyhow!("输出目录不存在"));
                        }
                    }
                    handle_file_path_vec(query, file_path, ignore_case, output)?;
                }
                (true, false) => {
                    println!("在目录 {:?} 中查找内容", dir);
                    if let Some(out_dir) = output.clone() {
                        if !PathBuf::from(&out_dir).exists() {
                            return Err(anyhow!("输出目录不存在"));
                        }
                    }
                    handle_dir_vec(query, dir, ignore_case, output)?;
                }
            }
        }
        Some(Commands::Diff { left_file, right_file }) => {
            println!("比较文件 {} 和 {}", left_file, right_file);
        }
        None => {
            return Err(anyhow!("请指定一个子命令: find 或 diff"));
        }
    }

    Ok(())
}
