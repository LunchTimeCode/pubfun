use std::path::PathBuf;

use itertools::Itertools;
use tree_sitter::Parser;

use crate::kotlin;

pub struct KtPackage {
    package: Option<String>,
    files: Vec<KtFile>,
}

impl KtPackage {
    pub fn md(&self) -> String {
        let mut files = self
            .files
            .iter()
            .map(|f| f.md())
            .collect::<Vec<String>>()
            .join("\n");
        files = format!(
            "## All public API {}\n\n{}",
            self.package.clone().unwrap_or_default(),
            files
        );
        files
    }
}

pub struct KtFile {
    file_name: String,
    package: Option<String>,
    functions: Vec<KtFlatNode>,
}

impl KtFile {
    pub fn md(&self) -> String {
        let mut functions = self
            .functions
            .iter()
            .map(|f| f.md())
            .collect::<Vec<String>>()
            .join("\n");
        functions = format!("### {}\n\n{}", self.file_name, functions);
        functions
    }
}

pub fn packages(paths: Vec<PathBuf>) -> Vec<KtPackage> {
    let functions = find_functions(paths);

    let grouped_by_file: Vec<KtFile> = functions
        .into_iter()
        .group_by(|f| f.file.clone())
        .into_iter()
        .map(|(file_name, group)| KtFile {
            file_name,
            package: None,
            functions: group.collect(),
        })
        .collect();

    let grouped_by_package: Vec<KtPackage> = grouped_by_file
        .into_iter()
        .group_by(|fi| fi.package.clone())
        .into_iter()
        .map(|(package, group)| KtPackage {
            package,
            files: group.collect(),
        })
        .collect();

    grouped_by_package
}

#[allow(unused)]
#[derive(Debug)]
pub struct KtFlatNode {
    path: String,
    file: String,
    package: Option<String>,
    content: String,
    kdoc: Option<String>,
}

const BLOCK: &str = r#"```"#;

impl KtFlatNode {
    pub fn md(&self) -> String {
        format!(
            "

{BLOCK}kotlin
{}
{}
{BLOCK}

",
            self.kdoc.clone().unwrap_or_default(),
            self.content
        )
    }
}

pub fn find_functions(paths: Vec<PathBuf>) -> Vec<KtFlatNode> {
    let mut p = kotlin::tree::parser();

    paths
        .iter()
        .flat_map(|path| find_function(path.to_path_buf(), &mut p))
        .collect()
}

pub fn find_function(path: PathBuf, parser: &mut Parser) -> Vec<KtFlatNode> {
    let content = std::fs::read_to_string(&path).expect("Failed to read file");

    let file_name = path.file_name().unwrap().to_str().unwrap();
    let path_as_str = path.to_str().unwrap();

    let package = content
        .lines()
        .find(|line| line.starts_with("package"))
        .map(|line| {
            line.replace("package", "")
                .replace(";", "")
                .trim()
                .to_string()
        });

    let funcs = kotlin::tree::pub_funs(parser, content.as_str());

    funcs
        .iter()
        .map(|f| KtFlatNode {
            path: path_as_str.to_string(),
            package: package.clone(),
            file: file_name.to_string(),
            content: f.content(),
            kdoc: f.kdoc(),
        })
        .collect()
}
