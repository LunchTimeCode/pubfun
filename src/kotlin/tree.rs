use tree_sitter::{Node, Parser};

pub fn parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_kotlin::language())
        .expect("Error loading Rust grammar");
    parser
}
#[derive(Debug)]
pub struct KTNode {
    content: String,
    kdoc: Option<String>,
}

impl KTNode {
    pub fn new(function: String, kdoc: Option<String>) -> Self {
        Self {
            content: function,
            kdoc,
        }
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn kdoc(&self) -> Option<String> {
        self.kdoc.clone()
    }
}

pub fn pub_funs(parser: &mut Parser, content: &str) -> Vec<KTNode> {
    let mut functions = Vec::new();

    let source_code = content.as_bytes();

    let tree = parser.parse(content, None);
    if let Some(tree) = tree {
        let root_node = tree.root_node();

        let public_functions = find_public_functions(root_node, content);
        for func in public_functions {
            let (f, kdoc) = func;
            let f_as_str = f.utf8_text(source_code).unwrap().to_string();

            let kdoc_as_str = kdoc.map(|k| k.utf8_text(source_code).unwrap().to_string());
            functions.push(KTNode::new(f_as_str, kdoc_as_str));
        }
    }
    functions
}

fn find_public_functions<'a>(
    node: Node<'a>,
    content: &'a str,
) -> Vec<(Node<'a>, Option<Node<'a>>)> {
    let source_code = content.as_bytes();
    let mut functions = Vec::new();

    let node_kind = node.kind();

    if node_kind == "function_declaration" {
        let mut is_public = false;
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == "modifiers" {
                for j in 0..child.named_child_count() {
                    let modifier = child.named_child(j).unwrap();
                    let pure = modifier.utf8_text(source_code).unwrap();
                    let kind = modifier.kind();
                    if kind == "visibility_modifier" && pure == "public" {
                        is_public = true;
                        break;
                    }
                }
            }
        }
        if is_public {
            let kdoc_comment = find_kdoc_comment(node, content);
            functions.push((node, kdoc_comment));
        }
    }

    if node_kind == "class_declaration" {
        let mut is_public = false;
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            if child.kind() == "modifiers" {
                for j in 0..child.named_child_count() {
                    let modifier = child.named_child(j).unwrap();
                    let pure = modifier.utf8_text(source_code).unwrap();
                    let kind = modifier.kind();
                    if kind == "visibility_modifier" && pure == "public" {
                        is_public = true;
                        break;
                    }
                }
            }
        }
        if is_public {
            let kdoc_comment = find_kdoc_comment(node, content);
            functions.push((node, kdoc_comment));
        }
    }

    for i in 0..node.child_count() {
        functions.extend(find_public_functions(node.child(i).unwrap(), content));
    }

    functions
}

fn find_kdoc_comment<'a>(node: Node<'a>, content: &'a str) -> Option<Node<'a>> {
    let source_code = content.as_bytes();
    let mut current_node = node;

    while let Some(prev_sibling) = current_node.prev_sibling() {
        if prev_sibling.kind() == "multiline_comment"
            && prev_sibling
                .utf8_text(source_code)
                .unwrap()
                .starts_with("/**")
        {
            return Some(prev_sibling);
        }
        current_node = prev_sibling;
    }
    None
}
