
mod files;
mod inspector;

fn main() {
    let paths = files::find("/dir/dir/proj/dir/repo/**/*.kt");
    println!("Files found: {}", paths.len());
    
    let functions = inspector::find_functions(paths);
    
    println!("{:?}", functions)
    
    
    
}
