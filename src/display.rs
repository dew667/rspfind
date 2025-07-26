use owo_colors::OwoColorize;
use crate::format::LineFormatter;

#[derive(Clone)]
pub struct DisPlay {
    query: String,
    file_name: String,
    line_index: usize,
    start_indexs: Vec<usize>,
    line_content: String,
    str: String,
}

impl DisPlay {
    pub fn new(query: String, file_name: String, line_index: usize, start_indexs: Vec<usize>, line_content: String, str: String) -> Self {
        DisPlay {
            query,
            file_name,
            line_index,
            start_indexs,
            line_content,
            str
        }
    }
    
    pub fn display(&self) -> String {
        let formatter = LineFormatter::new();
        
        let formatted_content = formatter.format_long_line(
            &self.line_content,
            &self.query,
            &self.start_indexs
        );
        
        let mut positions: String = String::new();
        for start_index in &self.start_indexs {
            positions.push_str(&format!("{}-{}, ", 
                (start_index + 1).green(), 
                (start_index + self.query.len()).green()
            ));
        }
        
        // 移除最后一个逗号和空格
        if positions.ends_with(", ") {
            positions.truncate(positions.len() - 2);
        }
        
        format!(
            "line number: {} position: [{}] line content: {}\n",
            (self.line_index + 1).green(),
            positions.blue(),
            formatted_content.yellow()
        )
    }

    pub fn pure_display(&self) -> String {
        let formatter = LineFormatter::new();
        
        let formatted_content = formatter.format_long_line_pure(
            &self.line_content,
            &self.query,
            &self.start_indexs
        );
        
        let mut positions: String = String::new();
        for start_index in &self.start_indexs {
            positions.push_str(&format!("{}-{}, ", 
                (start_index + 1), 
                (start_index + self.query.len())
            ));
        }
        
        // 移除最后一个逗号和空格
        if positions.ends_with(", ") {
            positions.truncate(positions.len() - 2);
        }
        
        format!(
            "line number: {} position: [{}] line content: {}\n",
            (self.line_index + 1),
            positions,
            formatted_content
        )
    }
}
