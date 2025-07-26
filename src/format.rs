use owo_colors::OwoColorize;
use std::env;

pub struct LineFormatter {
    max_width: usize,
    context_chars: usize,
}

impl LineFormatter {
    pub fn new() -> Self {
        let max_width = Self::get_terminal_width().unwrap_or(80);
        LineFormatter {
            max_width,
            context_chars: 20, // 匹配内容前后保留的字符数
        }
    }

    fn get_terminal_width() -> Option<usize> {
        // 尝试从环境变量获取终端宽度
        if let Ok(cols) = env::var("COLUMNS") {
            if let Ok(width) = cols.parse::<usize>() {
                return Some(width);
            }
        }
        
        // Windows 系统使用默认宽度
        Some(80)
    }

    pub fn format_long_line(
        &self,
        line_content: &str,
        query: &str,
        match_indices: &[usize],
    ) -> String {
        // 使用字符边界安全的处理
        let line_chars: Vec<char> = line_content.chars().collect();
        let line_len = line_chars.len();
        
        if line_len <= self.max_width {
            return self.highlight_matches_safe(line_content, query, match_indices);
        }

        // 找到所有匹配的位置（字符索引）
        let mut all_matches = Vec::new();
        for &start_idx in match_indices {
            let end_idx = start_idx + query.len();
            all_matches.push((start_idx, end_idx));
        }

        if all_matches.is_empty() {
            // 如果没有匹配，显示开头部分
            let end_char = (self.max_width - 3).min(line_len);
            let truncated: String = line_chars[..end_char].iter().collect();
            return format!("{}...", truncated);
        }

        // 计算需要显示的区域（字符索引）
        let first_match = all_matches[0].0;
        let last_match = all_matches.last().unwrap().1;
        
        let start_pos = first_match.saturating_sub(self.context_chars);
        let end_pos = (last_match + self.context_chars).min(line_len);
        
        // 确保总长度不超过最大宽度
        let available_width = self.max_width - 6; // 为"..."和空格预留空间
        let mut display_start = start_pos;
        let mut display_end = end_pos;
        
        if end_pos > start_pos && end_pos - start_pos > available_width {
            // 如果内容仍然太长，以第一个匹配为中心
            let center = first_match;
            let half_width = available_width / 2;
            display_start = center.saturating_sub(half_width);
            display_end = (center + half_width).min(line_len);
        }
        
        // 确保 display_start <= display_end 且都在有效范围内
        display_start = display_start.min(line_len);
        display_end = display_end.max(display_start).min(line_len);

        let mut result = String::new();
        
        // 添加前缀省略号
        if display_start > 0 {
            result.push_str("...");
        }
        
        // 截取并高亮显示的内容
        let segment_chars = &line_chars[display_start..display_end];
        let segment: String = segment_chars.iter().collect();
        
        let adjusted_indices: Vec<usize> = match_indices
            .iter()
            .filter(|&&idx| idx >= display_start && idx < display_end)
            .map(|&idx| idx - display_start)
            .collect();
        
        let highlighted = self.highlight_matches_safe(&segment, query, &adjusted_indices);
        result.push_str(&highlighted);
        
        // 添加后缀省略号
        if display_end < line_len {
            result.push_str("...");
        }
        
        result
    }

    pub fn format_long_line_pure(
        &self,
        line_content: &str,
        query: &str,
        match_indices: &[usize],
    ) -> String {
        // 使用字符边界安全的处理
        let line_chars: Vec<char> = line_content.chars().collect();
        let line_len = line_chars.len();
        
        if line_len <= self.max_width {
            return self.highlight_matches_safe_pure(line_content, query, match_indices);
        }

        // 找到所有匹配的位置（字符索引）
        let mut all_matches = Vec::new();
        for &start_idx in match_indices {
            let end_idx = start_idx + query.len();
            all_matches.push((start_idx, end_idx));
        }

        if all_matches.is_empty() {
            // 如果没有匹配，显示开头部分
            let end_char = (self.max_width - 3).min(line_len);
            let truncated: String = line_chars[..end_char].iter().collect();
            return format!("{}...", truncated);
        }

        // 计算需要显示的区域（字符索引）
        let first_match = all_matches[0].0;
        let last_match = all_matches.last().unwrap().1;
        
        let start_pos = first_match.saturating_sub(self.context_chars);
        let end_pos = (last_match + self.context_chars).min(line_len);
        
        // 确保总长度不超过最大宽度
        let available_width = self.max_width - 6; // 为"..."和空格预留空间
        let mut display_start = start_pos;
        let mut display_end = end_pos;
        
        if end_pos > start_pos && end_pos - start_pos > available_width {
            // 如果内容仍然太长，以第一个匹配为中心
            let center = first_match;
            let half_width = available_width / 2;
            display_start = center.saturating_sub(half_width);
            display_end = (center + half_width).min(line_len);
        }
        
        // 确保 display_start <= display_end 且都在有效范围内
        display_start = display_start.min(line_len);
        display_end = display_end.max(display_start).min(line_len);

        let mut result = String::new();
        
        // 添加前缀省略号
        if display_start > 0 {
            result.push_str("...");
        }
        
        // 截取并高亮显示的内容
        let segment_chars = &line_chars[display_start..display_end];
        let segment: String = segment_chars.iter().collect();
        
        let adjusted_indices: Vec<usize> = match_indices
            .iter()
            .filter(|&&idx| idx >= display_start && idx < display_end)
            .map(|&idx| idx - display_start)
            .collect();
        
        let highlighted = self.highlight_matches_safe_pure(&segment, query, &adjusted_indices);
        result.push_str(&highlighted);
        
        // 添加后缀省略号
        if display_end < line_len {
            result.push_str("...");
        }
        
        result
    }

    fn highlight_matches_safe(
        &self,
        content: &str,
        query: &str,
        match_indices: &[usize],
    ) -> String {
        if match_indices.is_empty() {
            return content.to_string();
        }

        let mut result = String::new();
        let mut last_end = 0;

        for &start_idx in match_indices {
            let end_idx = start_idx + query.len();
            
            // 使用字符边界安全的切片
            let content_chars: Vec<char> = content.chars().collect();
            if start_idx >= content_chars.len() {
                continue;
            }
            
            let actual_end = end_idx.min(content_chars.len());
            
            // 添加匹配前的文本
            if start_idx > last_end {
                let before: String = content_chars[last_end..start_idx].iter().collect();
                result.push_str(&before);
            }
            
            // 添加高亮的匹配文本
            let matched: String = content_chars[start_idx..actual_end].iter().collect();
            result.push_str(&matched.on_red().to_string());
            
            last_end = actual_end;
        }

        // 添加剩余的文本
        let content_chars: Vec<char> = content.chars().collect();
        if last_end < content_chars.len() {
            let remaining: String = content_chars[last_end..].iter().collect();
            result.push_str(&remaining);
        }

        result
    }

    fn highlight_matches_safe_pure(
        &self,
        content: &str,
        query: &str,
        match_indices: &[usize],
    ) -> String {
        if match_indices.is_empty() {
            return content.to_string();
        }

        let mut result = String::new();
        let mut last_end = 0;

        for &start_idx in match_indices {
            let end_idx = start_idx + query.len();
            
            // 使用字符边界安全的切片
            let content_chars: Vec<char> = content.chars().collect();
            if start_idx >= content_chars.len() {
                continue;
            }
            
            let actual_end = end_idx.min(content_chars.len());
            
            // 添加匹配前的文本
            if start_idx > last_end {
                let before: String = content_chars[last_end..start_idx].iter().collect();
                result.push_str(&before);
            }
            
            // 添加高亮的匹配文本
            let matched: String = content_chars[start_idx..actual_end].iter().collect();
            result.push_str(&matched.to_string());
            
            last_end = actual_end;
        }

        // 添加剩余的文本
        let content_chars: Vec<char> = content.chars().collect();
        if last_end < content_chars.len() {
            let remaining: String = content_chars[last_end..].iter().collect();
            result.push_str(&remaining);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_line_formatting() {
        let formatter = LineFormatter::new();
        let line = "This is a short line";
        let result = formatter.format_long_line(line, "short", &[10]);
        assert!(result.contains("short"));
    }

    #[test]
    fn test_long_line_formatting() {
        let formatter = LineFormatter {
            max_width: 50,
            context_chars: 5,
        };
        
        let long_line = "This is a very long line with multiple words and the search term appears somewhere in the middle of this long content";
        let result = formatter.format_long_line(long_line, "search", &[60]);
        
        assert!(result.contains("..."));
        assert!(result.contains("search"));
        assert!(result.len() <= 50);
    }
}
