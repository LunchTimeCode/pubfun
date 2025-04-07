use std::env;

mod files;
mod inspector;
mod kotlin;

fn main() {
    let path = env::var("PUB_PATH").expect("PUB_PATH not set");
    let paths = files::find(&path);

    let packages = inspector::packages(paths);

    let as_print = packages
        .iter()
        .map(|p| p.md())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", as_print)
}
