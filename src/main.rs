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
        println!("Output saved to: {:?}", output_file);
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
                let tip = format!("Found the following matches in file '{}': \n", file_name.yellow());
                output.push_str(&tip);
            } else {
                let tip = format!("Found the following matches in file '{}': \n", file_name);
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
            eprintln!("Warning: '{}' does not exist", file_path);
            continue;
        }
        
        if !path.is_file() {
            eprintln!("Warning: '{}' is not a valid file", file_path);
            continue;
        }
        
        let canonical_path = match path.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Warning: Cannot canonicalize path '{}': {}", file_path, e);
                continue;
            }
        };
        
        if unique_paths.insert(canonical_path.clone()) {
            valid_paths.push(canonical_path);
        }
    }
    
    if valid_paths.is_empty() {
        return Err(anyhow!("No valid files found"));
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

        // Convert to lowercase if ignoring case
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
        return Err(anyhow!("Only one directory path can be specified"));
    }
    let dir_path = dir_paths.first().ok_or_else(|| anyhow!("Must provide a directory path"))?;
    let valid_file_path = find_valid_dirs(dir_path.clone())?;
    
    // Use parallel processing
    let mut processor = ParallelProcessor::new(query);
    let results = processor.process_directory(valid_file_path, ignore_case)?;
    
    // Convert DashMap to HashMap for output
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
        println!("Output saved to: {:?}", output_file);
    }
    Ok(())
}

fn find_valid_dirs(dir_path: String) -> Result<PathBuf> {
    let path = PathBuf::from(&dir_path);
    if !path.exists() {
        return Err(anyhow!("Directory '{}' does not exist", dir_path));
    }
    if !path.is_dir() {
        return Err(anyhow!("Path '{}' is not a valid directory", dir_path));
    }
    let canonical_path = match path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            return Err(anyhow!("Warning: Cannot canonicalize path '{}': {}", dir_path, e))
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
                    return Err(anyhow!("Can only specify one of file_path or dir, not both"));
                }
                (true, true) => {
                    return Err(anyhow!("Must specify either file_path or dir"));
                }
                (false, true) => {
                    println!("Searching in files {:?}", file_path);
                    if let Some(out_dir) = output.clone() {
                        if !PathBuf::from(&out_dir).exists() {
                            return Err(anyhow!("Output directory does not exist"));
                        }
                    }
                    handle_file_path_vec(query, file_path, ignore_case, output)?;
                }
                (true, false) => {
                    println!("Searching in directory {:?}", dir);
                    if let Some(out_dir) = output.clone() {
                        if !PathBuf::from(&out_dir).exists() {
                            return Err(anyhow!("Output directory does not exist"));
                        }
                    }
                    handle_dir_vec(query, dir, ignore_case, output)?;
                }
            }
        }
        Some(Commands::Diff { left_file, right_file }) => {
            println!("Comparing files {} and {}", left_file, right_file);
        }
        None => {
            return Err(anyhow!("Please specify a subcommand: find or diff"));
        }
    }

    Ok(())
}
