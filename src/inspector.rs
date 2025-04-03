use std::{io::Stdin, path::PathBuf};

use regex::Regex;

#[derive(Debug)]
pub struct Function {
    path: String,
    range: (u32, u32),
    content: String
}

pub fn find_functions(paths: Vec<PathBuf>) -> Vec<Function> {
    let re = Regex::new(r"pub\s+fun\s+\w+\s*\([^)]*\)\s*\{(?:[^{}]*|\{[^{}]*\})*\}").unwrap();
    let name_re = Regex::new(r"pub\s+fun\s+(\w+)\s*\(").unwrap();
    
    paths.iter().map(|path| find_function(path.to_path_buf(), &re, &name_re)).flatten().collect()
}

pub fn find_function(path: PathBuf, re: &Regex, name_re: &Regex) -> Vec<Function> {
    let content = std::fs::read_to_string(&path).expect("Failed to read file");
    
    let path_as_str = path.to_str().unwrap();
    
    let mut functions = Vec::new();
    
    let lines: Vec<String> =  content.lines().collect();
    
    content.lines().into_iter().enumerate().for_each(|(i,l)|{
       let is_pub =  l.contains("pub");
       let is_fun =  l.contains("fun");
       
       if is_pub && is_fun {
           
           let index = if i < 10 { 0 } else { i - 10 };
           let end = if i + 10 > content.len() { content.len() } else { i + 10 };
      
           let content: Vec<String> = lines[index..end];
               
           
        let f =  Function {
                 path: path_as_str.to_owned(),
                 range: (i as u32, i as u32 + 10),
                 content: content.to_string()
             };
         functions.push(f);  
       }
        
    });
    functions
}

